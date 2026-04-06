use anyhow::Result;
use ime_core::{EngineResponse, ImeEngine, KeyEvent, SessionState};
use ime_dict::MemoryDictionary;
use ime_platform_api::{CandidatePage, InputMode, PlatformHost, PreeditState};
#[cfg(windows)]
use std::sync::{Arc, Mutex};
use tracing::info;

#[cfg(windows)]
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
    CoUninitialize,
};
#[cfg(windows)]
use windows::Win32::UI::TextServices::{
    CLSID_TF_ThreadMgr, ITfContext, ITfDocumentMgr, ITfTextInputProcessor,
    ITfTextInputProcessor_Impl, ITfThreadMgr,
};
#[cfg(windows)]
use windows_core::{implement, Interface, IUnknown, Ref as WinRef, Result as WinResult};

#[derive(Default)]
pub struct WindowsTsfHost {
    runtime: Option<TsfRuntime>,
    processor: Option<TextServiceHost>,
}

#[derive(Clone)]
pub struct DocumentManagerHandle {
    #[cfg(windows)]
    inner: ITfDocumentMgr,
}

impl DocumentManagerHandle {
    pub fn raw_identity(&self) -> usize {
        #[cfg(windows)]
        {
            self.inner.as_raw() as usize
        }

        #[cfg(not(windows))]
        {
            0
        }
    }
}

#[derive(Clone)]
pub struct ContextHandle {
    #[cfg(windows)]
    inner: ITfContext,
    edit_cookie: u32,
}

impl ContextHandle {
    pub fn raw_identity(&self) -> usize {
        #[cfg(windows)]
        {
            self.inner.as_raw() as usize
        }

        #[cfg(not(windows))]
        {
            0
        }
    }

    pub fn edit_cookie(&self) -> u32 {
        self.edit_cookie
    }
}

#[cfg(windows)]
struct ComApartment;

#[cfg(windows)]
impl ComApartment {
    fn initialize_sta() -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        }

        Ok(Self)
    }
}

#[cfg(windows)]
impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

#[cfg(windows)]
struct TsfRuntime {
    _apartment: ComApartment,
    thread_mgr: ITfThreadMgr,
    client_id: u32,
}

#[cfg(not(windows))]
struct TsfRuntime;

#[cfg(windows)]
impl TsfRuntime {
    fn initialize() -> Result<Self> {
        let apartment = ComApartment::initialize_sta()?;
        let thread_mgr: ITfThreadMgr =
            unsafe { CoCreateInstance(&CLSID_TF_ThreadMgr, None, CLSCTX_INPROC_SERVER)? };
        let client_id = unsafe { thread_mgr.Activate()? };

        Ok(Self {
            _apartment: apartment,
            thread_mgr,
            client_id,
        })
    }

    fn client_id(&self) -> u32 {
        self.client_id
    }

    fn create_document_manager(&self) -> Result<DocumentManagerHandle> {
        let document = unsafe { self.thread_mgr.CreateDocumentMgr()? };
        Ok(DocumentManagerHandle { inner: document })
    }

    fn set_focus(&self, document: &DocumentManagerHandle) -> Result<()> {
        unsafe {
            self.thread_mgr.SetFocus(&document.inner)?;
        }

        Ok(())
    }

    fn focused_document(&self) -> Result<DocumentManagerHandle> {
        let document = unsafe { self.thread_mgr.GetFocus()? };
        Ok(DocumentManagerHandle { inner: document })
    }

    fn create_context(&self, document: &DocumentManagerHandle) -> Result<ContextHandle> {
        let mut context = None;
        let mut edit_cookie = 0;

        unsafe {
            document.inner.CreateContext(
                self.client_id(),
                0,
                None::<&IUnknown>,
                &mut context,
                &mut edit_cookie,
            )?;
        }

        let context = context.expect("TSF returned no context");
        Ok(ContextHandle {
            inner: context,
            edit_cookie,
        })
    }

    fn push_context(&self, document: &DocumentManagerHandle, context: &ContextHandle) -> Result<()> {
        unsafe {
            document.inner.Push(&context.inner)?;
        }

        Ok(())
    }

    fn pop_context(&self, document: &DocumentManagerHandle) -> Result<()> {
        unsafe {
            document.inner.Pop(0)?;
        }

        Ok(())
    }

    fn top_context(&self, document: &DocumentManagerHandle) -> Result<ContextHandle> {
        let context = unsafe { document.inner.GetTop()? };
        Ok(ContextHandle {
            inner: context,
            edit_cookie: 0,
        })
    }
}

#[cfg(windows)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ProcessorLifecycleState {
    activated: bool,
    client_id: Option<u32>,
}

#[cfg(windows)]
#[implement(ITfTextInputProcessor)]
struct TextServiceProcessor {
    lifecycle: Arc<Mutex<ProcessorLifecycleState>>,
}

#[cfg(windows)]
impl TextServiceProcessor {
    fn new(lifecycle: Arc<Mutex<ProcessorLifecycleState>>) -> Self {
        Self { lifecycle }
    }
}

#[cfg(windows)]
impl ITfTextInputProcessor_Impl for TextServiceProcessor_Impl {
    fn Activate(&self, _ptim: WinRef<ITfThreadMgr>, tid: u32) -> WinResult<()> {
        let mut lifecycle = self.lifecycle.lock().expect("processor lifecycle lock");
        lifecycle.activated = true;
        lifecycle.client_id = Some(tid);
        Ok(())
    }

    fn Deactivate(&self) -> WinResult<()> {
        let mut lifecycle = self.lifecycle.lock().expect("processor lifecycle lock");
        lifecycle.activated = false;
        Ok(())
    }
}

#[cfg(windows)]
struct TextServiceHost {
    interface: ITfTextInputProcessor,
    lifecycle: Arc<Mutex<ProcessorLifecycleState>>,
}

#[cfg(windows)]
impl TextServiceHost {
    fn create(runtime: &TsfRuntime) -> Result<Self> {
        let lifecycle = Arc::new(Mutex::new(ProcessorLifecycleState::default()));
        let interface: ITfTextInputProcessor = TextServiceProcessor::new(lifecycle.clone()).into();

        unsafe {
            interface.Activate(&runtime.thread_mgr, runtime.client_id())?;
        }

        Ok(Self {
            interface,
            lifecycle,
        })
    }

    fn is_activated(&self) -> bool {
        self.lifecycle
            .lock()
            .expect("processor lifecycle lock")
            .activated
    }

    fn client_id(&self) -> Option<u32> {
        self.lifecycle
            .lock()
            .expect("processor lifecycle lock")
            .client_id
    }
}

#[cfg(windows)]
impl Drop for TextServiceHost {
    fn drop(&mut self) {
        let _ = unsafe { self.interface.Deactivate() };
    }
}

#[cfg(not(windows))]
struct TextServiceHost;

#[cfg(not(windows))]
impl TextServiceHost {
    fn create(_runtime: &TsfRuntime) -> Result<Self> {
        Ok(Self)
    }

    fn is_activated(&self) -> bool {
        false
    }

    fn client_id(&self) -> Option<u32> {
        None
    }
}

#[cfg(windows)]
impl Drop for TsfRuntime {
    fn drop(&mut self) {
        let _ = unsafe { self.thread_mgr.Deactivate() };
    }
}

#[cfg(not(windows))]
impl TsfRuntime {
    fn initialize() -> Result<Self> {
        Ok(Self)
    }
}

#[cfg(not(windows))]
impl TsfRuntime {
    fn create_document_manager(&self) -> Result<DocumentManagerHandle> {
        Ok(DocumentManagerHandle {})
    }

    fn set_focus(&self, _document: &DocumentManagerHandle) -> Result<()> {
        Ok(())
    }

    fn focused_document(&self) -> Result<DocumentManagerHandle> {
        Ok(DocumentManagerHandle {})
    }

    fn create_context(&self, _document: &DocumentManagerHandle) -> Result<ContextHandle> {
        Ok(ContextHandle { edit_cookie: 0 })
    }

    fn push_context(&self, _document: &DocumentManagerHandle, _context: &ContextHandle) -> Result<()> {
        Ok(())
    }

    fn pop_context(&self, _document: &DocumentManagerHandle) -> Result<()> {
        Ok(())
    }

    fn top_context(&self, _document: &DocumentManagerHandle) -> Result<ContextHandle> {
        Ok(ContextHandle { edit_cookie: 0 })
    }
}

impl WindowsTsfHost {
    pub fn bootstrap() -> Result<Self> {
        let runtime = TsfRuntime::initialize()?;
        let processor = TextServiceHost::create(&runtime)?;

        #[cfg(windows)]
        info!(
            client_id = runtime.client_id(),
            processor_activated = processor.is_activated(),
            "windows-tsf runtime initialized"
        );

        #[cfg(not(windows))]
        info!("windows-tsf scaffold initialized without Windows TSF runtime");

        Ok(Self {
            runtime: Some(runtime),
            processor: Some(processor),
        })
    }

    pub fn is_runtime_ready(&self) -> bool {
        self.runtime.is_some()
    }

    pub fn client_id(&self) -> Option<u32> {
        #[cfg(windows)]
        {
            self.runtime.as_ref().map(TsfRuntime::client_id)
        }

        #[cfg(not(windows))]
        {
            None
        }
    }

    pub fn processor_is_activated(&self) -> bool {
        self.processor
            .as_ref()
            .is_some_and(TextServiceHost::is_activated)
    }

    pub fn processor_client_id(&self) -> Option<u32> {
        self.processor.as_ref().and_then(TextServiceHost::client_id)
    }

    pub fn create_document_manager(&self) -> Result<DocumentManagerHandle> {
        match &self.runtime {
            Some(runtime) => runtime.create_document_manager(),
            None => anyhow::bail!("TSF runtime is not initialized"),
        }
    }

    pub fn set_focus(&self, document: &DocumentManagerHandle) -> Result<()> {
        match &self.runtime {
            Some(runtime) => runtime.set_focus(document),
            None => anyhow::bail!("TSF runtime is not initialized"),
        }
    }

    pub fn focused_document(&self) -> Result<DocumentManagerHandle> {
        match &self.runtime {
            Some(runtime) => runtime.focused_document(),
            None => anyhow::bail!("TSF runtime is not initialized"),
        }
    }

    pub fn create_context(&self, document: &DocumentManagerHandle) -> Result<ContextHandle> {
        match &self.runtime {
            Some(runtime) => runtime.create_context(document),
            None => anyhow::bail!("TSF runtime is not initialized"),
        }
    }

    pub fn push_context(
        &self,
        document: &DocumentManagerHandle,
        context: &ContextHandle,
    ) -> Result<()> {
        match &self.runtime {
            Some(runtime) => runtime.push_context(document, context),
            None => anyhow::bail!("TSF runtime is not initialized"),
        }
    }

    pub fn pop_context(&self, document: &DocumentManagerHandle) -> Result<()> {
        match &self.runtime {
            Some(runtime) => runtime.pop_context(document),
            None => anyhow::bail!("TSF runtime is not initialized"),
        }
    }

    pub fn top_context(&self, document: &DocumentManagerHandle) -> Result<ContextHandle> {
        match &self.runtime {
            Some(runtime) => runtime.top_context(document),
            None => anyhow::bail!("TSF runtime is not initialized"),
        }
    }

    pub fn process_key(&self, session: &mut SessionState, event: KeyEvent) -> EngineResponse {
        let engine = ImeEngine::new(MemoryDictionary);
        let response = engine.handle_key_event(session, event);
        self.apply_response(&response);
        response
    }

    fn apply_response(&self, response: &EngineResponse) {
        if let Some(commit) = &response.commit_text {
            self.commit_text(commit);
        } else if response.preedit.composition_text.is_empty()
            && response.candidates.items.is_empty()
        {
            self.clear_session();
        } else {
            self.update_preedit(response.preedit.clone());
            self.update_candidates(response.candidates.clone());
        }

        self.set_mode_indicator(response.input_mode.clone());
    }
}

impl PlatformHost for WindowsTsfHost {
    fn update_preedit(&self, preedit: PreeditState) {
        info!(composition = %preedit.composition_text, cursor = preedit.cursor, "update preedit");
    }

    fn update_candidates(&self, candidates: CandidatePage) {
        info!(count = candidates.items.len(), "update candidates");
    }

    fn commit_text(&self, text: &str) {
        info!(commit = %text, "commit text");
    }

    fn clear_session(&self) {
        info!("clear session");
    }

    fn set_mode_indicator(&self, mode: InputMode) {
        info!(?mode, "set input mode");
    }
}

#[cfg(test)]
mod tests {
    use super::WindowsTsfHost;
    use ime_core::{KeyEvent, SessionState};

    #[test]
    fn host_processes_basic_input() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let mut session = SessionState::default();

        assert!(host.is_runtime_ready());
        #[cfg(windows)]
        {
            assert!(host.processor_is_activated());
            assert_eq!(host.processor_client_id(), host.client_id());
        }

        let response = host.process_key(&mut session, KeyEvent::Char('s'));
        assert!(response.consumed);
        assert_eq!(response.preedit.composition_text, "s");
    }

    #[test]
    fn host_creates_document_manager() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");

        assert_ne!(document.raw_identity(), 0);
    }

    #[cfg(windows)]
    #[test]
    fn host_can_set_and_read_focus_document() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");

        host.set_focus(&document).expect("focus set");
        let focused = host.focused_document().expect("focused document");

        assert_eq!(focused.raw_identity(), document.raw_identity());
    }

    #[test]
    fn host_creates_context() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let _context = host.create_context(&document).expect("context created");
    }

    #[cfg(windows)]
    #[test]
    fn host_can_push_and_read_top_context() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");

        host.push_context(&document, &context).expect("context pushed");
        let _top = host.top_context(&document).expect("top context");
    }
}

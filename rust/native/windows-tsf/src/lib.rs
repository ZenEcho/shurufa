mod composition;
mod context_bridge;
mod key_event;
mod text_executor;
mod registration;
mod response;

use anyhow::Result;
use composition::{CompositionAction, CompositionState};
use context_bridge::{TextExecutionPlan, TextWriteRequest, TsfTextContextBridge};
use ime_core::{EngineResponse, ImeEngine, KeyEvent, SessionState};
use ime_dict::MemoryDictionary;
use ime_platform_api::{CandidatePage, InputMode, PlatformHost, PreeditState};
use key_event::translate_wparam;
pub use registration::{
    SHURUFA_PROFILE_GUID, SHURUFA_TSF_CLSID, TsfRegistrationManifest, default_dll_file_name,
    map_self_registration_result, register_server, should_serve_class_object, unregister_server,
};
use response::{HostAction, HostActionPlan};
use text_executor::{TextExecutionReport, execute_plan};
#[cfg(windows)]
use std::ffi::c_void;
#[cfg(not(windows))]
use std::sync::Mutex;
#[cfg(windows)]
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

#[cfg(windows)]
use windows::Win32::Foundation::{
    CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_FAIL, E_POINTER, S_FALSE,
};
#[cfg(windows)]
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
    CoUninitialize, IClassFactory, IClassFactory_Impl,
};
#[cfg(windows)]
use windows::Win32::UI::TextServices::{
    CLSID_TF_ThreadMgr, ITfContext, ITfDocumentMgr, ITfKeyEventSink, ITfKeyEventSink_Impl,
    ITfKeystrokeMgr, ITfTextInputProcessor, ITfTextInputProcessor_Impl, ITfThreadMgr,
};
#[cfg(windows)]
use windows_core::{BOOL, GUID, HRESULT};
#[cfg(windows)]
use windows_core::{Error as WinError, IUnknown, Interface, Ref as WinRef, Result as WinResult, implement};

pub struct WindowsTsfHost {
    runtime: Option<TsfRuntime>,
    processor: Option<TextServiceHost>,
    composition_state: Mutex<CompositionState>,
    text_context_bridge: Mutex<TsfTextContextBridge>,
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
    pub(crate) inner: ITfContext,
    edit_cookie: u32,
}

impl ContextHandle {
    #[cfg(windows)]
    fn from_inner(inner: ITfContext, edit_cookie: u32) -> Self {
        Self { inner, edit_cookie }
    }

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
        Ok(ContextHandle::from_inner(context, edit_cookie))
    }

    fn push_context(
        &self,
        document: &DocumentManagerHandle,
        context: &ContextHandle,
    ) -> Result<()> {
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
        Ok(ContextHandle::from_inner(context, 0))
    }
}

#[cfg(windows)]
#[derive(Debug)]
struct ProcessorLifecycleState {
    activated: bool,
    client_id: Option<u32>,
    thread_mgr: Option<ITfThreadMgr>,
    key_sink_advised: bool,
    session: SessionState,
    composition_state: CompositionState,
    text_context_bridge: TsfTextContextBridge,
    last_execution_report: Option<TextExecutionReport>,
}

#[cfg(windows)]
impl Default for ProcessorLifecycleState {
    fn default() -> Self {
        Self {
            activated: false,
            client_id: None,
            thread_mgr: None,
            key_sink_advised: false,
            session: SessionState::default(),
            composition_state: CompositionState::default(),
            text_context_bridge: TsfTextContextBridge::default(),
            last_execution_report: None,
        }
    }
}

#[cfg(windows)]
#[implement(ITfTextInputProcessor)]
struct TextServiceProcessor {
    lifecycle: Arc<Mutex<ProcessorLifecycleState>>,
    key_event_sink: ITfKeyEventSink,
}

#[cfg(windows)]
#[implement(ITfKeyEventSink)]
struct TextServiceKeyEventSink {
    lifecycle: Arc<Mutex<ProcessorLifecycleState>>,
}

#[cfg(windows)]
impl TextServiceProcessor {
    fn new(lifecycle: Arc<Mutex<ProcessorLifecycleState>>) -> Self {
        let key_event_sink: ITfKeyEventSink =
            TextServiceKeyEventSink::new(lifecycle.clone()).into();
        Self {
            lifecycle,
            key_event_sink,
        }
    }
}

#[cfg(windows)]
impl TextServiceKeyEventSink {
    fn new(lifecycle: Arc<Mutex<ProcessorLifecycleState>>) -> Self {
        Self { lifecycle }
    }
}

#[cfg(windows)]
fn create_text_service_processor() -> ITfTextInputProcessor {
    TextServiceProcessor::new(Arc::new(Mutex::new(ProcessorLifecycleState::default()))).into()
}

#[cfg(windows)]
fn apply_response_to_processor_state(
    lifecycle: &mut ProcessorLifecycleState,
    response: &EngineResponse,
) {
    for action in HostActionPlan::from_response(response).actions {
        for composition_action in lifecycle.composition_state.apply_host_action(&action) {
            lifecycle
                .text_context_bridge
                .apply_composition_action(composition_action);
        }
    }
}

#[cfg(windows)]
fn process_key_with_bound_context(
    lifecycle: &Arc<Mutex<ProcessorLifecycleState>>,
    context: &ITfContext,
    event: KeyEvent,
) -> WinResult<BOOL> {
    let mut lifecycle = lifecycle.lock().expect("processor lifecycle lock");
    let client_id = lifecycle.client_id.ok_or_else(|| WinError::from(E_POINTER))?;
    let context_handle = ContextHandle::from_inner(context.clone(), 0);

    lifecycle
        .text_context_bridge
        .bind_context(&context_handle)
        .map_err(|_| WinError::from(E_FAIL))?;

    let engine = ImeEngine::new(MemoryDictionary);
    let response = engine.handle_key_event(&mut lifecycle.session, event);
    apply_response_to_processor_state(&mut lifecycle, &response);

    if let Some(plan) = lifecycle.text_context_bridge.build_execution_plan() {
        match execute_plan(&context_handle, client_id, &plan) {
            Ok(report) => {
                info!(
                    committed = report.committed_count(),
                    skipped = report.skipped_steps.len(),
                    "executed text plan for TSF context"
                );
                lifecycle.last_execution_report = Some(report);
                lifecycle.text_context_bridge.clear_requests();
            }
            Err(_) => {
                lifecycle.last_execution_report = Some(TextExecutionReport {
                    context_identity: plan.context_identity,
                    committed_texts: Vec::new(),
                    skipped_steps: plan.steps,
                });
            }
        }
    }

    Ok(response.consumed.into())
}

#[cfg(windows)]
impl ITfTextInputProcessor_Impl for TextServiceProcessor_Impl {
    fn Activate(&self, ptim: WinRef<ITfThreadMgr>, tid: u32) -> WinResult<()> {
        let thread_mgr = ptim.clone().ok_or_else(|| WinError::from(E_POINTER))?;
        let key_sink_advised = if let Ok(keystroke_mgr) = thread_mgr.cast::<ITfKeystrokeMgr>() {
            match unsafe { keystroke_mgr.AdviseKeyEventSink(tid, &self.key_event_sink, true) } {
                Ok(()) => true,
                Err(error) => {
                    warn!(client_id = tid, ?error, "failed to advise key event sink during activate");
                    false
                }
            }
        } else {
            false
        };

        let mut lifecycle = self.lifecycle.lock().expect("processor lifecycle lock");
        lifecycle.activated = true;
        lifecycle.client_id = Some(tid);
        lifecycle.thread_mgr = Some(thread_mgr.clone());
        lifecycle.key_sink_advised = key_sink_advised;
        Ok(())
    }

    fn Deactivate(&self) -> WinResult<()> {
        let (thread_mgr, client_id, key_sink_advised) = {
            let lifecycle = self.lifecycle.lock().expect("processor lifecycle lock");
            (
                lifecycle.thread_mgr.clone(),
                lifecycle.client_id,
                lifecycle.key_sink_advised,
            )
        };

        if key_sink_advised {
            if let (Some(thread_mgr), Some(client_id)) = (thread_mgr, client_id) {
                if let Ok(keystroke_mgr) = thread_mgr.cast::<ITfKeystrokeMgr>() {
                    let _ = unsafe { keystroke_mgr.UnadviseKeyEventSink(client_id) };
                }
            }
        }

        let mut lifecycle = self.lifecycle.lock().expect("processor lifecycle lock");
        lifecycle.activated = false;
        lifecycle.client_id = None;
        lifecycle.thread_mgr = None;
        lifecycle.key_sink_advised = false;
        lifecycle.session = SessionState::default();
        lifecycle.composition_state = CompositionState::default();
        lifecycle.text_context_bridge.clear_binding();
        lifecycle.text_context_bridge.clear_requests();
        lifecycle.last_execution_report = None;
        Ok(())
    }
}

#[cfg(windows)]
impl ITfKeyEventSink_Impl for TextServiceKeyEventSink_Impl {
    fn OnSetFocus(&self, _fforeground: BOOL) -> WinResult<()> {
        Ok(())
    }

    fn OnTestKeyDown(
        &self,
        _pic: WinRef<ITfContext>,
        wparam: windows::Win32::Foundation::WPARAM,
        _lparam: windows::Win32::Foundation::LPARAM,
    ) -> WinResult<BOOL> {
        Ok(translate_wparam(wparam).is_some().into())
    }

    fn OnTestKeyUp(
        &self,
        _pic: WinRef<ITfContext>,
        _wparam: windows::Win32::Foundation::WPARAM,
        _lparam: windows::Win32::Foundation::LPARAM,
    ) -> WinResult<BOOL> {
        Ok(false.into())
    }

    fn OnKeyDown(
        &self,
        pic: WinRef<ITfContext>,
        wparam: windows::Win32::Foundation::WPARAM,
        _lparam: windows::Win32::Foundation::LPARAM,
    ) -> WinResult<BOOL> {
        let Some(event) = translate_wparam(wparam) else {
            return Ok(false.into());
        };
        let context = pic.clone().ok_or_else(|| WinError::from(E_POINTER))?;
        process_key_with_bound_context(&self.lifecycle, &context, event)
    }

    fn OnKeyUp(
        &self,
        _pic: WinRef<ITfContext>,
        _wparam: windows::Win32::Foundation::WPARAM,
        _lparam: windows::Win32::Foundation::LPARAM,
    ) -> WinResult<BOOL> {
        Ok(false.into())
    }

    fn OnPreservedKey(
        &self,
        _pic: WinRef<ITfContext>,
        _rguid: *const GUID,
    ) -> WinResult<BOOL> {
        Ok(false.into())
    }
}

#[cfg(windows)]
#[implement(IClassFactory)]
struct TextServiceClassFactory;

#[cfg(windows)]
impl IClassFactory_Impl for TextServiceClassFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: WinRef<IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut c_void,
    ) -> WinResult<()> {
        unsafe {
            if ppvobject.is_null() {
                return Err(E_POINTER.into());
            }

            *ppvobject = std::ptr::null_mut();

            if punkouter.is_some() {
                return Err(CLASS_E_NOAGGREGATION.into());
            }

            if riid.is_null() {
                return Err(E_POINTER.into());
            }

            let processor = create_text_service_processor();
            processor.query(riid, ppvobject).ok()?;
            Ok(())
        }
    }

    fn LockServer(&self, _flock: BOOL) -> WinResult<()> {
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

    fn push_context(
        &self,
        _document: &DocumentManagerHandle,
        _context: &ContextHandle,
    ) -> Result<()> {
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
    pub fn registration_manifest() -> TsfRegistrationManifest {
        TsfRegistrationManifest::shurufa_default()
    }

    pub fn register_profiles() -> Result<()> {
        Self::registration_manifest().register_profiles()
    }

    pub fn unregister_profiles() -> Result<()> {
        Self::registration_manifest().unregister_profiles()
    }

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
            composition_state: Mutex::new(CompositionState::default()),
            text_context_bridge: Mutex::new(TsfTextContextBridge::default()),
        })
    }

    pub fn active_composition_text(&self) -> Option<String> {
        self.composition_state
            .lock()
            .expect("composition state lock")
            .active_text
            .clone()
    }

    pub fn bind_text_context(&self, context: &ContextHandle) -> Result<()> {
        self.text_context_bridge
            .lock()
            .expect("text context bridge lock")
            .bind_context(context)
    }

    pub fn clear_text_context_binding(&self) {
        self.text_context_bridge
            .lock()
            .expect("text context bridge lock")
            .clear_binding();
    }

    pub fn bound_text_context_identity(&self) -> Option<usize> {
        self.text_context_bridge
            .lock()
            .expect("text context bridge lock")
            .bound_context()
            .map(|context| context.identity)
    }

    pub fn pending_write_requests(&self) -> Vec<TextWriteRequest> {
        self.text_context_bridge
            .lock()
            .expect("text context bridge lock")
            .write_requests()
            .to_vec()
    }

    pub fn pending_execution_plan(&self) -> Option<TextExecutionPlan> {
        self.text_context_bridge
            .lock()
            .expect("text context bridge lock")
            .build_execution_plan()
    }

    pub fn clear_pending_write_requests(&self) {
        self.text_context_bridge
            .lock()
            .expect("text context bridge lock")
            .clear_requests();
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
        for action in HostActionPlan::from_response(response).actions {
            self.apply_composition_action(&action);

            match action {
                HostAction::CommitText(commit) => self.commit_text(&commit),
                HostAction::ClearSession => self.clear_session(),
                HostAction::UpdatePreedit(preedit) => self.update_preedit(preedit),
                HostAction::UpdateCandidates(candidates) => self.update_candidates(candidates),
                HostAction::SetModeIndicator(mode) => self.set_mode_indicator(mode),
            }
        }
    }

    fn apply_composition_action(&self, action: &HostAction) {
        let mut composition_state = self
            .composition_state
            .lock()
            .expect("composition state lock");

        for composition_action in composition_state.apply_host_action(action) {
            self.handle_composition_action(composition_action);
        }
    }

    fn handle_composition_action(&self, action: CompositionAction) {
        self.text_context_bridge
            .lock()
            .expect("text context bridge lock")
            .apply_composition_action(action.clone());

        match action {
            CompositionAction::Start { text, cursor } => {
                info!(composition = %text, cursor, "start composition");
            }
            CompositionAction::Update { text, cursor } => {
                info!(composition = %text, cursor, "update composition");
            }
            CompositionAction::Commit(text) => {
                info!(commit = %text, "commit composition text");
            }
            CompositionAction::End => {
                info!("end composition");
            }
        }
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

#[cfg(windows)]
#[unsafe(no_mangle)]
pub extern "system" fn DllRegisterServer() -> HRESULT {
    map_self_registration_result(register_server())
}

#[cfg(windows)]
#[unsafe(no_mangle)]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
    map_self_registration_result(unregister_server())
}

#[cfg(windows)]
#[unsafe(no_mangle)]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_FALSE
}

#[cfg(windows)]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() || riid.is_null() || rclsid.is_null() {
        return E_POINTER;
    }

    unsafe {
        *ppv = std::ptr::null_mut();
    }

    let requested_clsid = unsafe { &*rclsid };
    if !should_serve_class_object(requested_clsid) {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    let factory: IClassFactory = TextServiceClassFactory.into();
    unsafe { factory.query(riid, ppv) }
}

#[cfg(test)]
mod tests {
    use super::{DllGetClassObject, WindowsTsfHost};
    use crate::context_bridge::{TextExecutionStep, TextWriteRequest};
    use ime_core::{KeyEvent, SessionState};
    #[cfg(windows)]
    use windows::Win32::Foundation::CLASS_E_CLASSNOTAVAILABLE;
    #[cfg(windows)]
    use windows::Win32::System::Com::IClassFactory;
    #[cfg(windows)]
    use windows_core::{GUID, Interface};

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
        assert_eq!(host.active_composition_text().as_deref(), Some("s"));
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

    #[test]
    fn host_can_bind_text_context() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");

        host.bind_text_context(&context).expect("bind text context");

        assert_eq!(
            host.bound_text_context_identity(),
            Some(context.raw_identity())
        );
    }

    #[test]
    fn host_records_write_requests_for_bound_preedit_updates() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");
        let mut session = SessionState::default();

        host.bind_text_context(&context).expect("bind text context");
        let _response = host.process_key(&mut session, KeyEvent::Char('s'));

        assert_eq!(
            host.pending_write_requests(),
            vec![TextWriteRequest::StartComposition {
                text: "s".to_string(),
                cursor: 1,
            }]
        );
    }

    #[test]
    fn host_records_end_and_commit_requests_when_selection_commits() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");
        let mut session = SessionState::default();

        host.bind_text_context(&context).expect("bind text context");
        let _response = host.process_key(&mut session, KeyEvent::Char('n'));
        let _response = host.process_key(&mut session, KeyEvent::Char('i'));
        host.clear_pending_write_requests();
        let _response = host.process_key(&mut session, KeyEvent::Number(1));

        assert_eq!(
            host.pending_write_requests(),
            vec![
                TextWriteRequest::EndComposition,
                TextWriteRequest::CommitText("你".to_string()),
            ]
        );
    }

    #[test]
    fn host_exposes_pending_execution_plan_for_bound_context() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");
        let mut session = SessionState::default();

        host.bind_text_context(&context).expect("bind text context");
        let _response = host.process_key(&mut session, KeyEvent::Char('s'));

        let plan = host.pending_execution_plan().expect("execution plan");
        assert_eq!(plan.context_identity, context.raw_identity());
        assert_eq!(
            plan.steps,
            vec![TextExecutionStep::StartComposition {
                text: "s".to_string(),
                cursor: 1,
            }]
        );
    }

    #[cfg(windows)]
    #[test]
    fn host_can_push_and_read_top_context() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");

        host.push_context(&document, &context)
            .expect("context pushed");
        let _top = host.top_context(&document).expect("top context");
    }

    #[cfg(windows)]
    #[test]
    fn dll_get_class_object_serves_registered_clsid() {
        let mut result = std::ptr::null_mut();
        let hr = unsafe {
            DllGetClassObject(&crate::SHURUFA_TSF_CLSID, &IClassFactory::IID, &mut result)
        };

        assert_eq!(hr.0, 0);
        assert!(!result.is_null());
    }

    #[cfg(windows)]
    #[test]
    fn dll_get_class_object_rejects_unknown_clsid() {
        let mut result = std::ptr::null_mut();
        let hr = unsafe { DllGetClassObject(&GUID::zeroed(), &IClassFactory::IID, &mut result) };

        assert_eq!(hr, CLASS_E_CLASSNOTAVAILABLE);
        assert!(result.is_null());
    }
}

use anyhow::Result;
#[cfg(windows)]
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
    CoUninitialize,
};
#[cfg(windows)]
use windows::Win32::UI::TextServices::{
    CLSID_TF_CategoryMgr, CLSID_TF_InputProcessorProfiles, GUID_TFCAT_TIP_KEYBOARD, ITfCategoryMgr,
    ITfInputProcessorProfiles,
};
use windows_core::{GUID, HRESULT};

pub const SHURUFA_TSF_CLSID: GUID = GUID::from_u128(0x9f1e7af0_7981_4c9c_9a6d_26d1b2d11810);
pub const SHURUFA_PROFILE_GUID: GUID = GUID::from_u128(0x4bd8d561_b3f4_4932_a9f5_7c3d7c1cf4de);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsfRegistrationManifest {
    pub service_name: &'static str,
    pub display_name: &'static str,
    pub clsid: GUID,
    pub profile_guid: GUID,
    pub langid: u16,
    pub icon_path: &'static str,
    pub icon_index: u32,
}

impl TsfRegistrationManifest {
    pub fn shurufa_default() -> Self {
        Self {
            service_name: "ShurufaTSF",
            display_name: "书入法输入法",
            clsid: SHURUFA_TSF_CLSID,
            profile_guid: SHURUFA_PROFILE_GUID,
            langid: 0x0804,
            icon_path: r"%ProgramFiles%\Shurufa\shurufa-ime.dll",
            icon_index: 0,
        }
    }

    pub fn clsid_string(&self) -> String {
        guid_to_registry_string(&self.clsid)
    }

    pub fn profile_guid_string(&self) -> String {
        guid_to_registry_string(&self.profile_guid)
    }

    pub fn clsid_registry_path(&self) -> String {
        format!(r"Software\Classes\CLSID\{}", self.clsid_string())
    }

    pub fn language_profile_registry_path(&self) -> String {
        format!(
            r"Software\Microsoft\CTF\TIP\{}\LanguageProfile\0x{:04x}\{}",
            self.clsid_string(),
            self.langid,
            self.profile_guid_string()
        )
    }

    pub fn tip_keyboard_category_string(&self) -> String {
        guid_to_registry_string(&GUID_TFCAT_TIP_KEYBOARD)
    }

    #[cfg(windows)]
    pub fn register_profiles(&self) -> Result<()> {
        let _apartment = RegistrationApartment::initialize_sta()?;
        let description = wide_null(self.display_name);
        let icon_path = wide_null(self.icon_path);

        unsafe {
            let profiles: ITfInputProcessorProfiles =
                CoCreateInstance(&CLSID_TF_InputProcessorProfiles, None, CLSCTX_INPROC_SERVER)?;
            profiles.Register(&self.clsid)?;
            profiles.AddLanguageProfile(
                &self.clsid,
                self.langid,
                &self.profile_guid,
                &description,
                &icon_path,
                self.icon_index,
            )?;
            profiles.EnableLanguageProfile(&self.clsid, self.langid, &self.profile_guid, true)?;
            profiles.EnableLanguageProfileByDefault(
                &self.clsid,
                self.langid,
                &self.profile_guid,
                true,
            )?;
            let _ = profiles.ActivateLanguageProfile(&self.clsid, self.langid, &self.profile_guid);

            let category_mgr: ITfCategoryMgr =
                CoCreateInstance(&CLSID_TF_CategoryMgr, None, CLSCTX_INPROC_SERVER)?;
            category_mgr.RegisterCategory(&self.clsid, &GUID_TFCAT_TIP_KEYBOARD, &self.clsid)?;
        }

        Ok(())
    }

    #[cfg(windows)]
    pub fn unregister_profiles(&self) -> Result<()> {
        let _apartment = RegistrationApartment::initialize_sta()?;

        unsafe {
            let profiles: ITfInputProcessorProfiles =
                CoCreateInstance(&CLSID_TF_InputProcessorProfiles, None, CLSCTX_INPROC_SERVER)?;
            let _ = profiles.RemoveLanguageProfile(&self.clsid, self.langid, &self.profile_guid);
            let _ = profiles.Unregister(&self.clsid);

            let category_mgr: ITfCategoryMgr =
                CoCreateInstance(&CLSID_TF_CategoryMgr, None, CLSCTX_INPROC_SERVER)?;
            let _ =
                category_mgr.UnregisterCategory(&self.clsid, &GUID_TFCAT_TIP_KEYBOARD, &self.clsid);
        }

        Ok(())
    }

    #[cfg(not(windows))]
    pub fn register_profiles(&self) -> Result<()> {
        let _ = self;
        Ok(())
    }

    #[cfg(not(windows))]
    pub fn unregister_profiles(&self) -> Result<()> {
        let _ = self;
        Ok(())
    }
}

pub fn default_dll_file_name() -> &'static str {
    "windows_tsf.dll"
}

pub fn should_serve_class_object(clsid: &GUID) -> bool {
    clsid == &SHURUFA_TSF_CLSID
}

pub fn register_server() -> Result<()> {
    TsfRegistrationManifest::shurufa_default().register_profiles()
}

pub fn unregister_server() -> Result<()> {
    TsfRegistrationManifest::shurufa_default().unregister_profiles()
}

pub fn map_self_registration_result(result: Result<()>) -> HRESULT {
    match result {
        Ok(()) => HRESULT(0),
        Err(_) => HRESULT(0x80004005u32 as i32),
    }
}

fn guid_to_registry_string(guid: &GUID) -> String {
    format!(
        "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
        guid.data1,
        guid.data2,
        guid.data3,
        guid.data4[0],
        guid.data4[1],
        guid.data4[2],
        guid.data4[3],
        guid.data4[4],
        guid.data4[5],
        guid.data4[6],
        guid.data4[7]
    )
}

#[cfg(windows)]
struct RegistrationApartment;

#[cfg(windows)]
impl RegistrationApartment {
    fn initialize_sta() -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        }

        Ok(Self)
    }
}

#[cfg(windows)]
impl Drop for RegistrationApartment {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

#[cfg(windows)]
fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        SHURUFA_TSF_CLSID, TsfRegistrationManifest, default_dll_file_name,
        map_self_registration_result, should_serve_class_object,
    };
    use anyhow::anyhow;
    use windows_core::GUID;

    #[test]
    fn default_manifest_exposes_expected_registry_paths() {
        let manifest = TsfRegistrationManifest::shurufa_default();

        assert!(manifest.clsid_registry_path().contains("CLSID"));
        assert!(
            manifest
                .language_profile_registry_path()
                .contains("LanguageProfile")
        );
        assert!(
            manifest
                .language_profile_registry_path()
                .contains(&manifest.profile_guid_string())
        );
    }

    #[test]
    fn default_manifest_targets_simplified_chinese_profile() {
        let manifest = TsfRegistrationManifest::shurufa_default();

        assert_eq!(manifest.langid, 0x0804);
        assert_eq!(manifest.display_name, "书入法输入法");
    }

    #[test]
    fn self_registration_result_maps_success_and_failure() {
        assert_eq!(map_self_registration_result(Ok(())).0, 0);
        assert_ne!(
            map_self_registration_result(Err(anyhow!("register failed"))).0,
            0
        );
    }

    #[test]
    fn default_dll_file_name_matches_windows_cdylib_output() {
        assert_eq!(default_dll_file_name(), "windows_tsf.dll");
    }

    #[test]
    fn only_registered_clsid_is_served_by_class_factory() {
        assert!(should_serve_class_object(&SHURUFA_TSF_CLSID));
        assert!(!should_serve_class_object(&GUID::zeroed()));
    }
}

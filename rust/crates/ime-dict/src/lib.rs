use ime_platform_api::Candidate;

pub trait DictionaryProvider {
    fn search(&self, code: &str) -> Vec<Candidate>;
}

#[derive(Default)]
pub struct MemoryDictionary;

impl DictionaryProvider for MemoryDictionary {
    fn search(&self, code: &str) -> Vec<Candidate> {
        if code.is_empty() {
            return Vec::new();
        }

        vec![
            Candidate {
                id: format!("{code}-1"),
                text: "shurufa".to_string(),
                annotation: Some("system-dict".to_string()),
                hotkey: Some("1".to_string()),
            },
            Candidate {
                id: format!("{code}-2"),
                text: "settings-center".to_string(),
                annotation: Some("sample-candidate".to_string()),
                hotkey: Some("2".to_string()),
            },
        ]
    }
}

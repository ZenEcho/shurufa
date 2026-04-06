use ime_platform_api::Candidate;

pub trait DictionaryProvider {
    fn search(&self, code: &str) -> Vec<Candidate>;
}

#[derive(Default)]
pub struct MemoryDictionary;

const BUILTIN_PINYIN: &[(&str, &[&str])] = &[
    ("ni", &["你"]),
    ("hao", &["好"]),
    ("nihao", &["你好"]),
    ("zhong", &["中"]),
    ("guo", &["国"]),
    ("zhongguo", &["中国"]),
    ("wo", &["我"]),
    ("women", &["我们"]),
];

impl DictionaryProvider for MemoryDictionary {
    fn search(&self, code: &str) -> Vec<Candidate> {
        if code.is_empty() {
            return Vec::new();
        }

        BUILTIN_PINYIN
            .iter()
            .find(|(pinyin, _)| *pinyin == code)
            .map(|(pinyin, words)| {
                words
                    .iter()
                    .enumerate()
                    .map(|(index, word)| Candidate {
                        id: format!("{pinyin}-{}", index + 1),
                        text: (*word).to_string(),
                        annotation: Some("builtin-pinyin".to_string()),
                        hotkey: Some((index + 1).to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::{DictionaryProvider, MemoryDictionary};

    #[test]
    fn exact_pinyin_codes_return_expected_chinese_candidates() {
        let dictionary = MemoryDictionary;

        let nihao = dictionary.search("nihao");
        let zhongguo = dictionary.search("zhongguo");

        assert_eq!(nihao.first().map(|item| item.text.as_str()), Some("你好"));
        assert_eq!(zhongguo.first().map(|item| item.text.as_str()), Some("中国"));
    }

    #[test]
    fn unknown_code_returns_no_candidates() {
        let dictionary = MemoryDictionary;

        assert!(dictionary.search("xyz").is_empty());
    }
}

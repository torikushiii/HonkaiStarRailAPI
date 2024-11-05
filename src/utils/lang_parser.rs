pub const SUPPORTED_LANGUAGES: &[&str] = &[
    "en-us", "zh-cn", "zh-tw", "de-de", "es-es", "fr-fr", "id-id",
    "it-it", "ja-jp", "ko-kr", "pt-pt", "ru-ru", "th-th", "tr-tr", "vi-vn"
];

pub fn parse_language_code(lang_code: &str) -> &'static str {
    match lang_code.trim().to_lowercase().as_str() {
        "en" => "en-us",
        "cn" => "zh-cn",
        "tw" => "zh-tw",
        "de" => "de-de",
        "es" => "es-es",
        "fr" => "fr-fr",
        "id" => "id-id",
        "it" => "it-it",
        "ja" | "jp" => "ja-jp",
        "ko" | "kr" => "ko-kr",
        "pt" => "pt-pt",
        "ru" => "ru-ru",
        "th" => "th-th",
        "tr" => "tr-tr",
        "vi" | "vn" => "vi-vn",
        _ => "en-us",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_parsing() {
        assert_eq!(parse_language_code("en"), "en-us");
        assert_eq!(parse_language_code("CN"), "zh-cn");
        assert_eq!(parse_language_code("jp"), "ja-jp");
        assert_eq!(parse_language_code("ja"), "ja-jp");
        assert_eq!(parse_language_code("kr"), "ko-kr");
        assert_eq!(parse_language_code("ko"), "ko-kr");
        assert_eq!(parse_language_code("vn"), "vi-vn");
        assert_eq!(parse_language_code("vi"), "vi-vn");
        assert_eq!(parse_language_code("invalid"), "en-us");
        assert_eq!(parse_language_code(""), "en-us");
        assert_eq!(parse_language_code(" en "), "en-us"); // tests trim
    }
} 
#[cfg(test)]
mod tests {
    use crate::resolvers::prydwen::PrydwenResolver;
    use crate::resolvers::CodeResolver;
    use tokio;

    #[tokio::test]
    async fn test_fetch_prydwen_codes() {
        let resolver = PrydwenResolver::new();
        let codes = resolver.fetch_codes().await.unwrap();
        
        println!("Found codes:");
        for code in &codes {
            println!("Code: {}", code.code);
            println!("Rewards: {:?}", code.rewards);
        }
        
        assert!(!codes.is_empty(), "Should find at least one code");
    }

    #[tokio::test]
    async fn test_parse_html_from_live_site() {
        let resolver = PrydwenResolver::new();
        
        let response = resolver.client
            .get(resolver.base_url())
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success(), "Failed to fetch page");
        
        let html = response.text().await.unwrap();
        let codes = resolver.parse_html(&html);
        
        // Only print during tests
        if std::env::var("RUST_TEST_NOCAPTURE").is_ok() {
            println!("\nParsed codes from live site:");
            for code in &codes {
                println!("Code: {}", code.code);
                println!("Rewards: {:?}", code.rewards);
            }
        }
        
        assert!(!codes.is_empty(), "Should find at least one code");
        
        for code in &codes {
            assert!(code.code.len() >= 8, "Code should be at least 8 characters long");
            assert!(code.code.chars().all(|c| c.is_ascii_alphanumeric()), "Code should only contain alphanumeric characters");
        }
    }
} 
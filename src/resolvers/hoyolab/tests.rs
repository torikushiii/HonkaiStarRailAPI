#[cfg(test)]
mod tests {
    use crate::resolvers::hoyolab::HoyolabResolver;
    use crate::resolvers::CodeResolver;
    use tokio;

    #[tokio::test]
    async fn test_fetch_hoyolab_codes() {
        let resolver = HoyolabResolver::new();
        let codes = resolver.fetch_codes().await.unwrap();
        
        if !codes.is_empty() {
            println!("Found codes:");
            for code in &codes {
                println!("Code: {}", code.code);
                println!("Rewards: {:?}", code.rewards);
            }
        } else {
            println!("No active codes found (this is expected when there's no ongoing event)");
        }
        
        // Don't assert codes.is_empty() since there might or might not be codes
        // Instead, verify that the function executed successfully
        for code in &codes {
            assert!(code.code.len() >= 8, "Code should be at least 8 characters long");
            assert!(code.code.chars().all(|c| c.is_ascii_alphanumeric()), "Code should only contain alphanumeric characters");
            assert!(!code.rewards.is_empty(), "Each code should have at least one reward");
        }
    }

    #[tokio::test]
    async fn test_parse_empty_response() {
        let empty_response = r#"{
            "retcode": 0,
            "message": "OK",
            "data": null
        }"#;

        let response: serde_json::Value = serde_json::from_str(empty_response).unwrap();
        
        // Verify that we can deserialize an empty response
        let parsed: crate::resolvers::hoyolab::HoyolabResponse = 
            serde_json::from_value(response).unwrap();
        assert!(parsed.data.is_none());
        assert_eq!(parsed.retcode, 0);
        assert_eq!(parsed.message, "OK");
    }

    #[tokio::test]
    async fn test_parse_error_response() {
        let error_response = r#"{
            "retcode": -1,
            "message": "Error message",
            "data": null
        }"#;

        let response: serde_json::Value = serde_json::from_str(error_response).unwrap();
        
        let parsed: crate::resolvers::hoyolab::HoyolabResponse = 
            serde_json::from_value(response).unwrap();
        assert!(parsed.data.is_none());
        assert_eq!(parsed.retcode, -1);
        assert_eq!(parsed.message, "Error message");
    }
} 
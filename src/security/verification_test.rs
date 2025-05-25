#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::verification::{VerificationError};

    #[test]
    fn test_verification_error() {
        let verification_failed = VerificationError::VerificationFailed("Signature mismatch".to_string());
        let signature_error = VerificationError::SignatureError("Invalid signature format".to_string());
        let hash_error = VerificationError::HashError("Hash mismatch".to_string());
        let other_error = VerificationError::Other("Unknown error".to_string());
        
        assert_eq!(
            format!("{}", verification_failed),
            "Verification failed: Signature mismatch"
        );
        assert_eq!(
            format!("{}", signature_error),
            "Signature error: Invalid signature format"
        );
        assert_eq!(
            format!("{}", hash_error),
            "Hash error: Hash mismatch"
        );
        assert_eq!(
            format!("{}", other_error),
            "Other error: Unknown error"
        );
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}

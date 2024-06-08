pub mod ksuid_rs {
    use pgrx::prelude::*;
    use svix_ksuid::{KsuidLike, KsuidMs};

    #[pg_extern]
    pub(crate) fn generate_ksuid() -> String {
        let ksuid_string = KsuidMs::new(None, None);
        ksuid_string.to_string()
    }

    #[pg_extern]
    pub(crate) fn generate_ksuid_bytes() -> Vec<u8> {
        let ksuid_bytes = KsuidMs::new(None, None);
        ksuid_bytes.bytes().to_vec()
    }

    // tests
    #[cfg(any(test, feature = "pg_test"))]
    #[pgrx::pg_schema]
    mod tests {
        use pgrx::pg_test;

        // Test Ksuid length
        #[pg_test]
        fn test_generate_ksuid_length() {
            let ksuid_string: String = crate::ksuid::ksuid_rs::generate_ksuid();
            assert_eq!(ksuid_string.len(), 27);
        }

        // Test ksuid bytes
        #[pg_test]
        fn test_generate_ksuid_bytes() {
            let ksuid_bytes: Vec<u8> = crate::ksuid::ksuid_rs::generate_ksuid_bytes();
            assert_eq!(ksuid_bytes.len(), 20);
        }
    }
}

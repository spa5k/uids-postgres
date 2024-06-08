pub mod uuidv7_rs {
    use pgrx::prelude::*;
    use uuid::Uuid;

    #[pg_extern]
    pub(crate) fn generate_uuidv7() -> String {
        let uuid = Uuid::now_v7();
        uuid.to_string()
    }

    #[pg_extern]
    pub(crate) fn generate_uuidv7_bytes() -> Vec<u8> {
        let uuid = Uuid::now_v7();
        uuid.as_bytes().to_vec()
    }

    #[pg_extern]
    pub fn generate_uuidv7_from_string(from_str: String) -> String {
        let result = Uuid::parse_str(&from_str);
        result.unwrap().to_string()
    }

    #[pg_extern]
    pub fn parse_uuidv7(from_str: String) -> String {
        let result = Uuid::parse_str(&from_str);
        result.unwrap().to_string()
    }

    // tests
    #[cfg(any(test, feature = "pg_test"))]
    #[pgrx::pg_schema]
    mod tests {
        use pgrx::pg_test;

        // Test UUIDv7 length
        #[pg_test]
        fn test_generate_uuidv7_length() {
            let uuidv7_string: String = crate::uuidv7::uuidv7_rs::generate_uuidv7();
            assert_eq!(uuidv7_string.len(), 36);
        }

        // Test UUIDv7 bytes
        #[pg_test]
        fn test_generate_uuidv7_bytes() {
            let uuidv7_bytes: Vec<u8> = crate::uuidv7::uuidv7_rs::generate_uuidv7_bytes();
            assert_eq!(uuidv7_bytes.len(), 16);
        }

        // Test UUIDv7 from string
        #[pg_test]
        fn test_generate_uuidv7_from_string() {
            let uuidv7_string: String = "67e55044-10b1-426f-9247-bb680e5fe0c8".to_string();
            // copy of uuidv7_string
            let uuidv7_from_string: String =
                crate::uuidv7::uuidv7_rs::generate_uuidv7_from_string(uuidv7_string);
            assert_eq!(uuidv7_from_string, "67e55044-10b1-426f-9247-bb680e5fe0c8");
        }

        // Test UUIDv7 parse
        #[pg_test]
        fn test_parse_uuidv7() {
            let uuidv7_string: String = "67e55044-10b1-426f-9247-bb680e5fe0c8".to_string();
            let uuidv7: String = crate::uuidv7::uuidv7_rs::parse_uuidv7(uuidv7_string.clone());
            assert_eq!(uuidv7, uuidv7_string);
        }

        #[pg_test]
        fn test_generate_uuidv7() {
            let uuid: String = crate::uuidv7::uuidv7_rs::generate_uuidv7();
            assert_eq!(uuid.len(), 36); // UUIDv7 length
            assert!(uuid.contains('-')); // UUID format check
        }
    }
}

pub mod ulid_rs {
    use std::str::FromStr;

    use pgrx::prelude::*;
    use rusty_ulid::{generate_ulid_bytes as ulid_bytes, Ulid};

    use rusty_ulid::generate_ulid_string;

    #[pg_extern]
    pub(crate) fn generate_ulid() -> String {
        let ulid_string: String = generate_ulid_string();
        ulid_string
    }

    #[pg_extern]
    pub(crate) fn generate_ulid_bytes() -> Vec<u8> {
        let ulid_bytes = ulid_bytes();
        ulid_bytes.to_vec()
    }

    #[pg_extern]
    pub fn generate_ulid_from_string(from_str: String) -> String {
        let result = Ulid::from_str(&from_str);
        result.unwrap().to_string()
    }

    // tests
    #[cfg(any(test, feature = "pg_test"))]
    #[pgrx::pg_schema]
    mod tests {
        use pgrx::pg_test;

        // Test Ulid length
        #[pg_test]
        fn test_generate_ulid_length() {
            let ulid_string: String = crate::ulid::ulid_rs::generate_ulid();
            assert_eq!(ulid_string.len(), 26);
        }

        // Test Ulid bytes
        #[pg_test]
        fn test_generate_ulid_bytes() {
            let ulid_bytes: Vec<u8> = crate::ulid::ulid_rs::generate_ulid_bytes();
            assert_eq!(ulid_bytes.len(), 16);
        }

        // Test Ulid from string
        #[pg_test]
        fn test_generate_ulid_from_string() {
            let ulid_string: String = "01CAT3X5Y5G9A62F1rFA6Tnice".to_string();
            // copy of ulid_string
            let ulid_from_string: String =
                crate::ulid::ulid_rs::generate_ulid_from_string(ulid_string);
            assert_eq!(ulid_from_string, "01CAT3X5Y5G9A62F1RFA6TN1CE");
        }
    }
}

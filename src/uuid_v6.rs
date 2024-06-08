pub mod uuidv6_rs {
    use std::io::{Error as IoError, ErrorKind};

    use getrandom::getrandom;
    use pgrx::pg_extern;
    use uuid::Uuid;

    /// Generate a new UUIDv6
    fn new_uuidv6() -> Uuid {
        let mut buf = [0u8; 6];
        let res = getrandom(&mut buf);
        if res.is_err() {
            panic!("failed to get random bytes for building uuidv6");
        }
        Uuid::now_v6(&buf)
    }

    /// Generate a UUID v6
    #[pg_extern]
    pub(crate) fn generate_uuidv6() -> String {
        new_uuidv6().as_hyphenated().to_string()
    }

    /// Generate a UUID v6, producing a Postgres text object
    #[pg_extern]
    pub(crate) fn generate_uuidv6_text() -> String {
        generate_uuidv6()
    }

    /// Generate a UUID v6, producing a Postgres uuid object
    #[pg_extern]
    pub(crate) fn generate_uuidv6_uuid() -> pgrx::Uuid {
        pgrx::Uuid::from_slice(new_uuidv6().as_bytes())
            .map_err(|e| IoError::new(ErrorKind::Other, e))
            .unwrap()
    }

    #[cfg(any(test, feature = "pg_test"))]
    #[pgrx::pg_schema]
    mod tests {
        use pgrx::pg_test;

        use crate::uuid_v6::uuidv6_rs::{generate_uuidv6, generate_uuidv6_uuid};

        /// Basic length test
        #[pg_test]
        fn test_uuidv6_len() {
            assert_eq!(generate_uuidv6().len(), 36);
        }

        /// Basic length test for bytes
        #[pg_test]
        fn test_uuidv6_len_uuid() {
            assert_eq!(generate_uuidv6_uuid().len(), 16);
        }

        /// Check version integer in UUID string
        #[pg_test]
        fn test_uuidv6_version_int() {
            let generated = generate_uuidv6();
            let c9 = generated.chars().nth(14);
            assert!(c9.is_some());
            assert_eq!(c9.unwrap(), '6');
        }
    }
}

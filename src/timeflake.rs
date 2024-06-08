pub mod timeflake_rs {
    use pgrx::prelude::*;
    use timeflake_rs::Timeflake;

    #[pg_extern]
    pub(crate) fn generate_timeflake() -> String {
        let result = Timeflake::random();
        result.unwrap().to_string()
    }

    #[pg_extern]
    pub(crate) fn generate_timeflake_bytes() -> Vec<u8> {
        let result = Timeflake::random();
        result.unwrap().as_uuid().as_bytes().to_vec()
    }

    #[pg_extern]
    pub fn generate_timeflake_uuid() -> pgrx::Uuid {
        pgrx::Uuid::from_slice(Timeflake::random().unwrap().as_uuid().as_bytes()).unwrap()
    }

    #[cfg(any(test, feature = "pg_test"))]
    #[pgrx::pg_schema]
    mod tests {
        use pgrx::pg_test;

        use crate::timeflake::timeflake_rs::{
            generate_timeflake, generate_timeflake_bytes, generate_timeflake_uuid,
        };

        #[pg_test]
        fn test_generate_timeflake() {
            let timeflake_string: String = generate_timeflake();
            assert!(timeflake_string.len() == 36);
        }

        #[pg_test]
        fn test_generate_timeflake_bytes() {
            let timeflake_bytes: Vec<u8> = generate_timeflake_bytes();
            assert!(timeflake_bytes.len() == 16);
        }

        #[pg_test]
        fn test_generate_timeflake_uuid() {
            let timeflake_uuid: pgrx::Uuid = generate_timeflake_uuid();
            assert!(timeflake_uuid.len() == 16);
        }
    }
}

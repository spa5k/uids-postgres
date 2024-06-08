pub mod pushid_rs {
    use pgrx::*;
    use pushid::PushId;
    use pushid::PushIdGen;

    /// Generate a PushId
    #[pg_extern]
    fn generate_pushid() -> String {
        PushId::new().get_id()
    }

    /// Generate a PushId as text
    #[pg_extern]
    fn generate_pushid_text() -> String {
        generate_pushid()
    }

    // tests
    #[cfg(any(test, feature = "pg_test"))]
    #[pg_schema]
    mod tests {
        use pgrx::*;

        #[pg_test]
        fn test_pushid_len() {
            let generated = crate::pushid::pushid_rs::generate_pushid();
            assert_eq!(generated.len(), 20);
        }

        #[pg_test]
        fn test_pushid_text_len() {
            let generated = crate::pushid::pushid_rs::generate_pushid_text();
            assert_eq!(generated.len(), 20);
        }
    }
}

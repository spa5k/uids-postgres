pub mod typeid_rs {
    use pgrx::prelude::*;
    use type_safe_id::{DynamicType, TypeSafeId};

    #[pg_extern]
    pub(crate) fn generate_typeid(prefix: &str) -> String {
        let dynamic_type = DynamicType::new(prefix).expect("invalid prefix");
        let id: TypeSafeId<DynamicType> = TypeSafeId::new_with_type(dynamic_type);
        id.to_string()
    }

    #[pg_extern]
    pub(crate) fn check_typeid(prefix: &str, id_str: &str) -> bool {
        let id: TypeSafeId<DynamicType> = id_str.parse().expect("invalid id");
        id.type_prefix() == prefix
    }

    // tests
    #[cfg(any(test, feature = "pg_test"))]
    #[pgrx::pg_schema]
    mod tests {
        use pgrx::pg_test;

        // Test TypeId length
        #[pg_test]
        fn test_generate_typeid_length() {
            let typeid_string: String = crate::typeid::typeid_rs::generate_typeid("custom");
            assert_eq!(typeid_string.len(), 33);
        }

        // Test TypeId prefix
        #[pg_test]
        fn test_generate_typeid_prefix() {
            let typeid_string: String = crate::typeid::typeid_rs::generate_typeid("custom");
            assert!(typeid_string.starts_with("custom_"));
        }

        // Test TypeId check
        #[pg_test]
        fn test_check_typeid() {
            let prefix = "custom";
            let id: String = crate::typeid::typeid_rs::generate_typeid(prefix);
            let is_match: bool = crate::typeid::typeid_rs::check_typeid(prefix, &id);
            assert!(is_match);

            let is_not_match: bool = crate::typeid::typeid_rs::check_typeid("other", &id);
            assert!(!is_not_match);
        }
    }
}

pub mod cuid2_rs {
    use cuid2;
    use pgrx::prelude::*;

    #[pg_extern]
    pub(crate) fn generate_cuid2() -> String {
        let id = cuid2::create_id();
        id.to_string()
    }

    #[pg_extern]
    pub(crate) fn check_cuid2(id_str: &str) -> bool {
        cuid2::is_cuid2(id_str)
    }

    // tests
    #[cfg(any(test, feature = "pg_test"))]
    #[pgrx::pg_schema]
    mod tests {
        use pgrx::pg_test;

        use crate::cuid2::cuid2_rs::{check_cuid2, generate_cuid2};

        #[pg_test]
        fn test_generate_cuid2() {
            let cuid2_string: String = generate_cuid2();
            assert!(cuid2_string.len() == 24);
        }

        #[pg_test]
        fn test_check_cuid2() {
            let cuid2_string: String = generate_cuid2();
            assert!(check_cuid2(&cuid2_string));
        }
    }
}

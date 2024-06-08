pub mod nanoid_rs {
    use nanoid::nanoid;
    use pgrx::prelude::*;

    #[pg_extern]
    pub(crate) fn generate_nanoid() -> String {
        let id = nanoid!();
        id
    }

    // Nano id extension that takes optional length as input argument
    #[pg_extern]
    pub(crate) fn generate_nanoid_length(length: i32) -> String {
        let length_as_usize = length as usize;
        let id = nanoid!(length_as_usize);
        id
    }

    #[pg_extern]
    pub(crate) fn generate_nanoid_length_c(length: i32, alphabets: &str) -> String {
        let alphabets_vec = alphabets.chars().collect::<Vec<char>>();
        let length_as_usize = length as usize;
        let id = nanoid!(length_as_usize, &alphabets_vec);
        id
    }

    #[pg_extern]
    pub(crate) fn generate_nanoid_c(alphabets: &str) -> String {
        let alphabets_vec = alphabets.chars().collect::<Vec<char>>();
        let id = nanoid!(21, &alphabets_vec);
        id
    }

    // tests
    #[cfg(any(test, feature = "pg_test"))]
    #[pgrx::pg_schema]
    mod tests {
        use pgrx::pg_test;

        // Test Nanoid length
        #[pg_test]
        fn test_generate_nanoid_length() {
            let nanoid_string: String = crate::nanoid::nanoid_rs::generate_nanoid_length(10);
            assert_eq!(nanoid_string.len(), 10);
        }

        // test nanoid without legnth
        #[pg_test]
        fn test_generate_nanoid() {
            let nanoid_string: String = crate::nanoid::nanoid_rs::generate_nanoid();
            assert_eq!(nanoid_string.len(), 21);
        }

        #[pg_test]
        fn test_generate_nanoid_custom() {
            let nanoid_string: String =
                crate::nanoid::nanoid_rs::generate_nanoid_c("1234567890abcdef");
            assert_eq!(nanoid_string.len(), 21);
        }
    }
}

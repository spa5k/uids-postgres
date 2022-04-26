mod ksuid;
mod nanoid;
mod ulid;

use pgx::*;

pg_module_magic!();

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_generate_ulid() {
        let ulid_string: String = crate::ulid::ulid::generate_ulid();
        assert_eq!(ulid_string.len(), 26);
    }

    #[pg_test]
    fn test_generate_ulid_from_string() {
        let ulid_string: String = "01CAT3X5Y5G9A62F1rFA6Tnice".to_string();
        // copy of ulid_string
        let ulid_from_string: String = crate::ulid::ulid::generate_ulid_from_string(ulid_string);
        assert_eq!(ulid_from_string, "01CAT3X5Y5G9A62F1RFA6TN1CE");
    }

    #[pg_test]
    fn test_generate_ulid_bytes() {
        let ulid_bytes: Vec<u8> = crate::ulid::ulid::generate_ulid_bytes();
        assert_eq!(ulid_bytes.len(), 16);
    }

    // Test Ksuid length
    #[pg_test]
    fn test_generate_ksuid_length() {
        let ksuid_string: String = crate::ksuid::ksuid::generate_ksuid();
        assert_eq!(ksuid_string.len(), 27);
    }

    // Test ksuid bytes
    #[pg_test]
    fn test_generate_ksuid_bytes() {
        let ksuid_bytes: Vec<u8> = crate::ksuid::ksuid::generate_ksuid_bytes();
        assert_eq!(ksuid_bytes.len(), 20);
    }

    // Test nanoid length
    #[pg_test]
    fn test_generate_nanoid_length() {
        let nanoid_string: String = crate::nanoid::nanoid::gen_nanoid_length(10);
        assert_eq!(nanoid_string.len(), 10);
    }

    // test nanoid without legnth
    #[pg_test]
    fn test_generate_nanoid() {
        let nanoid_string: String = crate::nanoid::nanoid::gen_nanoid();
        assert_eq!(nanoid_string.len(), 21);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}

pub mod ulid {
    use std::str::FromStr;

    use pgx::*;
    use rusty_ulid::{generate_ulid_bytes as ulid_bytes, Ulid};

    use rusty_ulid::generate_ulid_string;

    #[pg_extern]
    pub(crate) fn generate_ulid() -> String {
        let ulid_string: String = generate_ulid_string();
        return ulid_string;
    }

    #[pg_extern]
    pub(crate) fn generate_ulid_bytes() -> Vec<u8> {
        let ulid_bytes = ulid_bytes();
        return ulid_bytes.to_vec();
    }

    #[pg_extern]
    pub fn generate_ulid_from_string(from_str: String) -> String {
        let result = Ulid::from_str(&from_str);
        return result.unwrap().to_string();
    }
}

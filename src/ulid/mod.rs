pub mod ulid {
    use pgx::*;
    use rusty_ulid::generate_ulid_bytes as ulid_bytes;

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
}

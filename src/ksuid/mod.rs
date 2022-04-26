pub mod ksuid {
    use pgx::*;
    use svix_ksuid::{KsuidLike, KsuidMs};

    #[pg_extern]
    pub(crate) fn generate_ksuid() -> String {
        let ksuid_string = KsuidMs::new(None, None);
        return ksuid_string.to_string();
    }

    #[pg_extern]
    pub(crate) fn generate_ksuid_bytes() -> Vec<u8> {
        let ksuid_bytes = KsuidMs::new(None, None);
        return ksuid_bytes.bytes().to_vec();
    }
}

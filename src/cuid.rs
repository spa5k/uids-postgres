pub mod cuid_rs {
    use cuid::{cuid, is_cuid};
    use pgx::*;

    #[pg_extern]
    pub(crate) fn generate_cuid() -> String {
        match cuid() {
            Err(e) => error!("cuid generation failed: {}", e),
            Ok(id) => id,
        }
    }

    #[pg_extern]
    pub(crate) fn check_cuid(input: String) -> bool {
        is_cuid(input)
    }
}

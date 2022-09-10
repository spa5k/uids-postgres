pub mod pushid_rs {
    use pgx::*;
    use pushid::{PushId, PushIdGen};

    #[pg_extern]
    pub fn generate_pushid() -> String {
        PushId::new().get_id()
    }
}

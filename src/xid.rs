pub mod xid_rs {
    use pgx::*;

    use ::xid::new as new_xid;

    /// Generate a random xid UUID
    #[pg_extern]
    pub(crate) fn generate_xid() -> String {
        new_xid().to_string()
    }
}

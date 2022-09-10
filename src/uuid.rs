pub mod uuid_rs {
    use pgx::*;
    use uuid7::uuid7;
    use uuidv6::{Node, UUIDv6};

    /// Generate a UUID v6
    #[pg_extern]
    pub(crate) fn generate_uuidv6() -> String {
        let node = Node::new();
        UUIDv6::new(&node).create()
    }
    #[pg_extern]
    pub(crate) fn generate_uuidv7() -> String {
        uuid7().to_string()
    }
}

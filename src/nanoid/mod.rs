pub mod nanoid {
    use nanoid::nanoid;
    use pgx::*;

    #[pg_extern]
    pub(crate) fn gen_nanoid() -> String {
        let id = nanoid!();
        return id;
    }

    // Nano id extension that takes optional length as input argument
    #[pg_extern]
    pub(crate) fn gen_nanoid_length(length: i32) -> String {
        let length_as_usize = length as usize;
        let id = nanoid!(length_as_usize);
        return id;
    }
}

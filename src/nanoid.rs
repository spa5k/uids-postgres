pub mod nanoid_rs {
    use nanoid::nanoid;
    use pgx::*;

    #[pg_extern]
    pub(crate) fn gen_nanoid() -> String {
        let id = nanoid!();
        id
    }

    // Nano id extension that takes optional length as input argument
    #[pg_extern]
    pub(crate) fn gen_nanoid_length(length: i32) -> String {
        let length_as_usize = length as usize;
        let id = nanoid!(length_as_usize);
        id
    }

    #[pg_extern]
    pub(crate) fn gen_nanoid_c(length: i32, alphabets: &str) -> String {
        // Turn alphabets into a vec![]
        let alphabets_vec = alphabets.chars().collect::<Vec<char>>();
        let length_as_usize = length as usize;
        let id = nanoid!(length_as_usize, &alphabets_vec);
        id
    }
}

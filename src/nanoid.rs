pub mod nanoid_rs {
    use nanoid::nanoid;
    use pgx::*;

    #[pg_extern]
    pub(crate) fn generate_nanoid() -> String {
        let id = nanoid!();
        id
    }

    // Nano id extension that takes optional length as input argument
    #[pg_extern]
    pub(crate) fn generate_nanoid_length(length: i32) -> String {
        let length_as_usize = length as usize;
        let id = nanoid!(length_as_usize);
        id
    }

    #[pg_extern]
    pub(crate) fn generate_nanoid_length_c(length: i32, alphabets: &str) -> String {
        let alphabets_vec = alphabets.chars().collect::<Vec<char>>();
        let length_as_usize = length as usize;
        let id = nanoid!(length_as_usize, &alphabets_vec);
        id
    }

    #[pg_extern]
    pub(crate) fn generate_nanoid_c(alphabets: &str) -> String {
        let alphabets_vec = alphabets.chars().collect::<Vec<char>>();
        let id = nanoid!(21, &alphabets_vec);
        id
    }
}

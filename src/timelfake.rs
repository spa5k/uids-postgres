pub mod timeflake {
    use timeflake_rs::Timeflake;

    pub(crate) fn generate_timeflake() -> String {
        let generated = Timeflake::random();
        match generated {
            Ok(_) => generated.unwrap().to_string(),
            Err(e) => format!("timeflake generation failed: {}", e),
        }
    }
    // pub(crate) fn generate_timeflake_from_values() -> String {
    //     Timeflake::from_values(time, None).unwrap()
    // }
}

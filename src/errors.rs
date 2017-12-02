#[derive(Debug, Fail)]
pub enum DayError {
    #[fail(display = "not a digit")]
    NotDigit,
}

use errors::DayError;
use failure::Error;

/// Convert the given character to an integer.
pub fn char_to_digit(input: char) -> Result<u32, Error> {
    input.to_digit(10).ok_or(DayError::NotDigit).map_err(Into::into)
}

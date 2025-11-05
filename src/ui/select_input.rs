use std::str::FromStr;
use crate::ui::errors::UiError;

pub fn parse_value<T, E, F>(input: &str, error: E, validation: F) -> Result<T, E>
where
    E: Copy,
    T: FromStr,
    F: FnOnce(&T) -> bool,
{
    if input.is_empty() {
        return Err(error);
    }

    input.parse()
        .map_err(|_| error)
        .and_then(|value| {
            if validation(&value) {
                Ok(value)
            } else {
                Err(error)
            }
        })
}

pub fn parse_id(id: &str) -> Result<usize, UiError> {
    parse_value(
        id,
        UiError::NumberNotFoundError,
        |&n: &i32| n >= 0,
    ).map(|n| n as usize)
}

pub fn parse_fov(fov: &str) -> Result<f32, UiError> {
    parse_value(
        fov,
        UiError::BadFovError,
        |&n: &f32| n >= 0.0 && n < 180.0,
    )
}

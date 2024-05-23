use crate::StatusCode;

pub fn get_environment() -> (StatusCode, String) {
    (200, "Environment".to_string())
}

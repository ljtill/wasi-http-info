use crate::StatusCode;

pub fn get_version() -> (StatusCode, String) {
    (200, "Version".to_string())
}

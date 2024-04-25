use crate::StatusCode;

pub fn get_base() -> (StatusCode, String) {
    (200, "Base".to_string())
}

use crate::{Router, StatusCode};
use serde::Serialize;
use std::{collections::HashMap, env};

pub fn register(router: &mut Router) {
    router.route(crate::Method::Get, "/", get_base);
    router.route(crate::Method::Get, "/version", get_version);
    router.route(crate::Method::Get, "/environment", get_environment);
}

pub fn get_base() -> (StatusCode, String) {
    (200, "{}".to_string())
}

pub fn get_environment() -> (StatusCode, String) {
    let mut vars = HashMap::new();
    for (key, value) in env::vars() {
        vars.insert(key, value);
    }

    (200, serde_json::to_string(&vars).unwrap())
}

#[derive(Serialize)]
struct Version {
    commit: String,
}

pub fn get_version() -> (StatusCode, String) {
    // TODO: Implement this function
    let commit = "6298f3d";

    (
        200,
        serde_json::to_string(&Version {
            commit: commit.to_string(),
        })
        .unwrap(),
    )
}

use std::process::exit;

use reqwest::StatusCode;
use serde::Deserialize;
use url::Url;

use crate::env::{Env, TOKEN_ENV_VAR, URL_ENV_VAR};

const COURSE_API_ENDPOINT: &str = "/api/v1/courses";

#[derive(Deserialize, Debug)]
pub struct Course {
    pub id: usize,
    pub name: String,
    pub course_code: String,
}

pub async fn get_course(env: &Env) -> Vec<Course> {
    let url = Url::parse_with_params(
        env.canvas_url
            .join(&COURSE_API_ENDPOINT)
            .expect("Hardcoded API Endpoint should never fail")
            .as_str(),
        &[
            ("access_token", &env.access_token),
            ("enrollment_state", &"active".to_string()),
        ],
    )
    .expect("Hardcoded params should never fail");

    let response = reqwest::get(url).await.unwrap_or_else(|err| {
        if err.is_request() {
            eprintln!(
                "Error when sending request to url: \"{}\" Please check that {} is correct",
                &env.canvas_url, &URL_ENV_VAR
            );
        } else {
            eprintln!("Unknown GET request error: {}", &err);
        }
        exit(1);
    });

    match response.status() {
        StatusCode::OK => {}
        StatusCode::UNAUTHORIZED => {
            eprintln!("Invalid canvas access token. Please check that the token you provided in {} is still valid", &TOKEN_ENV_VAR);
            exit(1);
        }
        StatusCode::NOT_FOUND => {
            eprintln!(
                "Invalid url: \"{}\" Please check that {} is correct",
                &env.canvas_url, &URL_ENV_VAR
            );
            exit(1);
        }
        _ => {
            panic!("Unexpected status code after sending GET request!")
        }
    }

    let courses: Vec<Course> = response
        .json()
        .await
        .expect("Canvas API response in unexpected format");

    courses
}

use std::{env, process::exit};

use url::Url;

pub const TOKEN_ENV_VAR: &str = "CANVAS_ACCESS_TOKEN";
pub const URL_ENV_VAR: &str = "CANVAS_URL";

pub struct Env {
    pub access_token: String,
    pub canvas_url: Url,
}

impl Env {
    pub fn get() -> Self {
        let access_token = env::var(&TOKEN_ENV_VAR).unwrap_or_else(|_| {
            eprintln!("Please set the {} environment variable", &TOKEN_ENV_VAR);
            exit(1);
        });

        let canvas_url = env::var(&URL_ENV_VAR).unwrap_or_else(|_| {
            eprintln!("Please set the {} environment variable", &URL_ENV_VAR);
            exit(1);
        });
        let canvas_url = Url::parse(&canvas_url).unwrap_or_else(|_| {
            eprintln!(
                "Unable to parse URL: \"{}\" from environment variable {}",
                &canvas_url, &URL_ENV_VAR
            );
            exit(1);
        });

        Env {
            access_token,
            canvas_url,
        }
    }
}

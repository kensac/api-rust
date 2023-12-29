use std::fs;

use handlebars::Handlebars;
use hyper::StatusCode;
use mrml;
use sendgrid::{Mail, SGClient};
use serde_json::json;

pub async fn send_test_email() -> Result<StatusCode, StatusCode> {
    /*     let key = std::env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");
    let sg = SGClient::new(key); */

    let mail_text = mrml::parse(fs::read_to_string("registration.mjml").unwrap().as_str())
        .expect("Failed to parse HTML");
    let opts = mrml::prelude::render::RenderOptions::default();
    let content = match mail_text.render(&opts) {
        Ok(content) => content,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let reg = Handlebars::new();
    let template = reg
        .render_template(
            &content,
            &json!(
                {
                    "firstName": "John Doe",
                    "address": "test@email.com",
                    "date": chrono::Local::now().format("%m/%d/%Y").to_string(),
                }
            ),
        )
        .unwrap();
    println!("{}", template);

    return Ok(StatusCode::OK);
}

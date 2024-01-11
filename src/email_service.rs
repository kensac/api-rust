use std::fs;

use axum::extract::State;
use handlebars::Handlebars;
use hyper::StatusCode;
use mrml;
use sendgrid::{Destination, Mail, SGClient};
use serde_json::json;

use crate::base_types::AppState;

pub async fn send_test_email(State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
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

    let mail_entity = Mail::new()
        .add_from("")
        .add_to(Destination {
            address: "",
            name: "",
        })
        .add_subject("HackPSU API Rust Test Email")
        .add_html(template.as_str());

    state.send_grid.send(mail_entity).await.unwrap();

    Ok(StatusCode::OK)
}

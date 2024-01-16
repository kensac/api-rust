use axum::extract::State;
use handlebars::Handlebars;
use hyper::StatusCode;
use mrml::{self, parse, prelude::render::RenderOptions};
use sendgrid::{Destination, Mail};

use crate::base_types::AppState;

pub struct MailData<'a> {
    from: &'a str,
    to: Destination<'a>,
    subject: &'a str,
    html: &'a str,
}

impl<'a> MailData<'a> {
    pub const fn new(from: &'a str, to: Destination<'a>, subject: &'a str, html: &'a str) -> Self {
        Self {
            from,
            to,
            subject,
            html,
        }
    }
}

pub async fn send_email(
    State(state): State<AppState>,
    mail_data: MailData<'_>,
) -> Result<StatusCode, (String, StatusCode)> {
    let mail_entity = Mail::new()
        .add_from(mail_data.from)
        .add_to(mail_data.to)
        .add_subject(mail_data.subject)
        .add_html(mail_data.html);

    match state.send_grid.send(mail_entity).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => Err((err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub fn render_template(
    template_html: &str,
    template_data: &serde_json::Value,
) -> Result<String, (String, StatusCode)> {
    let mail_text = parse(template_html).unwrap();
    let opts = RenderOptions::default();
    let content = match mail_text.render(&opts) {
        Ok(content) => content,
        Err(_) => {
            return Err((
                String::from("Failed to render template"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };

    let reg = Handlebars::new();
    match reg.render_template(&content, &template_data) {
        Ok(template) => Ok(template),
        Err(_) => Err((
            String::from("Failed to populate template"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

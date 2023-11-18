use utoipa::openapi::security::{
    ApiKey, ApiKeyValue, Flow, HttpAuthScheme, HttpBuilder, Password, Scopes, SecurityScheme,
};
use utoipa::{Modify, OpenApi};

use crate::routes::{extra_credit_classes, hackathons, locations};
use crate::utils;

#[derive(OpenApi)]
#[openapi(
    paths(
        utils::health_check,
        hackathons::create_hackathon,
        hackathons::get_hackathon,
        hackathons::get_hackathon_by_id,
        hackathons::delete_hackathon_by_id,
        hackathons::set_active_hackathon,
        extra_credit_classes::create_extra_credit_class,
        extra_credit_classes::get_all_extra_credit_classes,
        extra_credit_classes::get_extra_credit_class_by_id,
        extra_credit_classes::delete_extra_credit_class_by_id,
        locations::create_location,
        locations::get_all_locations,
        locations::get_location_by_id,
        locations::delete_location_by_id
    ),
    components(
        schemas(
            hackathons::CreateHackathonEntity,
            hackathons::HackathonEntity,
            extra_credit_classes::CreateExtraCreditClassEntity,
            locations::CreateLocationEntity
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "hackathons", description = "Hackathon related operations"),
        (name = "extra_credit_class", description = "Extra Credit Class related operations"),
        (name = "location", description = "Location related operations")
    )
)]
pub struct ApiDoc;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            components.add_security_scheme(
                "OAuth2",
                SecurityScheme::OAuth2(utoipa::openapi::security::OAuth2::new(vec![
                    Flow::Password(Password::new(
                        "/registration",
                        Scopes::from_iter(vec![
                            ("None".to_string(), "None".to_string()),
                            ("Volunteer".to_string(), "Volunteer".to_string()),
                            ("Team".to_string(), "Team".to_string()),
                            ("Exec".to_string(), "Exec".to_string()),
                            ("Tech".to_string(), "Tech".to_string()),
                            ("Finance".to_string(), "Finance".to_string()),
                        ]),
                    )),
                ])),
            );
        }
    }
}

pub struct SecurityAddon;

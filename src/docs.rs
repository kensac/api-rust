use utoipa::openapi::security::{
    /* ApiKey, ApiKeyValue, */ HttpAuthScheme, HttpBuilder, SecurityScheme,
};
use utoipa::{Modify, OpenApi};

use crate::routes::{events, extra_credit_classes, hackathons, locations};
use crate::{prisma, utils};

#[derive(OpenApi)]
#[openapi(
    paths(
        utils::health_check,
        hackathons::create_hackathon,
        hackathons::get_all_hackathon,
        hackathons::get_hackathon_by_id,
        hackathons::delete_hackathon_by_id,
        hackathons::set_active_hackathon,
        hackathons::get_active_hackathon,

        locations::create_location,
        locations::get_all_locations,
        locations::get_location_by_id,
        locations::delete_location_by_id,

        events::create_event,
        events::get_all_events,
        events::get_event_by_id,
        events::delete_event_by_id,
        events::check_in_user_to_event,

        extra_credit_classes::create_extra_credit_class,
        extra_credit_classes::get_all_extra_credit_classes,
        extra_credit_classes::get_extra_credit_class_by_id,
        extra_credit_classes::delete_extra_credit_class_by_id,
    ),
    components(
        schemas(
            hackathons::CreateHackathonEntity,
            hackathons::HackathonEntity,

            locations::CreateLocationEntity,
            locations::LocationEntity,

            events::CreateEventEntity,
            events::CheckInUserToEventEntity,
            prisma::EventType,
            events::EventEntity,

            extra_credit_classes::CreateExtraCreditClassEntity,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
/*         (name = "hackathons", description = "Hackathon related operations"),
        (name = "extra_credit_class", description = "Extra Credit Class related operations"),
        (name = "locations", description = "Location related operations") */
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
                        .description(Some("Firebase idToken of the logged in user passed in the Authorization header with format Bearer idToken".to_owned()))
                        .build(),
                ),
            );
            components.add_security_scheme("http", 
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .description(Some("Firebase idToken of the logged in user passed in the Authorization header with format Bearer idToken".to_owned()))
                        .build(),
                )
            );
            /*             components.add_security_scheme(
                "OAuth2",
                SecurityScheme::OAuth2(utoipa::openapi::security::OAuth2::new(vec![
                    Flow::Password(Password::new(
                        "/registration",
                        Scopes::from_iter(vec![
                            ("None".to_owned(), "None".to_owned()),
                            ("Volunteer".to_owned(), "Volunteer".to_owned()),
                            ("Team".to_owned(), "Team".to_owned()),
                            ("Exec".to_owned(), "Exec".to_owned()),
                            ("Tech".to_owned(), "Tech".to_owned()),
                            ("Finance".to_owned(), "Finance".to_owned()),
                        ]),
                    )),
                ])),
            ); */
        }
    }
}

pub struct SecurityAddon;

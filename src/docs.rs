use utoipa::OpenApi;

use crate::routes::{ extra_credit_classes, hackathons, locations };
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
    tags(
        (name = "hackathons", description = "Hackathon related operations"),
        (name = "extra_credit_class", description = "Extra Credit Class related operations"),
        (name = "location", description = "Location related operations")
    )
)]
pub struct ApiDoc;

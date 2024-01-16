use chrono::FixedOffset;
use utoipa::ToSchema;

use crate::prisma::hackathon;

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
/* #[serde(remote = "Data")] */
pub struct HackathonEntity {
    id: String,
    name: String,
    start_time: chrono::DateTime<FixedOffset>,
    end_time: chrono::DateTime<FixedOffset>,
    active: bool,
    /*     event: Option<Vec<prisma::event::Data>>,
    extra_credit_class: Option<Vec<prisma::extra_credit_class::Data>>,
    project: Option<Vec<prisma::project::Data>>,
    sponsor: Option<Vec<prisma::sponsor::Data>>,
    registration: Option<Vec<prisma::registration::Data>>,
    scan: Option<Vec<prisma::scan::Data>>,
    score: Option<Vec<prisma::score::Data>>, */
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateHackathonEntity {
    pub name: String,
    pub start_time: chrono::DateTime<FixedOffset>,
    pub end_time: chrono::DateTime<FixedOffset>,
}

hackathon::partial_unchecked!(
    HackathonUpdateEntity {
        name
        start_time
        end_time
        active
    }
);

// Recursive expansion of ToSchema macro
// ======================================

impl<'__s> utoipa::ToSchema<'__s> for HackathonUpdateEntity {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            "HackathonUpdateEntity",
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    "name",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::String)
                        .nullable(true),
                )
                .property(
                    "startTime",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::String)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::DateTime,
                        )))
                        .nullable(true),
                )
                .property(
                    "endTime",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::String)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::DateTime,
                        )))
                        .nullable(true),
                )
                .property(
                    "active",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::Boolean)
                        .nullable(true),
                )
                .into(),
        )
    }
}

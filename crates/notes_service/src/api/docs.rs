use crate::api::routes;
use crate::models::api::note;
use crate::models::database::note as db_note;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::user_notes::create_note,
        routes::user_notes::get_all_notes,
        routes::user_notes::get_note_by_id,
        routes::user_notes::update_note,
        routes::user_notes::delete_note,
        routes::user_notes::delete_all_notes_of_user,

        routes::admin_notes::update_note,
        routes::admin_notes::delete_note,
        routes::admin_notes::delete_all_notes_of_user,
        routes::admin_notes::get_note_by_id,
        routes::admin_notes::get_all_notes,

        shared::health::routes::health_check
    ),
    components(
        schemas(
            note::CreateNoteRequest,
            note::UpdateNoteRequest,
            note::NoteListResponse,
            note::SingleNoteResponse,
            db_note::Note,
        )
    ),
    tags(
        (name = "Notes (Admin)", description = "Notes Management API for admin users"),
        (name = "Notes (User)", description = "Notes Management API for normal users")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            utoipa::openapi::security::SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}

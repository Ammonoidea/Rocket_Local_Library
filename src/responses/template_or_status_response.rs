use rocket::http::Status;
use rocket::response::Responder;
use rocket_dyn_templates::Template;

#[derive(Debug, Responder)]
pub enum TemplateOrStatusResponse {
    Template(Template),
    Status(Status),
}

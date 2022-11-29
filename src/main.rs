/* ---------------------------------------------- *
 * A simple website that shows the user their IP. *
 * ---------------------------------------------- */

// Imports:
use rocket::{get, Request, routes};
use rocket_dyn_templates::{context, Template};
use std::net::IpAddr;
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::request::{FromRequest, Outcome};

#[get("/")]
fn ip(req: RealIP) -> Template {
    // Get the client IP from the X-Real-IP header.
    Template::render("index", context! { client_address: req.0 })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Launch Rocket.
    let _ = rocket::build()
        .mount("/", routes![ip])
        .attach(Template::fairing())
        .launch()
        .await?;

    // Return Ok.
    Ok(())
}

// Real IP type.
struct RealIP(IpAddr);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RealIP {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(address) = request.real_ip() {
            Success(RealIP(address))
        } else {
            Failure((Status::InternalServerError, ()))
        }
    }
}
/* ---------------------------------------------- *
 * A simple website that shows the user their IP. *
 * ---------------------------------------------- */

use std::net::{IpAddr, Ipv4Addr};
// Imports:
use rocket::{get, Request, routes};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn ip(req: Request) -> Template {
    // Get the client IP from the X-Real-IP header, or 0.0.0.0 if unknown.
    let addr = req.real_ip();
    if let Some(address) = addr {
        Template::render("index", context! { client_address: address.to_string() })
    } else {
        Template::render("index", context! { client_address: "Error" })
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Starting server...");

    // Launch Rocket.
    let _ = rocket::build()
        .mount("/", routes![ip])
        .launch()
        .await?;

    // Return Ok.
    Ok(())
}

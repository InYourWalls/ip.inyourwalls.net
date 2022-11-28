/* ---------------------------------------------- *
 * A simple website that shows the user their IP. *
 * ---------------------------------------------- */

// Imports:
use rocket::{get, Request, routes};
use rocket_dyn_templates::{context, Template};
use std::net::{IpAddr, Ipv4Addr};

#[get("/")]
fn ip(req: Request) -> Template {
    // Get the client IP from the X-Real-IP header.
    let addr = req.real_ip();
    if let Some(address) = addr {
        // Return the IP address.
        Template::render("index", context! { client_address: address.to_string() })
    } else {
        // We don't know what it is.
        Template::render("index", context! { client_address: String::from("Unknown") })
    }
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

use clap::Parser;
use rocket::{post, routes};

#[derive(Parser)]
pub struct Command {
    #[clap(default_value = "80")]
    pub port: u16,
}

#[rocket::launch]
fn rocket() -> _ {
    pretty_env_logger::init();

    let command = Command::parse();

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", command.port)))
        .mount("/", routes![root])
}

#[post("/", format = "json", data = "<body>")]
async fn root(body: &str) -> &'static str {
    println!("{body}");
    "Hello world!"
}

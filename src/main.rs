mod db;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    // load up environment variables from the .env file
    dotenvy::dotenv().ok();
    let client = db::create_client();
    db::create_table(client);
    rocket::build().mount("/", routes![index])
}

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::{database, diesel as rocket_diesel};

#[database("mysql_db")]
struct MySqlConn(rocket_diesel::MysqlConnection);

#[get("/")]
fn index() -> &'static str {
        "Hello, world!"
}

#[launch]
fn rocket() -> _ {
        rocket::build()
                .attach(MySqlConn::fairing())
                .mount("/", routes![index])
}

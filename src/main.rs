#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::*;
use schema::posts::dsl::*;

use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel as rocket_diesel};

#[database("mysql_db")]
struct MySqlConn(rocket_diesel::MysqlConnection);

#[get("/")]
fn index() -> &'static str {
        "Hello, world!"
}

#[get("/post/<post_id>")]
async fn get_post(conn: MySqlConn, post_id: i64) -> Option<Json<Post>> {
        conn.run(move |c| posts.filter(id.eq(post_id)).first(c))
                .await
                .map(Json)
                .ok()
}

#[launch]
fn rocket() -> _ {
        rocket::build()
                .attach(MySqlConn::fairing())
                .mount("/", routes![index, get_post])
}

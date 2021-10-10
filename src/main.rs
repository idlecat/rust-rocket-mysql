#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::*;
use schema::posts::dsl::*;

use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel as rocket_diesel};

#[database("mysql_db")]
struct Db(rocket_diesel::MysqlConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
fn index() -> &'static str {
        "Hello, world!"
}

#[get("/post/<post_id>")]
async fn get_post(db: Db, post_id: i64) -> Option<Json<Post>> {
        db.run(move |c| posts.filter(id.eq(post_id)).first(c))
                .await
                .map(Json)
                .ok()
}

#[post("/post/create", data = "<post>")]
async fn create_post(db: Db, post: Json<NewPost>) -> Json<Post> {
        let insert_rows = db
                .run(move |conn| diesel::insert_into(posts).values(&*post).execute(conn))
                .await
                .ok()
                .unwrap();
        Json(Post {
                id: insert_rows as i64,
                content: "good".to_string(),
        })
}

#[launch]
fn rocket() -> _ {
        rocket::build()
                .attach(Db::fairing())
                .mount("/", routes![index, get_post, create_post])
}

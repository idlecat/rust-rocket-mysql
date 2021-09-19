use serde::Serialize;

// pub mod schema;
//use schema::posts::dsl::*;
table! {
  posts (id) {
      id -> Bigint,
      content -> Longtext,
  }
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize)]
pub struct Post {
  pub id: i64,
  pub content: String,
}

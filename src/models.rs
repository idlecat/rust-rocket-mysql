use serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Debug, Clone, Queryable, Insertable, Serialize)]
pub struct Post {
  pub id: i64,
  pub content: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
  pub content: String,
}

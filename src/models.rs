use serde::Serialize;

use crate::schema::posts;

#[derive(Debug, Clone, Queryable, Insertable, Serialize)]
pub struct Post {
  pub id: i64,
  pub content: String,
}

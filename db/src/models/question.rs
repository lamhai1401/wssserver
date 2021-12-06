// question.rs
use chrono::{DateTime, Utc};

// add Insertiable
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::questions;

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Question {
    pub fn get_all(conn: &PgConnection) -> Result<Vec<Question>, diesel::result::Error> {
        use crate::schema::questions::dsl::{body, questions};
        let all_questions = questions.order(body).load::<Question>(conn)?;
        Ok(all_questions)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub body: String,
}

// question.rs
use chrono::{DateTime, Utc};

// add Insertiable
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::questions;

// local error
use errors::Error;

#[derive(Debug, Insertable, Serialize, Deserialize, Queryable)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Question {
    pub fn create(conn: &PgConnection, body: &String) -> Result<Question, Error> {
        use crate::schema::questions::dsl::questions;

        let question = diesel::insert_into(questions)
            .values(NewQuestion { body: body.clone() })
            .get_result::<Question>(conn)?;

        Ok(question)
    }
    pub fn get_all(conn: &PgConnection) -> Result<Vec<Question>, Error> {
        use crate::schema::questions::dsl::{body, questions};

        let all_questions = questions.order(body).load::<Question>(conn)?;

        Ok(all_questions)
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub body: String,
}

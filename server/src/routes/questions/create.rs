use actix_web::{
    web::{block, Data, Json},
    Result,
};

use serde::{Deserialize, Serialize};

use db::{get_conn, models::Question, PgPool};
use errors::Error;

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateRequest {
    body: String,
}

pub async fn create(
    pool: Data<PgPool>,
    params: Json<CreateRequest>,
) -> Result<Json<Question>, Error> {
    if params.body == "" {
        return Err(Error::BadRequest("Body is required".to_string()));
    }

    let connection = get_conn(&pool)?;

    let question = block(move || Question::create(&connection, &params.body)).await?;

    Ok(Json(question))
}
#[cfg(test)]
mod tests {
    use diesel::{self, RunQueryDsl};

    use db::{
        get_conn,
        models::{NewQuestion, Question},
        new_pool,
        schema::questions,
    };

    use crate::tests;

    #[actix_rt::test]
    pub async fn test_create_question() {
        let pool = new_pool();
        let conn = get_conn(&pool).unwrap();

        let res: (u16, Question) = tests::test_post(
            "/api/questions",
            NewQuestion {
                body: "A new question".to_string(),
            },
        )
        .await;

        assert_eq!(res.0, 200);
        assert_eq!(res.1.body, "A new question");

        let result_questions = questions::dsl::questions.load::<Question>(&conn).unwrap();
        assert_eq!(result_questions.len(), 1);
        assert_eq!(result_questions[0].body, "A new question");

        diesel::delete(questions::dsl::questions)
            .execute(&conn)
            .unwrap();
    }
}
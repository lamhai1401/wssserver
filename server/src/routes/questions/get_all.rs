use actix_web::{
    web::{block, Data, Json},
    Result,
};

use errors::Error;

use db::{get_conn, models::Question, PgPool};

pub async fn get_all(pool: Data<PgPool>) -> Result<Json<Vec<Question>>, Error> {
    let connection = get_conn(&pool)?;

    let questions = block(move || Question::get_all(&connection)).await?;

    Ok(Json(questions))
}

#[cfg(test)]
mod tests {
    // not a type we're using directly, but we need to pull in this trait.
    // If you leave it out, you'll see a compiler error for calling the query to insert.
    // The Rust compiler is pretty good about telling you what traits you're missing.
    use diesel::RunQueryDsl;

    use db::{
        get_conn,
        models::{NewQuestion, Question},
        new_pool,
        schema::questions,
    };

    use crate::tests;

    #[actix_rt::test]
    async fn test_get_all_returns_questions() {
        let pool = new_pool();
        let conn = get_conn(&pool).unwrap();

        diesel::insert_into(questions::table)
            .values(NewQuestion {
                body: "one question".to_string(),
            })
            .execute(&conn)
            .unwrap();

        let res: (u16, Vec<Question>) = tests::test_get("/api/questions").await;
        assert_eq!(res.0, 200);
        assert_eq!(res.1.len(), 1);
        assert_eq!(res.1[0].body, "one question");
    }
}

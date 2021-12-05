use actix_web::{
    error::Error,
    web::{block, Data, Json},
    Result,
};

use db::{get_conn, models::Question, PgPool};

pub async fn get_all(pool: Data<PgPool>) -> Result<Json<Vec<Question>>, Error> {
    let connection = get_conn(&pool);

    let questions = block(move || Question::get_all(&connection)).await?;

    Ok(Json(questions))
}

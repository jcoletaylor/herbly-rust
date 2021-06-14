use crate::helpers;
use crate::models::db_results;
use crate::models::web_results;
use sqlx::{query_as, Error as SqlxError, PgPool};
use tide::Error;

fn convert_db_single_formula_to_web(formula: &db_results::Formula) -> web_results::Formula {
    web_results::Formula {
        id: formula.id,
        name: String::from(&formula.name),
        pinyin: helpers::copy_string_or_none(&formula.pinyin),
        hanzi: helpers::copy_string_or_none(&formula.hanzi),
        english: helpers::copy_string_or_none(&formula.english),
        common_english: helpers::copy_string_or_none(&formula.common_english),
    }
}

async fn get_single_formula(
    name: &String,
    db_pool: &PgPool,
) -> Result<Option<db_results::Formula>, SqlxError> {
    let formula = query_as!(
        db_results::Formula,
        r#"
        SELECT
            formulas.id,
            formulas.name,
            formulas.pinyin,
            formulas.hanzi,
            formulas.english,
            formulas.common_english
        FROM
            formulas
        WHERE formulas.name = $1
        LIMIT 1
        "#,
        name
    )
    .fetch_optional(db_pool)
    .await?;
    Ok(formula)
}

pub async fn get_one(name: String, db_pool: &PgPool) -> tide::Result<web_results::Formula> {
    let maybe_formula = get_single_formula(&name, db_pool)
        .await
        .map_err(|e| Error::new(409, e))?;

    match maybe_formula {
        Some(formula) => Ok(convert_db_single_formula_to_web(&formula)),
        None => Err(Error::from_str(
            404,
            format!("No formula found for {}", name),
        )),
    }
}

async fn get_all_formulas(
    limit: i64,
    offset: i64,
    db_pool: &PgPool,
) -> Result<Vec<db_results::Formula>, SqlxError> {
    let formulas = query_as!(
        db_results::Formula,
        r#"
        SELECT
            formulas.id,
            formulas.name,
            formulas.pinyin,
            formulas.hanzi,
            formulas.english,
            formulas.common_english
        FROM
            formulas
        ORDER BY formulas.name ASC LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(db_pool)
    .await?;
    Ok(formulas)
}

pub async fn get_all(
    limit: i64,
    offset: i64,
    db_pool: &PgPool,
) -> tide::Result<Vec<web_results::Formula>> {
    let formulas = get_all_formulas(limit, offset, db_pool)
        .await
        .map_err(|e| Error::new(409, e))?;
    let mut results: Vec<web_results::Formula> = vec![];
    for formula in formulas.iter() {
        results.push(convert_db_single_formula_to_web(formula));
    }
    Ok(results)
}

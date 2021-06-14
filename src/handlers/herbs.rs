use crate::helpers;
use crate::models::db_results;
use crate::models::web_results;
use sqlx::{query_as, Error as SqlxError, PgPool};
use tide::Error;

fn convert_db_single_herb_to_web(herb: &db_results::Herb) -> web_results::Herb {
    web_results::Herb {
        id: herb.id,
        name: String::from(&herb.name),
        pinyin: helpers::copy_string_or_none(&herb.pinyin),
        hanzi: helpers::copy_string_or_none(&herb.hanzi),
        latin: helpers::copy_string_or_none(&herb.latin),
        common_english: helpers::copy_string_or_none(&herb.common_english),
        pharm_latin: helpers::copy_string_or_none(&herb.pharm_latin),
        herb_category_id: herb.herb_category_id,
        herb_category: String::from(&herb.herb_category),
        herb_actions: None,
        herb_properties: None,
        herb_dosages: None,
        herb_warnings: None,
        herb_notes: None,
        herb_combination_herbs: None,
        herb_combinations: None,
    }
}

async fn get_single_herb(
    name: &String,
    db_pool: &PgPool,
) -> Result<Option<db_results::Herb>, SqlxError> {
    let herb = query_as!(
        db_results::Herb,
        r#"
        SELECT
            herbs.id,
            herbs.name,
            herbs.pinyin,
            herbs.hanzi,
            herbs.herb_category_id,
            herbs.latin,
            herbs.pharm_latin,
            herbs.common_english,
            herb_categories.name AS herb_category
        FROM
            herbs
            INNER JOIN herb_categories ON herbs.herb_category_id = herb_categories.id
        WHERE herbs.name = $1
        LIMIT 1
        "#,
        name
    )
    .fetch_optional(db_pool)
    .await?;
    Ok(herb)
}

pub async fn get_one(name: String, db_pool: &PgPool) -> tide::Result<web_results::Herb> {
    let maybe_herb = get_single_herb(&name, db_pool)
        .await
        .map_err(|e| Error::new(409, e))?;

    match maybe_herb {
        Some(herb) => Ok(convert_db_single_herb_to_web(&herb)),
        None => Err(Error::from_str(404, format!("No herb found for {}", name))),
    }
}

async fn get_all_herbs(
    limit: i64,
    offset: i64,
    db_pool: &PgPool,
) -> Result<Vec<db_results::Herb>, SqlxError> {
    let herbs = query_as!(
        db_results::Herb,
        r#"
        SELECT
            herbs.id,
            herbs.name,
            herbs.pinyin,
            herbs.hanzi,
            herbs.herb_category_id,
            herbs.latin,
            herbs.pharm_latin,
            herbs.common_english,
            herb_categories.name AS herb_category
        FROM
            herbs
            INNER JOIN herb_categories ON herbs.herb_category_id = herb_categories.id
        ORDER BY herbs.name ASC LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(db_pool)
    .await?;
    Ok(herbs)
}

pub async fn get_all(
    limit: i64,
    offset: i64,
    db_pool: &PgPool,
) -> tide::Result<Vec<web_results::Herb>> {
    let herbs = get_all_herbs(limit, offset, db_pool)
        .await
        .map_err(|e| Error::new(409, e))?;
    let mut results: Vec<web_results::Herb> = vec![];
    for herb in herbs.iter() {
        results.push(convert_db_single_herb_to_web(herb));
    }
    Ok(results)
}

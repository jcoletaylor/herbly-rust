use crate::helpers;
use crate::models::query_results;
use crate::models::tide_results;
use sqlx::{query_as, PgPool};
use tide::Error;

fn convert_query_to_tide(row: &query_results::Herb) -> tide_results::Herb {
    tide_results::Herb {
        id: row.id,
        name: String::from(&row.name),
        pinyin: helpers::copy_string_or_none(&row.pinyin),
        hanzi: helpers::copy_string_or_none(&row.hanzi),
        latin: helpers::copy_string_or_none(&row.latin),
        common_english: helpers::copy_string_or_none(&row.common_english),
        pharm_latin: helpers::copy_string_or_none(&row.pharm_latin),
        herb_category_id: row.herb_category_id,
        herb_category: String::from(&row.herb_category),
        herb_actions: None,
        herb_properties: None,
        herb_dosages: None,
        herb_warnings: None,
        herb_notes: None,
        herb_combination_herbs: None,
        herb_combinations: None,
    }
}

pub async fn get_one(name: String, db_pool: &PgPool) -> tide::Result<tide_results::Herb> {
    let row = query_as!(
        query_results::Herb,
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
    .await
    .map_err(|e| Error::new(409, e))?;

    match row {
        Some(r) => Ok(convert_query_to_tide(&r)),
        None => Err(Error::from_str(404, format!("No herb found for {}", name))),
    }
}

pub async fn get_all(
    limit: i64,
    offset: i64,
    db_pool: &PgPool,
) -> tide::Result<Vec<tide_results::Herb>> {
    let rows = query_as!(
        query_results::Herb,
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
    .await
    .map_err(|e| Error::new(409, e))?;
    let mut results: Vec<tide_results::Herb> = vec![];
    for row in rows.iter() {
        results.push(convert_query_to_tide(&row));
    }
    Ok(results)
}

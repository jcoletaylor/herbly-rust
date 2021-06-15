use crate::helpers;
use crate::models::api_results;
use crate::models::db_results;
use sqlx::{query_as, Error as SqlxError, PgPool};
use tide::{Error, StatusCode};

pub type HerbApiResult = api_results::ApiResult<api_results::Herb>;
pub type HerbsApiResult = api_results::ApiResult<Vec<api_results::Herb>>;

fn convert_db_to_api(
    herb: &db_results::Herb,
    herb_actions: &Vec<db_results::HerbAction>,
) -> api_results::Herb {
    let mut api_herb_actions: Vec<api_results::HerbAction> = vec![];
    for herb_action in herb_actions.iter() {
        api_herb_actions.push(api_results::HerbAction {
            id: herb_action.id,
            herb_id: herb_action.herb_id,
            herb_action_type_id: herb_action.herb_action_type_id,
            herb_action_type: String::from(&herb_action.herb_action_type),
        });
    }
    api_results::Herb {
        id: herb.id,
        name: String::from(&herb.name),
        pinyin: helpers::copy_string_or_none(&herb.pinyin),
        hanzi: helpers::copy_string_or_none(&herb.hanzi),
        latin: helpers::copy_string_or_none(&herb.latin),
        common_english: helpers::copy_string_or_none(&herb.common_english),
        pharm_latin: helpers::copy_string_or_none(&herb.pharm_latin),
        herb_category_id: herb.herb_category_id,
        herb_category: String::from(&herb.herb_category),
        herb_actions: Some(api_herb_actions),
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

async fn get_herb_actions(
    herb_ids: &Vec<i64>,
    db_pool: &PgPool,
) -> Result<Vec<db_results::HerbAction>, SqlxError> {
    let actions = query_as!(
        db_results::HerbAction,
        r#"
        SELECT
            herb_actions.id,
            herb_actions.herb_id,
            herb_actions.herb_action_type_id,
            herb_action_types.name AS herb_action_type
        FROM
            herb_actions
            INNER JOIN herb_action_types ON herb_actions.herb_action_type_id = herb_action_types.id
        WHERE herb_actions.herb_id = ANY($1)
        ORDER BY herb_actions.herb_id ASC
        "#,
        herb_ids
    )
    .fetch_all(db_pool)
    .await?;
    Ok(actions)
}

pub async fn get_one(name: String, db_pool: &PgPool) -> tide::Result<HerbApiResult> {
    let maybe_herb = get_single_herb(&name, db_pool)
        .await
        .map_err(|e| Error::new(StatusCode::NotFound, e))?;

    match maybe_herb {
        Some(herb) => {
            let herb_actions = get_herb_actions(&vec![herb.id], db_pool)
                .await
                .map_err(|e| Error::new(StatusCode::NotFound, e))?;
            let api_result = HerbApiResult {
                data: Some(convert_db_to_api(&herb, &herb_actions)),
                error: None,
            };
            Ok(api_result)
        }
        None => Err(Error::from_str(
            StatusCode::NotFound,
            format!("No herb found for {}", name),
        )),
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

pub async fn get_all(limit: i64, offset: i64, db_pool: &PgPool) -> tide::Result<HerbsApiResult> {
    let herbs = get_all_herbs(limit, offset, db_pool).await?;
    let mut results: Vec<api_results::Herb> = vec![];
    let mut herb_ids: Vec<i64> = vec![];
    for herb in herbs.iter() {
        herb_ids.push(herb.id);
    }
    let herb_actions = get_herb_actions(&herb_ids, db_pool)
        .await
        .map_err(|e| Error::new(StatusCode::NotFound, e))?;
    for herb in herbs.iter() {
        let mut this_herb_herb_actions: Vec<db_results::HerbAction> = vec![];
        for ha in herb_actions.iter() {
            if ha.herb_id == herb.id {
                this_herb_herb_actions.push(ha.clone());
            }
        }
        results.push(convert_db_to_api(herb, &this_herb_herb_actions));
    }
    let api_results = HerbsApiResult {
        data: Some(results),
        error: None,
    };
    Ok(api_results)
}

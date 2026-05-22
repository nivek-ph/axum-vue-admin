use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SysDictionary {
    #[serde(rename = "ID", default)]
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub dict_type: String,
    pub status: Option<bool>,
    pub desc: String,
    #[serde(rename = "parentID")]
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDictionaryDetail {
    #[serde(rename = "ID", default)]
    pub id: i64,
    pub label: String,
    pub value: String,
    pub extend: String,
    pub status: Option<bool>,
    pub sort: i32,
    #[serde(rename = "sysDictionaryID")]
    pub sys_dictionary_id: i64,
    #[serde(rename = "parentID")]
    pub parent_id: Option<i64>,
    pub level: i32,
    pub path: String,
    pub children: Vec<SysDictionaryDetail>,
}

#[derive(Debug, Clone, FromRow)]
struct SysDictionaryDetailRow {
    pub id: i64,
    pub label: String,
    pub value: String,
    pub extend: String,
    pub status: Option<bool>,
    pub sort: i32,
    pub sys_dictionary_id: i64,
    pub parent_id: Option<i64>,
    pub level: i32,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdRequest {
    #[serde(rename = "ID", alias = "id")]
    pub id: Option<i64>,
    #[serde(rename = "type")]
    pub dict_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DictionaryListQuery {
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DictionaryTreeQuery {
    #[serde(rename = "sysDictionaryID")]
    pub sys_dictionary_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DictionaryTypeQuery {
    #[serde(rename = "type")]
    pub dict_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DictionaryParentQuery {
    #[serde(rename = "parentID")]
    pub parent_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImportDictionaryPayload {
    pub json: String,
}

pub async fn list(
    pool: &sqlx::PgPool,
    query: DictionaryListQuery,
) -> Result<Vec<SysDictionary>, sqlx::Error> {
    let list = sqlx::query_as::<_, SysDictionary>(
        r#"
        select id, name, type as dict_type, status, "desc", parent_id
        from sys_dictionaries
        where ($1::text is null or name ilike '%' || $1 || '%')
        order by id desc
        "#,
    )
    .bind(query.name.as_deref())
    .fetch_all(pool)
    .await?;

    Ok(list)
}

pub async fn create(pool: &sqlx::PgPool, payload: SysDictionary) -> Result<(), sqlx::Error> {
    sqlx::query(
        "insert into sys_dictionaries (name, type, status, \"desc\", parent_id) values ($1, $2, $3, $4, $5)",
    )
    .bind(payload.name)
    .bind(payload.dict_type)
    .bind(payload.status)
    .bind(payload.desc)
    .bind(payload.parent_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update(pool: &sqlx::PgPool, payload: SysDictionary) -> Result<(), sqlx::Error> {
    sqlx::query(
        "update sys_dictionaries set name = $1, type = $2, status = $3, \"desc\" = $4, parent_id = $5 where id = $6",
    )
    .bind(payload.name)
    .bind(payload.dict_type)
    .bind(payload.status)
    .bind(payload.desc)
    .bind(payload.parent_id)
    .bind(payload.id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find(pool: &sqlx::PgPool, id: i64) -> Result<Option<SysDictionary>, sqlx::Error> {
    sqlx::query_as::<_, SysDictionary>(
        "select id, name, type as dict_type, status, \"desc\", parent_id from sys_dictionaries where id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_query(
    pool: &sqlx::PgPool,
    id: Option<i64>,
    dict_type: Option<String>,
) -> Result<Option<serde_json::Value>, sqlx::Error> {
    let dictionary = if let Some(id) = id {
        find(pool, id).await?
    } else if let Some(dict_type) = dict_type {
        find_by_type(pool, &dict_type).await?
    } else {
        None
    };

    if let Some(dictionary) = dictionary {
        let details = tree_by_dictionary(pool, dictionary.id).await?;
        return Ok(Some(serde_json::json!({
            "ID": dictionary.id,
            "name": dictionary.name,
            "type": dictionary.dict_type,
            "status": dictionary.status,
            "desc": dictionary.desc,
            "parentID": dictionary.parent_id,
            "sysDictionaryDetails": flatten_details(&details),
        })));
    }

    Ok(None)
}

pub async fn delete(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_dictionary_details where sys_dictionary_id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    sqlx::query("delete from sys_dictionaries where id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn find_by_type(
    pool: &sqlx::PgPool,
    dict_type: &str,
) -> Result<Option<SysDictionary>, sqlx::Error> {
    sqlx::query_as::<_, SysDictionary>(
        "select id, name, type as dict_type, status, \"desc\", parent_id from sys_dictionaries where type = $1",
    )
    .bind(dict_type)
    .fetch_optional(pool)
    .await
}

pub async fn create_detail(
    pool: &sqlx::PgPool,
    payload: SysDictionaryDetail,
) -> Result<(), sqlx::Error> {
    let (level, path) =
        detail_level_and_path(pool, payload.parent_id, payload.sys_dictionary_id).await?;
    sqlx::query(
        r#"
        insert into sys_dictionary_details
        (label, value, extend, status, sort, sys_dictionary_id, parent_id, level, path)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(payload.label)
    .bind(payload.value)
    .bind(payload.extend)
    .bind(payload.status)
    .bind(payload.sort)
    .bind(payload.sys_dictionary_id)
    .bind(payload.parent_id)
    .bind(level)
    .bind(path)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_detail(
    pool: &sqlx::PgPool,
    payload: SysDictionaryDetail,
) -> Result<(), sqlx::Error> {
    let (level, path) =
        detail_level_and_path(pool, payload.parent_id, payload.sys_dictionary_id).await?;
    sqlx::query(
        r#"
        update sys_dictionary_details
        set label = $1, value = $2, extend = $3, status = $4, sort = $5,
            sys_dictionary_id = $6, parent_id = $7, level = $8, path = $9
        where id = $10
        "#,
    )
    .bind(payload.label)
    .bind(payload.value)
    .bind(payload.extend)
    .bind(payload.status)
    .bind(payload.sort)
    .bind(payload.sys_dictionary_id)
    .bind(payload.parent_id)
    .bind(level)
    .bind(path)
    .bind(payload.id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_detail(
    pool: &sqlx::PgPool,
    id: i64,
) -> Result<Option<SysDictionaryDetail>, sqlx::Error> {
    sqlx::query_as::<_, SysDictionaryDetailRow>(
        r#"
        select id, label, value, extend, status, sort, sys_dictionary_id, parent_id, level, path
        from sys_dictionary_details where id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|opt| opt.map(detail_from_row))
}

pub async fn delete_detail(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_dictionary_details where id = $1 or parent_id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn tree_by_dictionary(
    pool: &sqlx::PgPool,
    sys_dictionary_id: i64,
) -> Result<Vec<SysDictionaryDetail>, sqlx::Error> {
    let rows = sqlx::query_as::<_, SysDictionaryDetailRow>(
        r#"
        select id, label, value, extend, status, sort, sys_dictionary_id, parent_id, level, path
        from sys_dictionary_details
        where sys_dictionary_id = $1
        order by sort asc, id asc
        "#,
    )
    .bind(sys_dictionary_id)
    .fetch_all(pool)
    .await?;

    Ok(build_detail_tree(&rows, None))
}

pub async fn tree_by_type(
    pool: &sqlx::PgPool,
    dict_type: &str,
) -> Result<Vec<SysDictionaryDetail>, sqlx::Error> {
    if let Some(dictionary) = find_by_type(pool, dict_type).await? {
        return tree_by_dictionary(pool, dictionary.id).await;
    }
    Ok(Vec::new())
}

pub async fn details_by_parent(
    pool: &sqlx::PgPool,
    parent_id: i64,
) -> Result<Vec<SysDictionaryDetail>, sqlx::Error> {
    let rows = sqlx::query_as::<_, SysDictionaryDetailRow>(
        r#"
        select id, label, value, extend, status, sort, sys_dictionary_id, parent_id, level, path
        from sys_dictionary_details
        where parent_id = $1
        order by sort asc, id asc
        "#,
    )
    .bind(parent_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(detail_from_row).collect())
}

pub async fn detail_path(
    pool: &sqlx::PgPool,
    id: i64,
) -> Result<Vec<SysDictionaryDetail>, sqlx::Error> {
    let Some(item) = find_detail(pool, id).await? else {
        return Ok(Vec::new());
    };
    if item.path.is_empty() {
        return Ok(vec![item]);
    }
    let mut ids = item
        .path
        .split(',')
        .filter_map(|part| part.parse::<i64>().ok())
        .collect::<Vec<_>>();
    ids.push(item.id);
    let rows = sqlx::query_as::<_, SysDictionaryDetailRow>(
        r#"
        select id, label, value, extend, status, sort, sys_dictionary_id, parent_id, level, path
        from sys_dictionary_details
        where id = any($1)
        order by level asc, id asc
        "#,
    )
    .bind(&ids)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(detail_from_row).collect())
}

pub async fn export_dictionary(
    pool: &sqlx::PgPool,
    id: i64,
) -> Result<Option<serde_json::Value>, sqlx::Error> {
    let Some(dictionary) = find(pool, id).await? else {
        return Ok(None);
    };
    let details = tree_by_dictionary(pool, id).await?;
    Ok(Some(serde_json::json!({
        "dictionary": dictionary,
        "details": details
    })))
}

pub async fn import_dictionary(
    pool: &sqlx::PgPool,
    payload: ImportDictionaryPayload,
) -> Result<(), sqlx::Error> {
    let value: serde_json::Value = serde_json::from_str(&payload.json)
        .map_err(|err| sqlx::Error::Protocol(err.to_string().into()))?;
    let dictionary = serde_json::from_value::<SysDictionary>(value["dictionary"].clone())
        .map_err(|err| sqlx::Error::Protocol(err.to_string().into()))?;
    create(
        pool,
        SysDictionary {
            id: 0,
            ..dictionary
        },
    )
    .await?;
    Ok(())
}

async fn detail_level_and_path(
    pool: &sqlx::PgPool,
    parent_id: Option<i64>,
    _sys_dictionary_id: i64,
) -> Result<(i32, String), sqlx::Error> {
    if let Some(parent_id) = parent_id {
        let parent: Option<(i32, String)> =
            sqlx::query_as("select level, path from sys_dictionary_details where id = $1")
                .bind(parent_id)
                .fetch_optional(pool)
                .await?;
        if let Some((level, path)) = parent {
            return Ok((
                level + 1,
                if path.is_empty() {
                    parent_id.to_string()
                } else {
                    format!("{path},{parent_id}")
                },
            ));
        }
    }
    Ok((0, String::new()))
}

fn build_detail_tree(
    rows: &[SysDictionaryDetailRow],
    parent_id: Option<i64>,
) -> Vec<SysDictionaryDetail> {
    let mut list = rows
        .iter()
        .filter(|row| row.parent_id == parent_id)
        .map(|row| {
            let mut item = detail_from_row(row.clone());
            item.children = build_detail_tree(rows, Some(row.id));
            item
        })
        .collect::<Vec<_>>();
    list.sort_by_key(|item| (item.sort, item.id));
    list
}

fn detail_from_row(row: SysDictionaryDetailRow) -> SysDictionaryDetail {
    SysDictionaryDetail {
        id: row.id,
        label: row.label,
        value: row.value,
        extend: row.extend,
        status: row.status,
        sort: row.sort,
        sys_dictionary_id: row.sys_dictionary_id,
        parent_id: row.parent_id,
        level: row.level,
        path: row.path,
        children: Vec::new(),
    }
}

fn flatten_details(details: &[SysDictionaryDetail]) -> Vec<serde_json::Value> {
    let mut result = Vec::new();
    for detail in details {
        result.push(serde_json::json!({
            "ID": detail.id,
            "label": detail.label,
            "value": detail.value,
            "extend": detail.extend,
            "status": detail.status,
            "sort": detail.sort,
            "sysDictionaryID": detail.sys_dictionary_id,
            "parentID": detail.parent_id,
            "level": detail.level,
            "path": detail.path,
        }));
        result.extend(flatten_details(&detail.children));
    }
    result
}

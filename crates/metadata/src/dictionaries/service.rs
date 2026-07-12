use sqlx::PgPool;

use super::model::SysDictionaryDetailRow;
use super::{
    DictionaryError, DictionaryListQuery, DictionaryWithDetails, ImportDictionaryPayload,
    SysDictionary, SysDictionaryDetail,
};

#[derive(Clone)]
pub struct DictionaryService {
    pool: PgPool,
}

impl DictionaryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn list(
        &self,
        query: DictionaryListQuery,
    ) -> Result<Vec<SysDictionary>, DictionaryError> {
        Ok(list(&self.pool, query).await?)
    }
    pub async fn create(&self, payload: SysDictionary) -> Result<(), DictionaryError> {
        Ok(create(&self.pool, payload).await?)
    }
    pub async fn update(&self, payload: SysDictionary) -> Result<(), DictionaryError> {
        Ok(update(&self.pool, payload).await?)
    }
    pub async fn find(
        &self,
        id: Option<i64>,
        kind: Option<String>,
    ) -> Result<Option<DictionaryWithDetails>, DictionaryError> {
        Ok(find_by_query(&self.pool, id, kind).await?)
    }
    pub async fn delete(&self, id: i64) -> Result<(), DictionaryError> {
        Ok(delete(&self.pool, id).await?)
    }
    pub async fn export(&self, id: i64) -> Result<Option<serde_json::Value>, DictionaryError> {
        Ok(export_dictionary(&self.pool, id).await?)
    }
    pub async fn import(&self, payload: ImportDictionaryPayload) -> Result<(), DictionaryError> {
        Ok(import_dictionary(&self.pool, payload).await?)
    }
    pub async fn create_detail(&self, payload: SysDictionaryDetail) -> Result<(), DictionaryError> {
        Ok(create_detail(&self.pool, payload).await?)
    }
    pub async fn update_detail(&self, payload: SysDictionaryDetail) -> Result<(), DictionaryError> {
        Ok(update_detail(&self.pool, payload).await?)
    }
    pub async fn find_detail(
        &self,
        id: i64,
    ) -> Result<Option<SysDictionaryDetail>, DictionaryError> {
        Ok(find_detail(&self.pool, id).await?)
    }
    pub async fn delete_detail(&self, id: i64) -> Result<(), DictionaryError> {
        Ok(delete_detail(&self.pool, id).await?)
    }
    pub async fn tree_by_dictionary(
        &self,
        id: i64,
    ) -> Result<Vec<SysDictionaryDetail>, DictionaryError> {
        Ok(tree_by_dictionary(&self.pool, id).await?)
    }
    pub async fn tree_by_type(
        &self,
        kind: &str,
    ) -> Result<Vec<SysDictionaryDetail>, DictionaryError> {
        Ok(tree_by_type(&self.pool, kind).await?)
    }
    pub async fn details_by_parent(
        &self,
        id: i64,
    ) -> Result<Vec<SysDictionaryDetail>, DictionaryError> {
        Ok(details_by_parent(&self.pool, id).await?)
    }
    pub async fn detail_path(&self, id: i64) -> Result<Vec<SysDictionaryDetail>, DictionaryError> {
        Ok(detail_path(&self.pool, id).await?)
    }
}

pub(crate) async fn list(
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

pub(crate) async fn create(pool: &sqlx::PgPool, payload: SysDictionary) -> Result<(), sqlx::Error> {
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

pub(crate) async fn update(pool: &sqlx::PgPool, payload: SysDictionary) -> Result<(), sqlx::Error> {
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

pub(crate) async fn find(
    pool: &sqlx::PgPool,
    id: i64,
) -> Result<Option<SysDictionary>, sqlx::Error> {
    sqlx::query_as::<_, SysDictionary>(
        "select id, name, type as dict_type, status, \"desc\", parent_id from sys_dictionaries where id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub(crate) async fn find_by_query(
    pool: &sqlx::PgPool,
    id: Option<i64>,
    dict_type: Option<String>,
) -> Result<Option<DictionaryWithDetails>, sqlx::Error> {
    let dictionary = if let Some(id) = id {
        find(pool, id).await?
    } else if let Some(dict_type) = dict_type {
        find_by_type(pool, &dict_type).await?
    } else {
        None
    };

    if let Some(dictionary) = dictionary {
        let details = tree_by_dictionary(pool, dictionary.id).await?;
        return Ok(Some(DictionaryWithDetails {
            dictionary,
            details,
        }));
    }

    Ok(None)
}

pub(crate) async fn delete(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
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

pub(crate) async fn find_by_type(
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

pub(crate) async fn create_detail(
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

pub(crate) async fn update_detail(
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

pub(crate) async fn find_detail(
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

pub(crate) async fn delete_detail(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_dictionary_details where id = $1 or parent_id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn tree_by_dictionary(
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

pub(crate) async fn tree_by_type(
    pool: &sqlx::PgPool,
    dict_type: &str,
) -> Result<Vec<SysDictionaryDetail>, sqlx::Error> {
    if let Some(dictionary) = find_by_type(pool, dict_type).await? {
        return tree_by_dictionary(pool, dictionary.id).await;
    }
    Ok(Vec::new())
}

pub(crate) async fn details_by_parent(
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

pub(crate) async fn detail_path(
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

pub(crate) async fn export_dictionary(
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

pub(crate) async fn import_dictionary(
    pool: &sqlx::PgPool,
    payload: ImportDictionaryPayload,
) -> Result<(), sqlx::Error> {
    let value: serde_json::Value = serde_json::from_str(&payload.json)
        .map_err(|err| sqlx::Error::Protocol(err.to_string()))?;
    let dictionary = serde_json::from_value::<SysDictionary>(value["dictionary"].clone())
        .map_err(|err| sqlx::Error::Protocol(err.to_string()))?;
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

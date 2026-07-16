use metadata::dictionaries::{
    DictionaryDetailInput, DictionaryError, DictionaryInput, DictionaryListQuery, DictionaryService,
};
use sqlx::PgPool;

fn dictionary(name: &str, kind: &str) -> DictionaryInput {
    DictionaryInput {
        name: name.to_owned(),
        dict_type: kind.to_owned(),
        status: Some(true),
        desc: String::new(),
        parent_id: None,
    }
}

fn detail(label: &str, parent_id: Option<i64>) -> DictionaryDetailInput {
    DictionaryDetailInput {
        label: label.to_owned(),
        value: label.to_lowercase(),
        extend: String::new(),
        status: Some(true),
        sort: 0,
        parent_id,
    }
}

async fn dictionary_id(service: &DictionaryService, name: &str) -> i64 {
    service
        .list(DictionaryListQuery {
            page: None,
            page_size: None,
            name: Some(name.to_owned()),
        })
        .await
        .expect("dictionary list should succeed")
        .into_iter()
        .find(|item| item.name == name)
        .expect("dictionary should exist")
        .id
}

async fn detail_id(service: &DictionaryService, dictionary_id: i64, label: &str) -> i64 {
    service
        .tree_by_dictionary(dictionary_id)
        .await
        .expect("dictionary tree should load")
        .into_iter()
        .find(|item| item.label == label)
        .expect("detail should exist")
        .id
}

#[sqlx::test(migrations = "../../migrations")]
async fn detail_operations_are_scoped_to_the_dictionary(pool: PgPool) {
    let service = DictionaryService::new(pool);
    service
        .create(dictionary("Scope A", "scope_a"))
        .await
        .unwrap();
    service
        .create(dictionary("Scope B", "scope_b"))
        .await
        .unwrap();
    let dictionary_a = dictionary_id(&service, "Scope A").await;
    let dictionary_b = dictionary_id(&service, "Scope B").await;

    service
        .create_detail(dictionary_b, detail("Only B", None))
        .await
        .unwrap();
    let detail_b = detail_id(&service, dictionary_b, "Only B").await;

    assert!(matches!(
        service.find_detail(dictionary_a, detail_b).await,
        Err(DictionaryError::DetailNotFound { .. })
    ));
    assert!(matches!(
        service
            .update_detail(dictionary_a, detail_b, detail("Moved", None))
            .await,
        Err(DictionaryError::DetailNotFound { .. })
    ));
    assert!(matches!(
        service.delete_detail(dictionary_a, detail_b).await,
        Err(DictionaryError::DetailNotFound { .. })
    ));
    assert!(matches!(
        service
            .create_detail(dictionary_a, detail("Wrong parent", Some(detail_b)))
            .await,
        Err(DictionaryError::DetailNotFound { .. })
    ));
    assert!(matches!(
        service
            .create_detail(i64::MAX, detail("Orphan", None))
            .await,
        Err(DictionaryError::DictionaryNotFound { .. })
    ));

    let item = service.find_detail(dictionary_b, detail_b).await.unwrap();
    assert_eq!(item.label, "Only B");
}

#[sqlx::test(migrations = "../../migrations")]
async fn deleting_a_node_deletes_its_entire_subtree(pool: PgPool) {
    let service = DictionaryService::new(pool);
    service
        .create(dictionary("Delete Tree", "delete_tree"))
        .await
        .unwrap();
    let dictionary_id = dictionary_id(&service, "Delete Tree").await;

    service
        .create_detail(dictionary_id, detail("Root", None))
        .await
        .unwrap();
    let root_id = detail_id(&service, dictionary_id, "Root").await;
    service
        .create_detail(dictionary_id, detail("Child", Some(root_id)))
        .await
        .unwrap();
    let child_id = service
        .details_by_parent(dictionary_id, root_id)
        .await
        .unwrap()[0]
        .id;
    service
        .create_detail(dictionary_id, detail("Grandchild", Some(child_id)))
        .await
        .unwrap();

    service.delete_detail(dictionary_id, root_id).await.unwrap();

    assert!(
        service
            .tree_by_dictionary(dictionary_id)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        service
            .details_by_parent(dictionary_id, child_id)
            .await
            .unwrap()
            .is_empty()
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn moving_a_node_updates_descendants_and_rejects_cycles(pool: PgPool) {
    let service = DictionaryService::new(pool.clone());
    service
        .create(dictionary("Move Tree", "move_tree"))
        .await
        .unwrap();
    let dictionary_id = dictionary_id(&service, "Move Tree").await;
    service
        .create_detail(dictionary_id, detail("Root A", None))
        .await
        .unwrap();
    service
        .create_detail(dictionary_id, detail("Root B", None))
        .await
        .unwrap();
    let root_a = detail_id(&service, dictionary_id, "Root A").await;
    let root_b = detail_id(&service, dictionary_id, "Root B").await;
    service
        .create_detail(dictionary_id, detail("Child", Some(root_a)))
        .await
        .unwrap();
    let child: i64 = sqlx::query_scalar(
        "select id from sys_dictionary_details where sys_dictionary_id = $1 and label = 'Child'",
    )
    .bind(dictionary_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    service
        .create_detail(dictionary_id, detail("Grandchild", Some(child)))
        .await
        .unwrap();
    let grandchild: i64 = sqlx::query_scalar(
        "select id from sys_dictionary_details where sys_dictionary_id = $1 and label = 'Grandchild'",
    )
    .bind(dictionary_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let error = service
        .update_detail(dictionary_id, root_a, detail("Root A", Some(grandchild)))
        .await
        .expect_err("a node cannot move below its descendant");
    assert!(matches!(error, DictionaryError::InvalidParent { .. }));

    service
        .update_detail(dictionary_id, child, detail("Child", Some(root_b)))
        .await
        .unwrap();
    let child_state: (i32, String) =
        sqlx::query_as("select level, path from sys_dictionary_details where id = $1")
            .bind(child)
            .fetch_one(&pool)
            .await
            .unwrap();
    let grandchild_state: (i32, String) =
        sqlx::query_as("select level, path from sys_dictionary_details where id = $1")
            .bind(grandchild)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(child_state, (1, root_b.to_string()));
    assert_eq!(grandchild_state, (2, format!("{root_b},{child}")));
}

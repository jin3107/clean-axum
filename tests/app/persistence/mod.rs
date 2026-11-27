use sea_orm::DbErr;
use utils::testing::setup_test_db;

use app::persistence::blog::create_blog;
use app::persistence::tenant::create_tenant_with_admin;
use app::persistence::user::get_user;
use app::tenant::TenantId;
use models::params::blog::CreateBlogParams;

#[tokio::test]
async fn database_enforces_tenant_boundaries() {
    let db = setup_test_db().await.expect("Set up db failed!");
    let register =
        |name: String| create_tenant_with_admin(&db, name, "admin".to_string(), "hash".to_string());
    let a = register(format!("a-{}", uuid::Uuid::new_v4()))
        .await
        .expect("Create tenant a failed!");
    let b = register(format!("b-{}", uuid::Uuid::new_v4()))
        .await
        .expect("Create tenant b failed!");

    let other = get_user(&db, TenantId(b.tenant_id), a.id).await.unwrap();
    assert!(other.is_none());

    let params = || CreateBlogParams {
        title: "title".to_string(),
        content: "test".to_string(),
    };
    create_blog(&db, TenantId(a.tenant_id), a.id, params())
        .await
        .expect("Create blog failed!");

    // the composite foreign key rejects an author from another tenant
    let crossed: Result<_, DbErr> = create_blog(&db, TenantId(b.tenant_id), a.id, params()).await;
    assert!(crossed.is_err());
}

pub use rbatis::RBatis;
use rbatis::rbdc::{DateTime, Error};
use rbatis::rbdc::db::ExecResult;
use tracing::info;
use models::base::User;
use rbatis::rbatis_codegen::IntoSql;

pub async fn create_user(rb: &RBatis) -> ExecResult {
    let usr = User {
        id: None,
        name: Some("rust".into()),
        username: Some("rust".into()),
        password: None,
        status: Some("1".into()),
        age: Some(20),
        gender: Some(1),
        phone: Some("15053150503".into()),
        email: Some("voncho@qq.com".into()),
        creator: Some(1),
        created: Some(DateTime::now()),
        creator_name: Some("admin".into()),
        modifier: None,
        modifier_name: None,
        modified: None
    };
    info!("service user mod");
    let usr = User::insert(rb, &usr).await.unwrap();
    usr
}
#[py_sql(
"`select * from user where 1 = 1`
                  if !ids.is_empty():
                    ` and id in `
                    ${ids.sql()}"
)]
pub async fn list_user_by_ids(rb: &RBatis, ids: Vec<usize>) -> Result<Vec<User>, Error> {
    impled!()
}
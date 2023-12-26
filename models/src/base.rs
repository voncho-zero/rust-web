use rbatis::impl_select;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use crate::Tree;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub status: Option<String>,
    pub age: Option<u8>,
    pub gender: Option<u8>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub creator: Option<u32>,
    pub creator_name: Option<String>,
    pub created: Option<DateTime>,
    pub modifier: Option<u32>,
    pub modifier_name: Option<String>,
    pub modified: Option<DateTime>,
}
//crud = async fn insert(...)+async fn  select_by_column(...)+ async fn  update_by_column(...)+async fn  delete_by_column(...)...and more
rbatis::crud!(User {});
impl_select!(User{select_by_id(id:usize) -> Option => "`where id = #{id} limit 1`"});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Menu {
    pub id: Option<usize>,
    pub parent_id: Option<usize>,
    pub order_no: Option<u8>,
    pub is_leaf: Option<u8>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub title: Option<String>,
}

impl Tree for Menu {
    fn get_id(&self) -> Option<usize> {
        self.id
    }

    fn get_parent_id(&self) -> Option<usize> {
        self.parent_id
    }
}
rbatis::crud!(Menu {});
impl_select!(Menu{select_by_id(id:usize) -> Option => "`where id = #{id} limit 1`"});
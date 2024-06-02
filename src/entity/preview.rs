use rbatis::{crud, impl_delete, impl_select, impl_select_page};
use rbatis::rbatis_codegen::IntoSql;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Preview {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub time: Option<String>,
    pub author: Option<u64>,
}

crud!(Preview {});
impl_select!(Preview{select_all_by_userid(author:&str) => "`where author = #{author} order by id desc`"});
impl_select!(Preview{select_by_method(ids:&[&str]) -> Vec => "`where id in ${ids.sql()}`"});
impl_delete!(Preview{delete_by_id(id:&str) => "`where id = #{id}`"});
impl_select_page!(Preview{select_page_by_userid(author:&str) => "`where author = #{author} order by id desc`"});
impl_select_page!(Preview{select_page() => "order by id desc"});

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct ResponseFavoritePreview {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub time: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct ResponsePreview {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub time: String,
    pub userid: String,
    pub username: String,
    pub avatar: String,
}

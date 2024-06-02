use rbatis::{crud, impl_delete, impl_select};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Article {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub time: Option<String>,
    pub author: Option<u64>,
}

crud!(Article {});
impl_select!(Article{select_by_id(id:&str) -> Option => "`where id = #{id} limit 1`"});
impl_delete!(Article{delete_by_id(id:&str) => "`where id = #{id}`"});

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestArticle {
    pub title: String,
    pub content: String,
    pub time: String,
    pub author: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestOwnArticle {
    pub userid: String,
    pub email: String,
    pub token: String,
    pub page: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestOtherArticle {
    pub userid: String,
    pub page: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestAllArticle {
    pub page: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestDeleteOneArticle {
    pub email: String,
    pub token: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestOneArticle {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct ResponseArticle {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub time: String,
    pub userid: u64,
    pub username: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct ResponseArticles {
    pub num: u64,
    pub articles: Vec<ResponseArticle>,
}


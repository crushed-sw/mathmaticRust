use rbatis::{crud, impl_delete, impl_select, impl_select_page};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Comment {
    pub id: Option<u64>,
    pub userid: Option<u64>,
    pub articleid: Option<u64>,
    pub content: Option<String>,
    pub time: Option<String>,
}

crud!(Comment {});
impl_delete!(Comment{delete_one(userid:&str, articleid:&str) => "`where userid = #{userid} and articleid = #{articleid}`"});
impl_delete!(Comment{delete_by_article(articleid:&str) => "`where articleid = #{articleid}`"});
impl_select!(Comment{select_all_by_articleid(articleid:&str) => "`where articleid = #{articleid}`"});
impl_select_page!(Comment{select_page_by_articleid(articleid:&str) => "`where articleid = #{articleid} order by id desc`"});

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestCommentPage {
    pub articleid: String,
    pub page: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestAddComment {
    pub userid: String,
    pub articleid: String,
    pub content: String,
    pub time: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestDeleteComment {
    pub userid: String,
    pub articleid: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct ResponseComment {
    pub userid: u64,
    pub username: String,
    pub avatar: String,
    pub content: String,
    pub time: String,
}


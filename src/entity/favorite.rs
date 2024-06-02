use rbatis::{crud, impl_delete, impl_select, impl_select_page};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Favorite {
    pub id: Option<u64>,
    pub userid: Option<u64>,
    pub articleid: Option<u64>,
}

crud!(Favorite {});
impl_select!(Favorite{select_one(userid:&str, articleid:&str) -> Option => "`where userid = #{userid} and articleid = #{articleid}`"});
impl_select!(Favorite{select_all_by_userid(userid:&str) => "`where userid = #{userid}`"});
impl_delete!(Favorite{delete_one(userid:&str, articleid:&str) => "`where userid = #{userid} and articleid = #{articleid}`"});
impl_delete!(Favorite{delete_by_article(articleid:&str) => "`where articleid = #{articleid}`"});
impl_select_page!(Favorite{select_page_by_userid(userid:&str) => "`where userid = #{userid} order by id desc`"});

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestFavoriteAll {
    pub userid: String,
    pub articleid: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestFavoriteUser {
    pub userid: String,
    pub email: String,
    pub token: String,
    pub page: String,
}


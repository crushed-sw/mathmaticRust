use rbatis::{crud, impl_select, impl_update};
use rbatis::rbatis_codegen::IntoSql;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct User {
    pub userid: Option<u64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub password: Option<String>,
}

crud!(User {});

impl_select!(User{select_all_by_email(email:&str) => "`where email = #{email}`"});
impl_select!(User{select_by_email(email:&str) -> Option => "`where email = #{email} limit 1`"});
impl_select!(User{select_by_id(userid:&str) -> Option => "`where userid = #{userid} limit 1`"});
impl_select!(User{select_by_method(ids:&[&str]) -> Vec => "`where userid in ${ids.sql()}`"});
impl_update!(User{update_by_id(userid:&str) => "`where userid = #{userid}`"});

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestLoginUser {
    pub email: String,
    pub password: String,
    pub time: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestRegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestSendCodeUser {
    pub email: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestUserInform {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestCheckUser {
    pub userid: String,
    pub username: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestChangeUsername {
    pub userid: String,
    pub username: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RequestResetPassword {
    pub email: String,
    pub password: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct ResponseUser {
    pub userid: String,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub token: Option<String>,
}

impl RequestRegisterUser {
    pub fn to_user(&self, avatar: Option<String>) -> User {
        User {
            userid: None,
            username: Some(self.username.clone()),
            email: Some(self.email.clone()),
            avatar,
            password: Some(self.password.clone()),
        }
    }
}

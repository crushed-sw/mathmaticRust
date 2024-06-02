use crate::pool;
use crate::entity::user::User;

use rbatis::Error;

pub struct UserService;

impl UserService {
    pub fn check(user: &User) -> Result<(), Error> {
        if user.email.is_none() || user.email.as_deref().unwrap_or_default().is_empty() ||
           user.username.is_none() || user.username.as_deref().unwrap_or_default().is_empty() ||
           user.password.is_none() || user.password.as_deref().unwrap_or_default().is_empty()
        {
            return Err(Error::from("用户信息不能为空"));
        }
        Ok(())
    }

    pub async fn find_by_email(email: &String) -> Result<Option<User>, Error> {
        User::select_by_email(pool!(), email.as_str()).await
    }

    pub async fn find_by_id(id: &String) -> Result<Option<User>, Error> {
        User::select_by_id(pool!(), id.as_str()).await
    }

    pub async fn update_username_by_id(new_name: &String, id: &String) -> Result<(), Error> {
        let user_opt = Self::find_by_id(&id).await?;
        match user_opt {
            None => Err(Error::from("查询失败")),
            Some(user) => {
                let new_user = User {
                    username: Some(new_name.clone()),
                    ..user
                };
                User::update_by_id(pool!(), &new_user, id.as_str()).await?;
                Ok(())
            }
        }
    }

    pub async fn update_password_by_email(password: &String, email: &String) -> Result<(), Error> {
        let user_opt = Self::find_by_email(email).await?;
        match user_opt {
            None => Err(Error::from("查询失败")),
            Some(user) => {
                let new_user = User {
                    password: Some(password.clone()),
                    ..user
                };
                User::update_by_id(pool!(), &new_user, user.userid.unwrap().to_string().as_str()).await?;
                Ok(())
            }
        }
    }

    pub async fn insert(user: &User) -> Result<u64, Error> {
        let email = user.email.as_deref().unwrap_or_default().to_string();
        let username = user.username.as_deref().unwrap_or_default().to_string();
        let password = user.password.as_deref().unwrap_or_default().to_string();
        let avatar = user.avatar.clone();

        let old_user = Self::find_by_email(&email).await?;
        if old_user.is_some() {
            return Err(Error::from(format!("邮箱{}已有账号", email)));
        }

        let new_user = User {
            userid: None,
            username: Some(username),
            email: Some(email),
            avatar,
            password: Some(password),
        };

        Ok(User::insert(pool!(), &new_user).await?.rows_affected)
    }
}

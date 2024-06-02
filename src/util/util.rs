use crate::entity::result;
use crate::entity::result_code::ResultCode;
use crate::service::CONTEXT;

pub struct Util;

impl Util {
    pub fn get_avatar(email: &str) -> Option<String> {
        if email.ends_with("@qq.com") {
            let v: Vec<&str> = email.split('@').collect();
            if v.len() == 0 {
                None
            } else {
                Some(format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=100", v[0]))
            }
        } else {
            None
        }
    }

    pub async fn check_token(email: &str, token: &str) -> result::Result {
        let new_token = CONTEXT.redis_service.get_string((email.to_string() + "-token").as_str()).await;
        match new_token {
            Err(_) => result::Result::error(20041, ResultCode::get_message(20041)),
            Ok(token_value) => {
                if token_value == token {
                    result::Result::success(20040, ResultCode::get_message(20040), String::from(""))
                } else {
                    result::Result::error(20041, ResultCode::get_message(20041))
                }
            },
        }
    }
}

use actix_files as fs;
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use actix_web::http::header::{ContentDisposition, DispositionType};
use crate::{
    entity::{
        result,
        result_code::ResultCode,
        user::{
            RequestChangeUsername,
            RequestResetPassword,
            RequestCheckUser,
            RequestLoginUser,
            RequestRegisterUser,
            RequestSendCodeUser,
            ResponseUser,
            RequestUserInform,
        },
    },
    service::{
        lettre_service::LettreService,
        user_service::UserService,
        CONTEXT
    },
    util::{
        util::Util,
        jwt::JwtToken,
    }
};

#[get("/login")]
pub async fn login(user: web::Query<RequestLoginUser>) -> impl Responder {
    let email = user.email.clone();
    let get_user_result = UserService::find_by_email(&email).await;

    let result = match get_user_result {
        Err(_) => result::Result::error(20051, ResultCode::get_message(20051)),
        Ok(get_user) => {
            match get_user {
                None => result::Result::error(20021, ResultCode::get_message(20021)),
                Some(new_user) => {
                    let new_password = new_user.password.unwrap();
                    if new_password != user.password {
                        result::Result::error(20021, ResultCode::get_message(20021))
                    } else {
                        let userid = new_user.userid.unwrap().to_string();
                        let username = new_user.username.unwrap();
                        let avatar = new_user.avatar;
                        let current_time = user.time.clone();

                        let jwt_token = JwtToken {
                            userid: userid.clone(),
                            username: username.clone(),
                            current_time: current_time.clone(),
                        };
                        let token = jwt_token.create_token(CONTEXT.config.jwt.secret.as_ref().unwrap().as_str()).unwrap();
                        let _ = CONTEXT.redis_service.set_string_ex((email.clone() + "-token").as_str(), token.as_str(), 86400).await;
                        let resp = ResponseUser { userid, username, email, avatar, token: Some(token) };
                        result::Result::success(20020, ResultCode::get_message(20020), resp)
                    }
                }
            }
        }
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[post("/register")]
pub async fn register(user: web::Json<RequestRegisterUser>) -> impl Responder {
    let code = user.code.as_str();

    let avatar = Util::get_avatar(user.email.as_str());
    let user = user.to_user(avatar);
    let checked = UserService::check(&user);

    let result = match checked {
        Err(_) => result::Result::error(20001, ResultCode::get_message(20001)),
        Ok(_) => {
            let code_cache = CONTEXT.redis_service.get_string(user.email.as_ref().unwrap().as_str()).await;
            match code_cache {
                Err(_) => result::Result::error(20012, ResultCode::get_message(20012)),
                Ok(code_value) => {
                    if code_value == code {
                        let res = UserService::insert(&user).await;
                        match res {
                            Err(rbatis::Error::E(err)) => {
                                if err.ends_with("已有账号") {
                                    result::Result::error(20031, ResultCode::get_message(20031))
                                } else {
                                    result::Result::error(20051, ResultCode::get_message(20051))
                                }
                            },
                            Ok(userid) => {
                                let resp = ResponseUser {
                                    userid: userid.to_string(),
                                    username: user.username.as_ref().unwrap().clone(),
                                    email: user.email.as_ref().unwrap().clone(),
                                    avatar: None,
                                    token: None,
                                };
                                let _ = CONTEXT.redis_service.del_string(user.email.as_ref().unwrap().as_str()).await;
                                result::Result::success(20030, ResultCode::get_message(20030), resp)
                            },
                        }
                    } else {
                        result::Result::error(20012, ResultCode::get_message(20012))
                    }
                },
            }
        }
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/send_code")]
pub async fn send_code(user: web::Query<RequestSendCodeUser>) -> impl Responder {
    let send_result = LettreService::get_and_send_code(&user.email).await;
    let result = match send_result {
        Err(_) => result::Result::error(20011, ResultCode::get_message(20011)),
        Ok(code) => {
            let is_ok = CONTEXT.redis_service.set_string_ex(user.email.as_str(), code.as_str(), 1800).await;
            match is_ok {
                Err(_) => result::Result::error(20051, ResultCode::get_message(20051)),
                Ok(_) => result::Result::success(20010, ResultCode::get_message(20010), ""),
            }
        }
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/check")]
pub async fn check(user: web::Query<RequestCheckUser>) -> impl Responder {
    let result = Util::check_token(user.email.as_str(), user.token.as_str()).await;

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/change_username")]
pub async fn change_username(user: web::Query<RequestChangeUsername>) -> impl Responder {
    let check_result = Util::check_token(user.email.as_str(), user.token.as_str()).await;
    let result = if !check_result.success {
        check_result
    } else {
        let res = UserService::update_username_by_id(&user.username, &user.userid).await;
        match res {
            Ok(_) => result::Result::success(20060, ResultCode::get_message(20060), &user.username),
            Err(_) => result::Result::error(20061, ResultCode::get_message(20061)),
        }
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_inform")]
pub async fn get_inform(user: web::Query<RequestUserInform>) -> impl Responder {
    let res = UserService::find_by_id(&user.id).await;
    let result = match res {
        Err(_) => result::Result::error(20111, ResultCode::get_message(20111)),
        Ok(user_option) => {
            match user_option {
                None => result::Result::error(20111, ResultCode::get_message(20111)),
                Some(res_user) => {
                    let user = ResponseUser {
                        userid: res_user.userid.unwrap_or_default().to_string(),
                        username: res_user.username.unwrap_or_default(),
                        email: res_user.email.unwrap_or_default(),
                        avatar: Some(res_user.avatar.unwrap_or_default()),
                        token: None,
                    };
                    result::Result::success(20110, ResultCode::get_message(20110), user)
                }
            }
        },
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[post("/reset_password")]
pub async fn reset_password(user: web::Json<RequestResetPassword>) -> impl Responder {
    let is_ok = CONTEXT.redis_service.get_string(user.email.as_str()).await;

    let result = match is_ok {
        Err(_) => result::Result::error(20051, ResultCode::get_message(20051)),
        Ok(code_value) => {
            if code_value == user.code {
                let update_result = UserService::update_password_by_email(&user.password, &user.email).await;
                let _ = CONTEXT.redis_service.del_string(user.email.as_str()).await;
                match update_result {
                    Err(_) => result::Result::error(20081, ResultCode::get_message(20081)),
                    Ok(_) => result::Result::success(20080, ResultCode::get_message(20080), ""),
                }
            } else {
                result::Result::error(20012, ResultCode::get_message(20012))
            }
        }
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/avatar/{avatar_email}")]
pub async fn get_avatar(path: web::Path<String>) -> Result<fs::NamedFile, Error> {
    let avatar_email = path.into_inner();
    let path_buf: std::path::PathBuf = std::path::PathBuf::from(format!("static/{}", avatar_email));
    println!("{:?}", path_buf);
    let file = fs::NamedFile::open(path_buf)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

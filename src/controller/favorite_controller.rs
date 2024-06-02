use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    entity::{
        favorite::{RequestFavoriteAll, RequestFavoriteUser},
        result::Result,
        result_code::ResultCode
    },
    service::favorite_service::FavoriteService,
    util::util::Util
};

#[post("/insert_favorite")]
pub async fn insert_favorite(inform: web::Json<RequestFavoriteAll>) -> impl Responder {
    let check = Util::check_token(inform.email.as_str(), inform.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let insert_result = FavoriteService::insert(inform.userid.as_str(), inform.articleid.as_str()).await;
    let result = match insert_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(_) => Result::success(200, ResultCode::get_message(200), ""),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[post("/delete_favorite")]
pub async fn delete_favorite(inform: web::Json<RequestFavoriteAll>) -> impl Responder {
    let check = Util::check_token(inform.email.as_str(), inform.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let delete_result = FavoriteService::delete_one(inform.userid.as_str(), inform.articleid.as_str()).await;
    let result = match delete_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(_) => Result::success(200, ResultCode::get_message(200), ""),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/is_favorite")]
pub async fn is_favorite(inform: web::Query<RequestFavoriteAll>) -> impl Responder {
    let check = Util::check_token(inform.email.as_str(), inform.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let get_result = FavoriteService::select_one(inform.userid.as_str(), inform.articleid.as_str()).await;
    let result = match get_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(_) => Result::success(200, ResultCode::get_message(200), ""),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_favorite")]
pub async fn get_favorite(inform: web::Query<RequestFavoriteUser>) -> impl Responder {
    let check = Util::check_token(inform.email.as_str(), inform.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let get_result = FavoriteService::select_by_userid(inform.userid.as_str(), inform.page.parse::<u64>().unwrap_or_default()).await;
    let result = match get_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(value) => Result::success(200, ResultCode::get_message(200), value),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_num_favorite")]
pub async fn get_num_favorite(inform: web::Query<RequestFavoriteUser>) -> impl Responder {
    let check = Util::check_token(inform.email.as_str(), inform.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let get_result = FavoriteService::select_num_by_userid(inform.userid.as_str()).await;
    let result = match get_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(value) => Result::success(200, ResultCode::get_message(200), value),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

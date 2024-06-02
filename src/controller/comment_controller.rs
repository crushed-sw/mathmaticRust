use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    entity::{
        comment::{RequestCommentPage, RequestAddComment, RequestDeleteComment},
        result::Result,
        result_code::ResultCode
    },
    service::comment_service::CommentService,
    util::util::Util
};

#[post("/insert_comment")]
pub async fn insert_comment(inform: web::Json<RequestAddComment>) -> impl Responder {
    let check = Util::check_token(inform.email.as_str(), inform.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let insert_result = CommentService::insert(inform.userid.as_str(), inform.articleid.as_str(), inform.content.as_str(), inform.time.as_str()).await;
    let result = match insert_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(_) => Result::success(200, ResultCode::get_message(200), ""),
    };

    return HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string());
}

#[post("/delete_comment")]
pub async fn delete_comment(inform: web::Json<RequestDeleteComment>) -> impl Responder {
    let check = Util::check_token(inform.email.as_str(), inform.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let delete_result = CommentService::delete_one(inform.userid.as_str(), inform.articleid.as_str()).await;
    let result = match delete_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(_) => Result::success(200, ResultCode::get_message(200), ""),
    };

    return HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string());
}

#[get("/get_num_comment")]
pub async fn get_num_comment(inform: web::Query<RequestCommentPage>) -> impl Responder {
    let get_result = CommentService::get_num_by_articleid(inform.articleid.as_str()).await;
    let result = match get_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(value) => Result::success(200, ResultCode::get_message(200), value),
    };

    return HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string());
}

#[get("/get_comment")]
pub async fn get_comment(inform: web::Query<RequestCommentPage>) -> impl Responder {
    let get_result = CommentService::get_page_by_articleid(inform.articleid.as_str(), inform.page.parse::<u64>().unwrap_or_default()).await;
    let result = match get_result {
        Err(_) => Result::error(1, ResultCode::get_message(1)),
        Ok(value) => Result::success(200, ResultCode::get_message(200), value),
    };

    return HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string());
}


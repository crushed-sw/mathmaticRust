use actix_web::{get, post, web, HttpResponse, Responder};
use crate::{
    entity::{
        article::{
            Article,
            RequestAllArticle,
            RequestArticle,
            RequestDeleteOneArticle,
            RequestOneArticle,
            RequestOtherArticle,
            RequestOwnArticle
        },
        result,
        result_code::ResultCode,
    },
    service::{
        comment_service::CommentService,
        favorite_service::FavoriteService,
        article_service::ArticleService,
    },
    util::util::Util,
};

#[post("/publish_article")]
pub async fn publish_article(article: web::Json<RequestArticle>) -> impl Responder {
    let check = Util::check_token(article.email.as_str(), article.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let new_article = Article {
        id: None,
        title: Some(article.title.clone()),
        content: Some(article.content.clone()),
        time: Some(article.time.clone()),
        author: Some(article.author.parse::<u64>().unwrap()),
    };

    let insert_result = ArticleService::insert(&new_article).await;
    let result = match insert_result {
        Err(_) => result::Result::error(20091, ResultCode::get_message(20091)),
        Ok(_) => result::Result::success(20090, ResultCode::get_message(20090), ""),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_num_own_article")]
pub async fn get_num_own_article(user: web::Query<RequestOwnArticle>) -> impl Responder {
    let check = Util::check_token(user.email.as_str(), user.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let num_result = ArticleService::get_preview_num_by_author(user.userid.as_str()).await;
    let result = match num_result {
        Err(_) => result::Result::error(20111, ResultCode::get_message(20111)),
        Ok(res_vec) => result::Result::success(20110, ResultCode::get_message(20110), res_vec.len())
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_own_article")]
pub async fn get_own_article(user: web::Query<RequestOwnArticle>) -> impl Responder {
    let check = Util::check_token(user.email.as_str(), user.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let page_result = ArticleService::select_preview_page_by_author(user.page.parse::<u64>().unwrap_or_default(), user.userid.as_str()).await;
    let result = match page_result {
        Err(_) => result::Result::error(20101, ResultCode::get_message(20101)),
        Ok(page) => result::Result::success(20100, ResultCode::get_message(20100), page),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_num_other_article")]
pub async fn get_num_other_article(user: web::Query<RequestOtherArticle>) -> impl Responder {
    let num_result = ArticleService::get_preview_num_by_author(user.userid.as_str()).await;
    let result = match num_result {
        Err(_) => result::Result::error(20111, ResultCode::get_message(20111)),
        Ok(res_vec) => result::Result::success(20110, ResultCode::get_message(20110), res_vec.len())
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_other_article")]
pub async fn get_other_article(user: web::Query<RequestOtherArticle>) -> impl Responder {
    let page_result = ArticleService::select_preview_page_by_author(user.page.parse::<u64>().unwrap_or_default(), user.userid.as_str()).await;
    let result = match page_result {
        Err(_) => result::Result::error(20101, ResultCode::get_message(20101)),
        Ok(page) => result::Result::success(20100, ResultCode::get_message(20100), page),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_num_all_article")]
pub async fn get_num_all_article(_inform: web::Query<RequestAllArticle>) -> impl Responder {
    let num_result = ArticleService::select_num_preview_page().await;
    let result = match num_result {
        Err(_) => result::Result::error(20111, ResultCode::get_message(20111)),
        Ok(res_vec) => result::Result::success(20110, ResultCode::get_message(20110), res_vec.len())
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_all_article")]
pub async fn get_all_article(inform: web::Query<RequestAllArticle>) -> impl Responder {
    let page_result = ArticleService::select_preview_page(inform.page.parse::<u64>().unwrap_or_default()).await;
    let result = match page_result {
        Err(_) => result::Result::error(20101, ResultCode::get_message(20101)),
        Ok(page) => result::Result::success(20100, ResultCode::get_message(20100), page),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/delete_article")]
pub async fn delete_article(inform: web::Query<RequestDeleteOneArticle>) -> impl Responder {
    let check = Util::check_token(inform.email.as_str(), inform.token.as_str()).await;
    if !check.success {
        return HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .append_header(("Access-Control-Allow-Origin", "*"))
            .body(check.to_string());
    };

    let delete_favorite = FavoriteService::delete_by_articleid(inform.id.as_str()).await;
    let delete_comment = CommentService::delete_by_articleid(inform.id.as_str()).await;
    let result = match (delete_favorite, delete_comment) {
        (Ok(_), Ok(_)) => {
            let delete_result = ArticleService::delete_by_id(inform.id.as_str()).await;
            match delete_result {
                Err(_) => result::Result::error(1, ResultCode::get_message(1)),
                Ok(_) => result::Result::success(200, ResultCode::get_message(200), ""),
            }
        },
        _ => result::Result::error(1, ResultCode::get_message(1)),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

#[get("/get_article")]
pub async fn get_article(inform: web::Query<RequestOneArticle>) -> impl Responder {
    let get_result = ArticleService::get_article_by_id(inform.id.as_str()).await;
    let result = match get_result {
        Err(_) => result::Result::error(1, ResultCode::get_message(1)),
        Ok(article) => result::Result::success(20100, ResultCode::get_message(20100), article),
    };

    HttpResponse::Ok()
        .append_header(("Content-Type", "text/plain"))
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(result.to_string())
}

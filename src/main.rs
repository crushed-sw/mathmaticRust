mod config;
mod controller;
mod entity;
mod service;
mod util;
mod test;

use actix_web::{App, HttpServer};

use controller::{
    article_controller::{
        delete_article,
        get_all_article,
        get_article,
        get_num_all_article,
        get_num_other_article,
        get_num_own_article,
        get_other_article,
        get_own_article,
        publish_article
    },
    comment_controller::{
        delete_comment,
        get_comment,
        get_num_comment,
        insert_comment
    },
    favorite_controller::{
        delete_favorite,
        get_favorite,
        get_num_favorite,
        insert_favorite,
        is_favorite
    },
    user_controller::{
        change_username,
        check,
        get_avatar,
        get_inform,
        login,
        register,
        reset_password,
        send_code
    },
    chem_controller::{
        molecule,
        atom,
        orbit
    }
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    service::CONTEXT.init_database().await;

    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(register)
            .service(send_code)
            .service(check)
            .service(change_username)
            .service(reset_password)
            .service(get_inform)
            .service(publish_article)
            .service(get_own_article)
            .service(get_num_own_article)
            .service(get_other_article)
            .service(get_num_other_article)
            .service(get_all_article)
            .service(get_num_all_article)
            .service(delete_article)
            .service(get_article)
            .service(insert_favorite)
            .service(delete_favorite)
            .service(is_favorite)
            .service(get_num_favorite)
            .service(get_favorite)
            .service(insert_comment)
            .service(delete_comment)
            .service(get_num_comment)
            .service(get_comment)

            .service(get_avatar)
            .service(molecule)
            .service(atom)
            .service(orbit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


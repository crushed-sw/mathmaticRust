#[cfg(test)]
mod tests {
    use super::super::super::controller::molecule::molecule;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_index_ok() {
        let app = test::init_service(App::new().service(molecule)).await;
        let req = test::TestRequest::get().uri("/molecule?value=CC").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}

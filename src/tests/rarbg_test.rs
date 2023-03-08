#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, web, App};

    #[actix_web::test]
    async fn rarbg_get_all_torrents() {
        let app = test::init_service(App::new().service(get_all_torrents)).await;
        let req = test::TestRequest::get()
            .uri("/rarbg/torrents/The%20last%20of%20us")
            .to_request();
        let resp = test::call_service(&app, req).await;
        //println!("{:?}",resp.into_body());
        assert!(resp.status().is_success());
    }
}

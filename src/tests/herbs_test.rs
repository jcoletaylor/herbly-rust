#[cfg(test)]
mod test {
    use crate::app;
    use crate::state;
    use tide::http::{Method, Request, Response, Url};
    use tide::Server;

    async fn get_server() -> Server<state::State> {
        let db_pool = app::make_db_pool().await;
        let server = app::make_server(db_pool).await;
        server
    }
    #[async_std::test]
    async fn get_one_herb() -> tide::Result<()> {
        dotenv::dotenv().ok();
        let server = get_server().await;
        let url = Url::parse("https://example.com/herbs/AiYe").unwrap();
        let req = Request::new(Method::Get, url);
        let res: Response = server.respond(req).await?;
        assert_eq!(res.status(), 200);
        Ok(())
    }

    #[async_std::test]
    async fn fail_to_get_one_herb() -> tide::Result<()> {
        dotenv::dotenv().ok();
        let server = get_server().await;
        let url = Url::parse("https://example.com/herbs/OBVIOUSLYFAKE").unwrap();
        let req = Request::new(Method::Get, url);
        let res: Response = server.respond(req).await?;
        assert_eq!(res.status(), 404);
        Ok(())
    }

    #[async_std::test]
    async fn get_all_herbs() -> tide::Result<()> {
        dotenv::dotenv().ok();
        let server = get_server().await;
        let url = Url::parse("https://example.com/herbs").unwrap();
        let req = Request::new(Method::Get, url);
        let res: Response = server.respond(req).await?;
        assert_eq!(res.status(), 200);
        Ok(())
    }
}

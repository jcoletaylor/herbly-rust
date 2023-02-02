#[cfg(test)]
mod test {
    use crate::tests::helpers;
    use tide::http::{Method, Request, Response, StatusCode, Url};

    #[async_std::test]
    async fn get_one_formula() -> tide::Result<()> {
        let server = helpers::get_server().await;
        let url = Url::parse("https://example.com/formulas/BaiHuTang").unwrap();
        let req = Request::new(Method::Get, url);
        let res: Response = server.respond(req).await?;
        assert_eq!(res.status(), StatusCode::Ok);
        Ok(())
    }

    #[async_std::test]
    async fn fail_to_get_one_formula() -> tide::Result<()> {
        let server = helpers::get_server().await;
        let url = Url::parse("https://example.com/formulas/OBVIOUSLYFAKE").unwrap();
        let req = Request::new(Method::Get, url);
        let res: Response = server.respond(req).await?;
        assert_eq!(res.status(), StatusCode::NotFound);
        Ok(())
    }

    #[async_std::test]
    async fn get_all_formulas() -> tide::Result<()> {
        let server = helpers::get_server().await;
        let url = Url::parse("https://example.com/formulas").unwrap();
        let req = Request::new(Method::Get, url);
        let res: Response = server.respond(req).await?;
        assert_eq!(res.status(), StatusCode::Ok);
        Ok(())
    }
}

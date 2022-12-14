pub mod database;

pub mod handler {
    use async_trait::async_trait;
    use salvo::{http::StatusCode, writer::Json, Depot, FlowCtrl, Handler, Request, Response};
    use sqlx::PgPool;

    use crate::{
        app::{resource::iam::CreateUserDto, use_case},
        domain::service::Argon2HashService,
        error::http::BadRequest,
    };

    macro_rules! map_res_err {
        ($result:ident, $response:ident) => {
            match $result {
                Err(err) => {
                    $response.render(err);
                    return;
                }
                Ok(ok) => ok,
            }
        };
    }

    pub struct CreateUserHandler {
        pool: PgPool,
        hash_service: Argon2HashService,
    }

    impl CreateUserHandler {
        pub fn new(pool: PgPool, hash_service: Argon2HashService) -> Self {
            Self { pool, hash_service }
        }
    }

    #[async_trait]
    impl Handler for CreateUserHandler {
        async fn handle(
            &self,
            req: &mut Request,
            _: &mut Depot,
            res: &mut Response,
            _: &mut FlowCtrl,
        ) {
            let result: Result<CreateUserDto, _> = req.parse_body().await.map_err(BadRequest::from);
            let dto = map_res_err!(result, res);

            let result = use_case::iam::create_user(&self.pool, &self.hash_service, dto).await;
            let user = map_res_err!(result, res);

            res.render(Json(user));
            res.set_status_code(StatusCode::CREATED);
        }
    }
}

pub mod query {}

pub mod router {
    use salvo::{logging::Logger, Router};
    use sqlx::PgPool;

    use super::handler::*;
    use crate::domain::service::Argon2HashService;

    pub fn app(pool: PgPool, hash_service: Argon2HashService) -> Router {
        Router::new().hoop(Logger).push(
            Router::with_path("api")
                .push(Router::with_path("user").post(CreateUserHandler::new(pool, hash_service))),
        )
    }
}

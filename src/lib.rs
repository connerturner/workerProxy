use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "[{}] {:?} {} {}",
        Date::now().to_string(),
        req.headers().get("cf-connecting-ip").unwrap_or_default().unwrap_or_default().to_string(),
        req.method().to_string(),
        req.path()
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/test", |mut req, _ctx| {
            Response::ok("/test success.")
        })

        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .or_else_any_method("/", |_, _ctx| {
            Response::error("Method not allowed", 405)
        })
        .run(req, env)
        .await
}

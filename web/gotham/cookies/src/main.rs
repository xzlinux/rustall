use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use gotham::state::{FromState,State};
use gotham::middleware::cookie::CookieParser;
use gotham::hyper::{Body,Response,StatusCode};
use gotham::hyper::header::SET_COOKIE;
use gotham::helpers::http::response::create_response;
use gotham::pipeline::new_pipeline;
use cookie::{Cookie,CookieJar};
use gotham::router::builder::*;


fn handler(state: State)->(State,Response<Body>){
    let adjective = {
        CookieJar::borrow_from(&state)
            .get("adjective")
            .map(|adj_cookie| adj_cookie.value().to_owned())
            .unwrap_or_else(|| "first time".to_string())
    };
    let mut response = create_response(
        &state,
        StatusCode::OK,
        mime::TEXT_PLAIN,
        format!("Hello {} visitor\n",adjective),
    );
    {
        let cookie = Cookie::build("adjective","repeat")
            .http_only(true)
            .finish();
        response.headers_mut().append(SET_COOKIE,cookie.to_string().parse().unwrap());
    }
    (state,response)
}
fn router() -> Router {
    let (chain,piplines) = single_pipeline(new_pipeline().add(CookieParser).build());
    build_router(chain,piplines,|route|{
        route.get("/").to(handler);
    })
}
fn main(){
    let addr = "127.0.0.1:7878";
    println!("Listening for request at http://{}",addr);
    gotham::start(addr,router());
}
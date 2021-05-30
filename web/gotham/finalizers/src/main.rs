
use gotham::hyper::{body::Body,Response,StatusCode};
use gotham::router::builder::*;
use gotham::router::response::extender::ResponseExtender;
use gotham::router::Router;
use gotham::state::State;

const HELLO_ROUTER: &str = "hello Router!";
pub fn say_hello(state: State) -> (State, &'static str){
    (state,HELLO_ROUTER)
}
struct ErrorExtender;
impl ResponseExtender<Body> for ErrorExtender{
    fn extend(&self,_state: &mut State, response: &mut Response<Body>){
        let body = format!("The status code is {}",response.status());
        *response.body_mut() = body.into();
    }
}
fn router() -> Router {
    build_simple_router(|route|{
        route.get("/").to(say_hello);
        route.add_response_extender(StatusCode::NOT_FOUND,ErrorExtender);
        
    })
}
pub fn main(){
    let addr = "127.0.0.1:7878";
    println!("Listening for request at http://{}",addr);
    gotham::start(addr,router())
}
extern crate iron;
#[macro_use]extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/logged", post_logged , "logged");

    println!("Wbijaj na localhost:3000");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {

    let mut response = Response::new();

     response.set_mut(status::Ok);
     response.set_mut(mime!(Text/Html; Charset=Utf8));
     response.set_mut(r#"

        <form action="/logged" method="post"> 

        <input type="text" name="username"></input>
        <input type="text" name="password"></input>
        <button type="sumbit">zaloguj sie</button>
        </form>
        "#
    );
    Ok(response)
}

fn post_logged(request : &mut Request) -> IronResult<Response> {

    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>(){
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Błąd parsowania danych formularza {:?}, {:?}", e));
            return Ok(response);

        }
        Ok(map) => map;
    }
    let unparsed_data = match form_data.get("username"){
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Formularz nie zawiera loginu"));
            return Ok(response);
        }
        Some(nums) => nums;
    }
    
}


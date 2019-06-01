extern crate iron;
#[macro_use] extern crate mime; // Use mime crate as well as its defined macros

use iron::prelude::*; // Prelude directives should always be put in scope
use iron::status;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

// This function is accessed via web
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
        <input type="text" name="m" />
        <input type="text" name="n" />
        <button type="submit">Compute GCD</button>
    </form>
    "#);

    Ok(response)
}

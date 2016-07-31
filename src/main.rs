extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::status::StatusCode;
use nickel::{Nickel, Request, Response, MiddlewareResult, HttpRouter, QueryString, StaticFilesHandler, JsonBody, MediaType};
use rustc_serialize::json;

// =============================================================
// Config Options
// =============================================================

// Default code which is used to populate initial text box
static CODE: &'static str  = "import whiley.lang.System\n\nmethod main(System.Console console):\n    console.out.println_s(\"Hello World\")";

// =============================================================
// Compile Service API
// =============================================================

#[derive(RustcDecodable, RustcEncodable)]
struct CompileRequest {
    code: String,
    verify: String
}

#[derive(RustcDecodable, RustcEncodable)]
struct CompileResponse {
    result: String
}

fn compile<'a>(request: &mut Request, mut response: Response<'a>) -> MiddlewareResult<'a> {
    //let cr = request.json_as::<CompileRequest>().unwrap();
    
    let object = CompileResponse {
        result: "success".to_string(),
    };
    let json_obj = json::encode(&object).unwrap();
    response.set(MediaType::Json);
    response.set(StatusCode::Ok);    
    //
    return response.send(json_obj);
}

// =============================================================
// Main
// =============================================================

fn main() {
    let mut server = Nickel::new();

    // Setup route for index which displays the main page.
    server.get("/", middleware! { |request, response|
        let mut data = HashMap::new();
	data.insert("CODE", CODE);	
	return response.render("assets/html/index.html", &data)
    });    

    // Setup route for compilation service.  This allows Whiley files
    // to be compiled and returns a JSON response.
    server.post("/compile", compile);
   
    // Configure utilisation options
    server.utilize(StaticFilesHandler::new("assets/"));
    // Done
    server.listen("127.0.0.1:8080");
}

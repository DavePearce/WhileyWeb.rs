extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;

use nickel::status::StatusCode;
use nickel::{Nickel, NickelError, Request, Response, MiddlewareResult, HttpRouter, StaticFilesHandler, JsonBody, MediaType};
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
    verify: bool,
}

#[derive(RustcDecodable, RustcEncodable)]
struct CompileResponse {
    result: String,
}

fn index<'mw, 'conn>(_req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("CODE", CODE);	
    return res.render("assets/html/index.html", &data)
}

fn compile<'a>(request: &mut Request, mut response: Response<'a>) -> MiddlewareResult<'a> {
    // Decode JSON request for processing
    let cr = request.json_as::<CompileRequest>().unwrap();
    // Write code to file for use in compilation
    let r = File::create("main.whiley");
    if r.is_err() {
        return Err(NickelError::new(response,"Error writing temporary file", StatusCode::BadRequest));
    }
    let r = r.unwrap().write_all(cr.code.as_bytes());
    // Perform compilation
    
    // Create JSON response
    let object = CompileResponse {
        result: "success".to_string(),
    };
    // Encode and send response
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
    server.get("/", index);    
    // Setup route for compilation service.  This allows Whiley files
    // to be compiled and returns a JSON response.
    server.post("/compile", compile);   
    // Configure utilisation options
    server.utilize(StaticFilesHandler::new("assets/"));
    // Done
    server.listen("127.0.0.1:8080");
}

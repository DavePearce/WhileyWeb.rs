#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, QueryString, StaticFilesHandler};

// =============================================================
// Config Options
// =============================================================

// Default code which is used to populate initial text box
static CODE: &'static str  = "import whiley.lang.System\n\nmethod main(System.Console console):\n    console.out.println_s(\"Hello World\")";

// =============================================================
// Main
// =============================================================

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("assets/"));

    server.get("/", middleware! { |request, response|
        let mut data = HashMap::new();
	data.insert("CODE", CODE);	
	return response.render("assets/html/index.html", &data)
    });

    server.listen("127.0.0.1:8080");
}

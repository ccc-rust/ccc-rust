#![deny(warnings)]
use warp::Filter;

#[allow(dead_code)]
async fn hello_server() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[allow(dead_code)]
async fn file_server() {
    pretty_env_logger::init();

    let readme = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));

    // dir already requires GET...
    let examples = warp::path("ex").and(warp::fs::dir("./examples/"));

    // GET / => README.md
    // GET /ex/... => ./examples/..
    let routes = readme.or(examples);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[tokio::main]
async fn main() {
    // hello_server().await;
    file_server().await;
}

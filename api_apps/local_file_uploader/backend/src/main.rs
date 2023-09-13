mod handlers;

use warp::Filter;

#[tokio::main]
async fn main() {
    let upload = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form())
        .and_then(handlers::upload_handler);

    warp::serve(upload).run(([127, 0, 0, 1], 8000)).await;
}

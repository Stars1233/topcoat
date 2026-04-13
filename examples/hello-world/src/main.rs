use topcoat::{View, axum::routing::get, page, view};

#[page("/carl")]
async fn my_page() -> View {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"hello world"</title>
            </head>
            <body id="test">
                <div>"hi"</div>
            </body>
        </html>
    }
}

#[tokio::main]
async fn main() {
    let topcoat_router = topcoat::router::Router::new().page(my_page);

    let axum_router = axum::Router::new()
        .merge(topcoat_router)
        .route("/axum", get(async || {}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, axum_router).await.unwrap();
}

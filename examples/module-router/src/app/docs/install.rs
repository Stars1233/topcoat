use topcoat::{Result, router::page, view::view};

// Child modules append their segment: docs/install.rs -> /docs/install.
#[page]
async fn install() -> Result {
    view! {
        <h1>"install"</h1>
        <p>"src/app/docs/install.rs -> /docs/install"</p>
    }
}

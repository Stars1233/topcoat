use topcoat::{Result, router::page, view::view};

#[page]
async fn contact_page() -> Result {
    view! { "contact" }
}

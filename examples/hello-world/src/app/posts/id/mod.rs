use topcoat::{
    context::Cx,
    router::{page, segment},
    view::{View, view},
};

segment!(id: uuid::Uuid);

#[page]
async fn post_page(cx: &Cx) -> View {
    view! { "showing post with id: " (id(cx).as_ref().unwrap().to_string()) }
}

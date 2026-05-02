use topcoat::{
    context::Cx,
    router::{page, path_param, segment},
    view::{View, view},
};

segment!(kind = Param);

path_param!(id: uuid::Uuid);

#[page]
async fn post_page(cx: &Cx) -> View {
    view! { "showing post with id: " (id(cx).to_string()) }
}

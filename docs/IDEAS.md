```rust
use topcoat::{component, html, Router};

#[component]
fn my_button() {

}

#[component]
async fn events() -> Html {
    let user = require_auth().await;

    let db = use_db();
    let events = Event::get_by_user_id().exec(&mut db).await;

    html! {
        div class="flex flex-col" {
            for event in events {
                div class="card" {
                    h3 { (event.title) }

                    if event.admin_user_id == user.id {
                        (my_button) { "Edit" }
                    }
                }
            }
        }
    }
}

#[layout]
async fn nav(children: Html) -> Html {
    html! {
        div {
            nav {
                // Type-safe href somehow based on the components?
                a href={events} { "Events" }
                a href="/submissions" { "Submissions" }

                if use_signed_in().await {
                    // Inline API handlers?
                    (my_button) onclick={async || {
                        delete_session().await;
                        return redirect(sign_in);
                    }} {
                        "Sign out"
                    }
                }
            }
            main {
                (children)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .layout(nav)
        .get("/events", events);

    // or
    let router = Router::file();

    topcoat::serve(router).await;
}
```

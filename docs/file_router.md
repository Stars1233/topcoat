# File-based routing

The `file_router!` macro derives routes from your Rust module structure. No manual route registration, no path strings scattered across files — your file tree *is* your route table.

## Setup

Call `file_router!()` from the root module of your route tree. This module becomes the root `/` path.

```rust
// src/app/mod.rs
pub fn router() -> topcoat::router::Router {
    topcoat::router::file_router!()
}
```

Every `#[page]` and `#[layout]` in modules under `app/` is automatically discovered and registered.

## How files map to routes

Each file's path relative to the root module determines its URL. Module names are converted to **kebab-case** (`user_settings` becomes `user-settings`). A `mod.rs` file represents its parent directory.

| File | Route |
|---|---|
| `app/mod.rs` | `/` |
| `app/about.rs` | `/about` |
| `app/blog_posts.rs` | `/blog-posts` |
| `app/settings/mod.rs` | `/settings` |
| `app/settings/profile.rs` | `/settings/profile` |

## Pages and layouts

A `#[page]` defines a route handler. A `#[layout]` wraps all pages in the same directory and its subdirectories.

```rust
// src/app/mod.rs — layout at "/" wraps all pages
#[layout]
async fn root_layout(slot: Slot) -> View {
    view! {
        <html><body>(slot.await)</body></html>
    }
}

#[page]
async fn home() -> View {
    view! { <h1>"Home"</h1> }
}
```

```rust
// src/app/about.rs — page at "/about"
#[page]
async fn about() -> View {
    view! { <h1>"About"</h1> }
}
```

## Groups

Directories prefixed with `_` are **groups**. They organize files and can hold shared layouts, but they don't add a path segment to the URL.

```
app/
  mod.rs              # layout at /
  _marketing/
    mod.rs            # layout wrapping marketing pages (no route segment)
    pricing.rs        # /pricing
    features.rs       # /features
  _docs/
    mod.rs            # layout wrapping docs pages (no route segment)
    getting_started.rs # /getting-started
```

Both `pricing` and `getting_started` are top-level routes, but they can have different layouts via their respective group `mod.rs` files.

You can also override a group to behave as a static segment with `segment!`:

```rust
// src/app/_group/mod.rs
topcoat::router::segment!(kind = Static);
```

This turns it into a regular path segment (the `_` prefix is stripped and the name is kebab-cased).

The `_`-prefix can also act as a naming convention for route-specific utilities. For example, a `_components` module for shared UI fragments:

```
app/
  mod.rs
  _components/
    mod.rs            # exports shared components, no route
    header.rs
    footer.rs
  about.rs            # /about — can use app::_components::header
  contact.rs          # /contact
```

## Dynamic segments (params)

Use `segment!` to mark a module as a dynamic parameter:

```rust
// src/app/users/id/mod.rs
topcoat::router::segment!(kind = Param);
```

This maps `app/users/id/` to `/users/{id}`. Any file inside that module inherits the param segment:

| File | Route |
|---|---|
| `app/users/id/mod.rs` | `/users/{id}` |
| `app/users/id/settings.rs` | `/users/{id}/settings` |

## Catch-all segments

For routes that match any number of trailing path segments:

```rust
// src/app/docs/path/mod.rs
topcoat::router::segment!(kind = CatchAll);
```

This maps `app/docs/path/` to `/docs/{*path}`.

## Renaming segments

If you want the URL segment to differ from the module name:

```rust
// src/app/blog_post.rs
topcoat::router::segment!(rename = "articles");
// Route: /articles instead of /blog-post
```

You can combine `kind` and `rename`:

```rust
// src/app/slug.rs
topcoat::router::segment!(kind = Param, rename = "id");
// Route: /{id}
```

## Segment kinds

| Kind | URL format | Use case |
|---|---|---|
| `Static` | `/name` | Default for regular modules |
| `Group` | *(hidden)* | Default for `_`-prefixed modules; layout-only grouping |
| `Param` | `/{name}` | Dynamic path parameter |
| `CatchAll` | `/{*name}` | Matches all remaining path segments |

## Example: blog with user profiles

```
src/app/
  mod.rs                    # layout at /, page at /
  _auth/
    mod.rs                  # auth-required layout (no URL segment)
    dashboard.rs            # /dashboard
  users/
    mod.rs                  # page at /users
    id/
      mod.rs                # page at /users/{id}  (segment! Param)
      posts.rs              # page at /users/{id}/posts
  posts/
    mod.rs                  # page at /posts
    slug.rs                 # page at /posts/{slug}  (segment! Param)
```

```rust
// src/app/mod.rs
mod _auth;
mod posts;
mod users;

use topcoat::{
    router::{Slot, layout, page},
    view::{View, view},
};

pub fn router() -> topcoat::router::Router {
    topcoat::router::file_router!()
}

#[layout]
async fn root_layout(slot: Slot) -> View {
    view! {
        <!DOCTYPE html>
        <html>
            <body>
                <nav>
                    <a href="/">"Home"</a>
                    <a href="/posts">"Blog"</a>
                    <a href="/dashboard">"Dashboard"</a>
                </nav>
                (slot.await)
            </body>
        </html>
    }
}

#[page]
async fn home() -> View {
    view! { <h1>"Welcome"</h1> }
}
```

```rust
// src/app/_auth/mod.rs — wraps dashboard in auth check, no URL segment
use topcoat::{
    router::{Slot, layout},
    view::{View, view},
};

#[layout]
async fn auth_layout(slot: Slot) -> View {
    // In a real app, check auth here
    view! {
        <div class="authenticated">
            (slot.await)
        </div>
    }
}
```

```rust
// src/app/users/id/mod.rs — /users/{id}
topcoat::router::segment!(kind = Param);

use topcoat::{router::page, view::{View, view}};

#[page]
async fn user_profile() -> View {
    view! { <h1>"User profile"</h1> }
}
```

```rust
// src/app/posts/slug.rs — /posts/{slug}
topcoat::router::segment!(kind = Param);

use topcoat::{router::page, view::{View, view}};

#[page]
async fn post() -> View {
    view! { <h1>"Blog post"</h1> }
}
```

The resulting routes:

| Route | File |
|---|---|
| `/` | `app/mod.rs` |
| `/dashboard` | `app/_auth/dashboard.rs` |
| `/users` | `app/users/mod.rs` |
| `/users/{id}` | `app/users/id/mod.rs` |
| `/users/{id}/posts` | `app/users/id/posts.rs` |
| `/posts` | `app/posts/mod.rs` |
| `/posts/{slug}` | `app/posts/slug.rs` |

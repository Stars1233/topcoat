fn main() {
    let rendered = topcoat::dom::render(&topcoat::view! {
        html {
            head {
                title { "hello world" }
            }
            body {
                "hi"
                b { "carl" }
            }
        }
    });
    println!("{}", rendered);
}

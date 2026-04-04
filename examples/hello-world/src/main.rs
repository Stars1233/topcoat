fn main() {
    let dynamic = if 5 > 6 {
        topcoat::view! { "hi" }
    } else {
        topcoat::view! { "bye" }
    };

    let content = topcoat::view! {
        html {
            head {
                title { "hello world" }
            }
            body {
                (dynamic)
                " "
                b class="cool" { "carl & friends" }
            }
        }
    };
    println!("{}", content);
}

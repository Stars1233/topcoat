fn main() {
    let content = topcoat::view! {
        html {
            head {
                title { "hello world" }
            }
            body {
                if 5 > 6 {
                    i { "hi" }
                } else {
                    "bye"
                }
                " "
                b class="cool" { "carl & friends" }
            }
        }
    };
    println!("{}", content);
}

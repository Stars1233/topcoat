fn main() {
    let names = ["carl", "julien", "joey"];

    let content = topcoat::view! {
        html {
            head {
                title { "hello world" }
            }
            body {
                for name in names {
                    if name.len() < 5 {
                        div {
                            "hello " (name)
                        }
                    } else {
                        "im " (name)
                    }
                }
            }
        }
    };

    println!("{}", content);
}

fn main() {
    let name = "carl";

    let content = topcoat::view! {
        <html>
            <head>
                <title>"hello world"</title>
            </head>
            <body>
                for name in ["carl", "julien"] {
                    if name.len() < 5 {
                        <div>
                            "hello " (name)
                        </div>
                    } else {
                        "im " (name)
                    }
                }
            </body>
        </html>
    };

    println!("{}", content);
}

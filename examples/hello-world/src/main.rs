// #[topcoat::component]
// fn button(x: i32, y: i32, z: i32, child: topcoat::View) -> topcoat::view::View {
//     topcoat::view! {
//         <button>(child)</button>
//     }
// }

fn main() {
    let content = topcoat::view! {
        <html>
            <head>
                <title>"hello world"</title>
            </head>
            <body id="test">
                [button id="5"]
                    "click me"
                [/button]
            </body>
        </html>
    };

    println!("{}", content);
}

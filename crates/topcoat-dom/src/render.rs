use topcoat_view::repr::{Node, View};

pub fn render(view: &View) -> String {
    let mut buf = String::new();
    render_to(view, &mut buf);
    buf
}

fn render_to(view: &View, buf: &mut String) {
    for node in view.nodes() {
        render_node(node, buf);
    }
}

fn render_node(node: &Node, buf: &mut String) {
    match node {
        // TODO: Handle XSS
        Node::Text(text) => *buf += text,
        Node::Element(element) => {
            *buf += "<";
            *buf += element.name();
            for attribute in element.attributes().items() {
                *buf += " ";
                *buf += attribute.name();
                *buf += "=";
                *buf += attribute.value();
            }
            *buf += ">";
            for node in element.children() {
                render_node(node, buf);
            }
            *buf += "</";
            *buf += element.name();
            *buf += ">";
        }
    }
}

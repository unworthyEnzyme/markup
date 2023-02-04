use std::{collections::HashMap, fmt::Display};

enum HtmlNode<'source> {
    String(&'source str),
    Tag(HtmlNodeTag<'source>),
}
struct HtmlNodeTag<'source> {
    name: &'source str,
    attributes: HashMap<&'source str, &'source str>,
    children: Vec<HtmlNode<'source>>,
}

impl<'source> Display for HtmlNode<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlNode::String(s) => write!(f, "{s}"),
            HtmlNode::Tag(node) => {
                let attributes =
                    node.attributes
                        .iter()
                        .fold(String::new(), |mut acc, (name, value)| {
                            acc.push_str(&format!("{name}=\"{value}\""));
                            acc
                        });
                let inner = node.children.iter().fold(String::new(), |mut acc, node| {
                    acc.push_str(&node.to_string());
                    acc
                });
                write!(
                    f,
                    "<{}{}{}>{}</{}>",
                    node.name,
                    if attributes.is_empty() { "" } else { " " },
                    attributes,
                    inner,
                    node.name
                )
            }
        }
    }
}

impl<'source> HtmlNodeTag<'source> {
    fn add_attribute(&mut self, name: &'source str, value: &'source str) -> &mut Self {
        self.attributes.insert(name, value);
        self
    }
    fn add_children(&mut self, child: HtmlNode<'source>) -> &mut Self {
        self.children.push(child);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::HtmlNodeTag;
    use crate::html::HtmlNode;
    use std::collections::HashMap;

    #[test]
    fn to_string() {
        let mut node = HtmlNodeTag {
            name: "div",
            attributes: HashMap::new(),
            children: vec![],
        };
        node.add_attribute("class", "p-4 flex")
            .add_children(HtmlNode::Tag(HtmlNodeTag {
                name: "p",
                attributes: HashMap::new(),
                children: vec![HtmlNode::String("contents")],
            }));
        let node = HtmlNode::Tag(node);
        assert_eq!(
            node.to_string(),
            r#"<div class="p-4 flex"><p>contents</p></div>"#
        );
    }
}

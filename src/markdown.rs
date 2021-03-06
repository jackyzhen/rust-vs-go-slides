// Based on: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs

use pulldown_cmark::{Alignment, Event, Parser, Tag, OPTION_ENABLE_TABLES};
use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::html::Component;
use yew::html::Html;
use yew::virtual_dom::{VNode, VTag, VText};

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
pub fn render_markdown<COMP>(src: &str) -> Html<COMP>
where
    COMP: Component,
{
    let mut elems: Vec<VNode<COMP>> = Vec::new();
    let mut spine: Vec<VNode<COMP>> = Vec::new();

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            if let VNode::VTag(ref mut vtag) = spine[l - 1] {
                vtag.add_child($child);
            }
        }};
    }

    for ev in Parser::new_ext(src, OPTION_ENABLE_TABLES) {
        match ev {
            Event::Start(tag) => {
                spine.push(VNode::VTag(make_tag(tag)));
            }
            Event::End(tag) => {
                // TODO Verify stack end.
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    top = VNode::VTag(pre);
                } else if let Tag::Table(aligns) = tag {
                    if let VNode::VTag(ref mut vtag) = top {
                        for r in vtag.childs.iter_mut() {
                            if let &mut VNode::VTag(ref mut vtag) = r {
                                for (i, c) in vtag.childs.iter_mut().enumerate() {
                                    if let &mut VNode::VTag(ref mut vtag) = c {
                                        match aligns[i] {
                                            Alignment::None => {}
                                            Alignment::Left => vtag.add_class("text-left"),
                                            Alignment::Center => vtag.add_class("text-center"),
                                            Alignment::Right => vtag.add_class("text-right"),
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Tag::TableHead = tag {
                    if let VNode::VTag(ref mut vtag) = top {
                        for c in vtag.childs.iter_mut() {
                            if let &mut VNode::VTag(ref mut vtag) = c {
                                vtag.add_attribute("scope", &"col");
                            }
                        }
                    }
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    if let VNode::VTag(ref mut vtag) = spine[l - 2] {
                        vtag.add_child(top.into());
                    }
                }
            }
            Event::Text(text) => add_child!(VText::new(text.to_string()).into()),
            Event::SoftBreak => add_child!(VText::new("\n".to_string()).into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),

            // render any raw html as is
            Event::Html(html) | Event::InlineHtml(html) => {
                let html_string = html.to_string();
                let dynamic_html = js! {
                    const div = document.createElement("div");
                    div.innerHTML = @{html_string};
                    return div;
                };
                let node = Node::try_from(dynamic_html).expect("convert dynamic html");
                let vnode = VNode::VRef(node);
                elems.push(vnode);
            }
            _ => println!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        elems.pop().unwrap()
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn make_tag<COMP>(t: Tag) -> VTag<COMP>
where
    COMP: Component,
{
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Rule => VTag::new("hr"),
        Tag::Header(n) => {
            assert!(n > 0);
            assert!(n < 7);
            VTag::new(format!("h{}", n))
        }
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_class("blockquote");
            el
        }
        Tag::CodeBlock(lang) => {
            let mut el = VTag::new("code");
            // Different color schemes may be used for different code blocks,
            // but a different library (likely js based at the moment) would be necessary to actually provide the
            // highlighting support by locating the language classes and applying dom transforms
            // on their contents.
            match lang.as_ref() {
                "html" => el.add_class("html-language"),
                "rust" => el.add_class("rust-language"),
                "java" => el.add_class("java-language"),
                "c" => el.add_class("c-language"),
                _ => {} // Add your own language highlighting support
            };
            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start);
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_class("table");
            el
        }
        Tag::TableHead => VTag::new("tr"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_class("font-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_class("font-weight-bold");
            el
        }
        Tag::Code => VTag::new("code"),
        Tag::Link(ref href, ref title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href);
            if title != "" {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::Image(ref src, ref title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src);
            if title != "" {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::FootnoteDefinition(ref _footnote_id) => VTag::new("span"), // Footnotes are not rendered as anything special
    }
}

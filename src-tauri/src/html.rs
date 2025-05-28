use std::collections::{HashMap, HashSet};
use ammonia::{Builder};
use kuchiki::{parse_html, traits::TendrilSink, NodeData, NodeRef, Attributes};
use cssparser::{Parser, ToCss, Token, ParserInput};
use html_to_pdf_lib::html_to_pdf;
#[macro_use] extern crate markup5ever;
fn sanitize_for_newspaper(html: &str) -> String {
    // 配置允许的HTML标签白名单
    let tags: HashSet<&str> = [
        "p", "br", "b", "i", "strong", "em",
        "ul", "ol", "li", "h2", "h3", "h4"
    ].iter().cloned().collect();

    // 配置允许的HTML属性白名单
    let attributes: HashMap<&str, HashSet<&str>> = [
        ("*", ["class"].iter().cloned().collect())
    ].iter().cloned().collect();

    // 构建专业清理器
    Builder::new()
        .tags(tags)
        .generic_attributes(attributes["*"].clone())
        .tag_attributes(attributes)
        .clean(html)
        .to_string()
}


pub fn clean_and_set_song_font(title: &str, from_who: &str, html: &str) -> String {
    let html=("<html><body>".to_string()+html+"</body></html>").as_str();

    // 第一阶段：基础清理
    let sanitized = sanitize_for_newspaper(html);

    // 第二阶段：字体强制设置
    let mut document = parse_html().one(sanitized);
    process_font_styles(&document);

    // 第三阶段：添加本报讯
    add_header(from_who, title, &mut document);
    // 获取处理后的HTML字符串
    document.to_string()
}

/// 深度处理字体样式
fn process_font_styles(node: &NodeRef) {
    match node.data() {
        NodeData::Element(mut element) => {
            let tag_name = element.name.local.to_lowercase();

            // 处理<font>标签的face属性
            if tag_name == "font" {
                if let Some(mut attributes) = element.attributes.borrow_mut().take() {
                    attributes.insert("face", "宋体");
                    element.attributes = std::cell::RefCell::new(Some(attributes));
                }
            }

            // 处理style属性
            if let Some(style) = element.attributes.borrow().get("style") {
                let new_style = rewrite_style(style);
                element.attributes.borrow_mut().insert("style", new_style);
            }

            // 添加默认字体声明
            if !element.attributes.borrow().contains("style")
                && TEXT_CONTAINERS.contains(&tag_name.as_str())
            {
                element.attributes.borrow_mut().insert("style", "font-family: 宋体, SimSun;font-size:10pt;");
            }

            // 递归处理子节点
            for child in node.children() {
                process_font_styles(&child);
            }
        }
        _ => {}
    }
}

/// 重写CSS样式表
fn rewrite_style(original: &str) -> String {
    let mut input = ParserInput::new(original);
    let mut parser = Parser::new(&mut input);
    let mut output = String::new();

    // 正确遍历CSS声明的方式
    while let Ok(property) = parser.next_including_whitespace_and_comments() {
        match property {
            Token::Ident(name) if name.eq_ignore_ascii_case("font-family") => {
                output.push_str("font-family: 宋体, SimSun;");
                // 跳过原有值
                while !matches!(parser.next(), Ok(Token::Semicolon) | Err(_)) {}
            }
            Token::Ident(name) if name.eq_ignore_ascii_case("font-size") => {
                output.push_str("font-size:10pt;");
                // 跳过原有值
                while !matches!(parser.next(), Ok(Token::Semicolon) | Err(_)) {}
            }
            Token::Semicolon => output.push(';'),
            _ => output.push_str(&property.to_css_string()),
        }
    }

    if !output.contains("font-family") {
        output.push_str("font-family: 宋体, SimSun;");
    }
    if !output.contains("font-size") {
        output.push_str("font-family: 宋体, SimSun; font-size:10pt;");
    }

    output
}

fn add_header(from_who:&str, title: &str, node:&mut NodeRef) {
    let h2 = NodeRef::new_element(
        "h2".into(),
        vec![
            ("style", "display: inline; font-weight: bold;font-family: 宋体, SimSun;font-size:16pt;"),
            ("class", "newspaper-header")
        ]
            .into_iter());
    h2.append(NodeRef::new_text(title));
    let header_span = NodeRef::new_element(
        "span".into(),
        vec![("style", "display: inline;font-family: stzhongs;font-size:10pt;")]
            .into_iter());
    header_span.append(NodeRef::new_text("本报讯 ".to_string()+from_who+" "));
    if let Ok(first_p) = node.select_first("p") {
        first_p.as_node().prepend(header_span);
    }
}
/// 需要添加默认字体的容器标签
const TEXT_CONTAINERS: &[&str] = &[
    "p", "div", "span",
    "h1", "h2", "h3", "h4", "h5", "h6",
    "li", "td", "th"
];

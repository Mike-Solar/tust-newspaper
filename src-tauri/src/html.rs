use core::cell::RefCell;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, Read, Write};
use std::rc::Rc;
use std::str::FromStr;
use ammonia::{Builder};
use cssparser::{Parser, ToCss, Token, ParserInput};
use html_to_pdf_lib::html_to_pdf;
use html5ever::{parse_document, LocalName};
use html5ever::serialize::{serialize};
use html5ever::tendril::{StrTendril, TendrilSink};
use markup5ever_rcdom::{RcDom, Handle, Node, NodeData, WeakHandle, SerializableHandle};
use markup5ever::{Attribute, QualName};
use markup5ever::{ns, local_name};
use markup5ever::interface::tree_builder::TreeSink;
use regex::Regex;
use serde::__private::de::IdentifierDeserializer;
use serde::Serialize;

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

    //第一个<p>去掉，因为已经有了
    let mut html=html.to_string().replacen("<p>", "", 1);
    // 第一阶段：基础清理
    let sanitized = sanitize_for_newspaper(html.as_str());

    // 第二阶段：字体强制设置
    let mut dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.to_string().as_bytes())
        .unwrap();
    modify_node(&mut dom.document);
    // 获取处理后的HTML字符串
    let mut html=dom.finish();
    let mut buf = Vec::new();
    let mut doc;SerializableHandle=html.document.clone().into();
    let _=serialize(&buf, &doc, Default::default());
    let mut html=String::from_utf8(buf).unwrap();
    // 读取模板
    let mut f =File::open("template.html").unwrap();
    let mut contents:Box<[u8]>=Box::new([]);
    f.read(&mut *contents).expect("Read failed.");
    let mut contents = str::from_utf8(&contents).unwrap().to_string();
    html.insert_str(html.find("<p>").unwrap()+"<p>".len(),
                    ("<span id=\"from\">".to_string() + from_who + "</span>").as_str());
    contents.replace("Content", html.as_str());
    return contents;
}
fn is_selected_tags(tag:&str)->bool{
    let tags =vec![
        "p", "br", "b", "i", "strong", "em",
        "ul", "ol", "li", "h2", "h3", "h4"
    ];
    for i in tags.iter(){
        if tag.to_string().contains(i) {
            return true;
        }
    }
    return false;
}
fn modify_node(handle: &mut Handle) {
    let node = handle;
    let children = node.children.borrow();

    for child in children.iter() {
        match child.data {
            markup5ever_rcdom::NodeData::Element { ref name, ref mut attrs, .. } => {
                if is_selected_tags(name.local.as_ref()){
                    for mut attr in attrs.borrow_mut().iter_mut() {
                        if attr.name().as_ref() == "style"{
                            let name=QualName::new(
                                None,
                                ns!(html),
                                local_name!("style")
                            );
                            attr=&mut Attribute{
                                name: name,
                                value:StrTendril::from("font-family:SimSun;font-size:10.5pt")
                            };
                        }
                    }
                } else {
                    modify_node(&mut child.clone());
                }
            }
            // 根据节点信息进行匹配...
            _ => (modify_node(&mut child.clone())),
        }
    }
}
/// 深度处理字体样式
fn process_font_styles(dom: &RcDom) {

}

/// 重写CSS样式表
fn rewrite_style(original: &str) -> String {
    let mut input = ParserInput::new(original);
    let mut parser = Parser::new(&mut input);
    let mut output = String::new();

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
        output.push_str("font-size:10pt;");
    }

    output
}
/// 需要添加默认字体的容器标签
const TEXT_CONTAINERS: &[&str] = &[
    "p", "div", "span",
    "h1", "h2", "h3", "h4", "h5", "h6",
    "li", "td", "th"
];

mod html;
mod test;
mod config;

use crate::config::{get_etc_path, ConfigError};
use std::fs::{copy, File};
use std::io::{Error, Read, Write};
use docx_rs::{*};
use std::path::{Path, PathBuf};
use html_to_pdf_lib::html_to_pdf;
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str, TextStr};
use pdf_writer::writers::Page;
use crate::html::clean_and_set_song_font;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
// Create a new handle


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
struct Article{
    pub title: String,
    pub text: String,
    pub from_who: String,
    pub picture: Vec<String>,
    pub words: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
struct Top{
    pub title: String,
    pub text: String,
    pub words: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
struct NewsPage{
    pub num_of_pages: i32,
    pub date_and_num:String,
    pub title: String,
    pub has_top: bool,
    pub top:Top,
    pub editors: String,
    pub articles: Vec<Article>
}
fn print_top(page:& NewsPage, mut pdf: &mut Pdf, mut i: i32)
    -> (i32, Vec<Ref>)
{
    // 常用数值定义
    let a4_height:f32=841.9;
    let a4_width:f32=595.3;
    let up_white:f32=90.1;
    let left_white:f32=72.0;
    let pt5_size:f32=10.5;
    let pt3_size:f32=16.0;
    let mut headline_id:Ref;
    let mut title_content_id:Ref;
    let mut date_content_id:Ref;

    let mut title_id:Ref;
    // 标题行
    title_content_id=Ref::new(i);
    i=i+1;
    let to_line=a4_height-up_white-pt5_size-2.0;
    let mut title_content=Content::new();
    title_content.rect(left_white, a4_height-to_line-10.0, pt3_size*6.0, pt3_size);
    title_content.begin_text();
    title_content.set_font(Name(b"SimSong"), pt3_size);
    title_content.show(Str("天津科技大学学报".to_string().as_bytes()));
    title_content.set_stroke_rgb(255.0,0.00,0.0);
    title_content.end_text();
    pdf.stream(title_content_id, &title_content.finish());

    //日期和版数
    date_content_id=Ref::new(i);
    i+=1;
    let mut date_content=Content::new();
    date_content.rect(left_white, a4_height-to_line-pt3_size-10.0,
                      pt5_size*(page.date_and_num.len() as f32), pt5_size);
    date_content.begin_text();
    date_content.set_font(Name(b"SimSong"), pt5_size);
    date_content.show(Str(page.date_and_num.as_bytes()));
    date_content.set_stroke_rgb(255.0,0.00,0.0);
    date_content.end_text();
    pdf.stream(date_content_id, &date_content.finish());

    title_id=Ref::new(i);
    i=i+1;

    //头版
    headline_id = Ref::new(i);
    i+=1;
    let headline_content_id=Ref::new(i);
    i+=1;
    let mut headline_content2=Content::new();
    headline_content2.rect(a4_width-left_white, to_line-10.0-a4_height,
                           (a4_width-left_white*2.0)/2.0, 340.1);
    headline_content2.begin_text();
    headline_content2.set_font(Name(b"SimSong"), pt5_size);
    headline_content2.show(Str((page.top.words.to_string()+page.top.title.as_str()).as_bytes()));
    // 画方框
    headline_content2.move_to(a4_width-left_white, a4_height-to_line-10.0);
    headline_content2.line_to(a4_width-left_white,a4_height-to_line-10.0-340.1);
    headline_content2.line_to(a4_width-left_white+(a4_width-left_white*2.0)/2.0,
                              a4_height-to_line-10.0-340.1);
    headline_content2.line_to(a4_width-left_white+(a4_width-left_white*2.0)/2.0,
                              a4_height-to_line-10.0);
    headline_content2.line_to(a4_width-left_white, a4_height-to_line-10.0);
    headline_content2.set_line_width(1.0);
    headline_content2.end_text();
    pdf.stream(headline_content_id, &headline_content2.finish());
    return (i, vec![headline_id, title_content_id, date_content_id, title_id, headline_content_id]);
}
fn print_body(page: & NewsPage, pdf:&mut Pdf, mut i: i32) -> (i32, Vec<Ref>)
{
    let a4_height:f32=841.9;
    let a4_width:f32=595.3;
    let up_white:f32=90.1;
    let left_white:f32=72.0;
    let pt5_size:f32=10.5;
    let mut y1:f32=a4_width-600.0;
    let x1:f32=left_white;
    let x2:f32=a4_height-left_white*2.0;

    let num_of_articles=page.articles.len();
    let y2:f32=(y1-up_white)/(num_of_articles as f32) - 10.0;
    let mut refs=Vec::new();
    // 文章
    for doc in page.clone().articles.into_iter() {
        let mut myref=Ref::new(i);
        i+=1;
        refs.push(myref);
        let mut page_content=Content::new();
        page_content.rect(x1, y1, x2, y2);
        page_content.begin_text();
        page_content.set_font(Name(b"SimSong"), pt5_size);
        page_content.show(Str((doc.words.to_string()+doc.title.as_str()).as_bytes()));
        // 画方框
        page_content.move_to(x1,y1);
        page_content.line_to(x1,y1-y2);
        page_content.line_to(x1+x2,y1-y2);
        page_content.line_to(x1+x2,y1);
        page_content.line_to(x1,y1);
        page_content.set_line_width(1.0);
        page_content.end_text();
        pdf.stream(myref, &page_content.finish());
        y1+=y2;
    }
    return (i, refs);
}

fn get_template_path(len:usize, has_top:bool)->Box<Path>{
    let template_name;
    if(has_top) {
        template_name="template-head".to_string()
            
    }
    else { 
        template_name = "template".to_string()+len.to_string().as_str();
    }
    
    let template_name=template_name+len.to_string().as_str()+".html";
    let mut template_path=get_etc_path();
    template_path.push(&template_name);
    Box::from(template_path.as_path())
}
fn insert_articles(page: &NewsPage, path: Box<Path>) -> Result<String, ConfigError>{
    let mut buf:Vec<u8>=Vec::new();
    let mut file=match File::open(path.clone()) {
        Ok(file) => file,
        Err(E)=>return Err(ConfigError::new("配置文件路径不可用"))
    };
    let _ = file.read_to_end(&mut buf).unwrap();
    let mut html=String::from_utf8(buf).unwrap();
    let i=1;
    for article in page.clone().articles.into_iter() {
        html=html.replace(("{Title".to_string()+i.to_string().as_str()+"}").as_str(),
                          (article.words.to_string()+article.title.as_str()).as_str());
    }
    Ok(html)
}

fn insert_top(page: &NewsPage, html: &String) -> String{
    let mut template=html.clone();
    let template=template.replace("{TopTitle}", 
                     (page.top.words.to_string()+page.top.title.as_str()).as_str());
    return template;
}
fn save_typesetting_as_pdf(page:&NewsPage,path: &Path) ->Result<String, impl std::error::Error> {
    let mut template_path:&Path;
    let mut html:String;
    if page.has_top {
        if !(1<=page.articles.len() && page.articles.len()<=4){
            return Err(ConfigError::new("必须有1篇到4篇文章"));
        }
        let template_path=get_template_path(page.articles.len(), true);
        html=match insert_articles(page, template_path)
        {
            Ok(content) => content,
            Err(E) => return Err(E)
        };
        html=insert_top(page, &html);
    }
    else{
        if !(1<=page.articles.len() && page.articles.len()<=5){
            return Err(ConfigError::new("必须有1篇到5篇文章"));
        }
        let template_path=get_template_path(page.articles.len(), false);
        html=match insert_articles(page, template_path)
        {
            Ok(content) => content,
            Err(E) => return Err(E)
        };
    }
    let html=html.replace("{TopTitle}",
                                  page.title.as_str());
    let html=html.replace("{Editors}",
                          page.editors.as_str());
    let html=html.replace("{Page}",
                          page.num_of_pages.to_string().as_str());
    match html_to_pdf(html.as_str(), path){
        Ok(_)=>"OK",
        Err(E)=>return Err(ConfigError::new("无法从HTML转为PDF"))
    };
    Ok(html.to_string())
}

fn save_article_as_pdf(article:&Article, path: &Path) {
    let html=clean_and_set_song_font(article.title.as_str(),
                                     article.from_who.as_str(), article.text.as_str());
    html_to_pdf(html.as_str(), path).unwrap();
}

#[tauri::command]
fn save(page: NewsPage, path: &str){
    let path_obj=Path::new(&path);
    let mut path_buf=PathBuf::from(path_obj);
    let mut path_buf_2=PathBuf::from(path_obj);
    path_buf_2.pop();
    let path_obj=path_buf.as_path();
    let typesetting_path=path_buf.as_path();
    save_typesetting_as_pdf(&page,typesetting_path);
    for article in &page.articles {
        let mut path_buf=PathBuf::from(path_obj);
        path_buf.push(page.title.as_str());
        std::fs::create_dir(path_buf.as_path()).unwrap();
        path_buf.push(page.title.as_str());
        path_buf.set_extension("pdf");
        let article_path=path_buf.as_path();
        save_article_as_pdf(article,article_path);
        let num_list=vec!["图一","图二","图三","图四","图五","图六","图七","图八","图九","图十"];
        let mut i=0;
        while i<article.picture.len() {
            let mut path_buf=PathBuf::from(path_obj);
            path_buf.push(page.title.as_str());
            path_buf.push(num_list[i]);
            let extension:&str=article.picture[i].as_str().split(".").collect::<Vec<&str>>().last().unwrap();
            path_buf.set_extension(
                extension
            );
            copy(Path::new(&article.picture[i]), path_buf.as_path()).unwrap();
        }
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

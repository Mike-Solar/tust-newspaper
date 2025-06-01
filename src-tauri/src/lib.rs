mod html;

use std::fs::{copy, File};
use std::io::Write;
use docx_rs::{*};
use std::path::{Path, PathBuf};
use html_to_pdf_lib::html_to_pdf;
use pdf_writer::{Content, Finish, Name, Pdf, Ref, Str, TextStr};
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
struct Document{
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
    pub page: Vec<Document>
}
fn print_top(page:& NewsPage, mut pdf: &mut Pdf, page_id: &Ref, mut i: i32)
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
    pdf.page(*page_id).contents(title_content_id);
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
    pdf.page(*page_id).contents(date_content_id);
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

    //头版
    headline_id = Ref::new(i);
    i+=1;
    pdf.page(*page_id).contents(headline_id);
    let headline_content_id=Ref::new(i);
    i+=1;
    let mut headline_content2=Content::new();
    headline_content2.rect(a4_width-left_white, a4_height-to_line-10.0,
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
}
fn print_body(page: & NewsPage, pdf:&mut Pdf, page_id: &Ref, mut i: i32)
{
    let a4_height:f32=841.9;
    let a4_width:f32=595.3;
    let up_white:f32=90.1;
    let left_white:f32=72.0;
    let pt5_size:f32=10.5;
    let mut y1:f32=a4_width-600.0;
    let x1:f32=left_white;
    let x2:f32=a4_height-left_white*2.0;

    let num_of_articles=page.page.len();
    let y2:f32=(y1-up_white)/(num_of_articles as f32) - 10.0;
    // 文章
    for doc in page.clone().page.into_iter() {
        let mut myref=Ref::new(i);
        pdf.page(*page_id).contents(myref);
        i+=1;
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
}
fn save_typesetting_as_pdf(page:&NewsPage,path: &Path) {
    // 常用数值定义
    let a4_height:f32=841.9;
    let a4_width:f32=595.3;
    let up_white:f32=90.1;
    let left_white:f32=72.0;
    let pt5_size:f32=10.5;
    let pt3_size:f32=16.0;

    let mut file = std::fs::File::create(path).unwrap();
    let mut pdf = Pdf::new();
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let page_number_id= Ref::new(4);
    let mut head_content_id:Ref = Ref::new(5);
    let mut headline_content_id:Ref = Ref::new(6);

    let mut i=7;

    pdf.catalog(catalog_id).pages(page_tree_id);
    // 页面
    pdf.pages(page_tree_id).kids([page_id]).count(1);
    ;

    pdf.page(page_id).contents(head_content_id);
    pdf.page(page_id).contents(page_number_id);
    pdf.page(page_id).contents(headline_content_id);

    // "x版"
    let mut page_num_content=Content::new();
    let content_str=page.num_of_pages.to_string()+"版";
    page_num_content.rect(left_white, a4_height-up_white,
                      (content_str.len() as f32)*pt5_size, pt5_size);
    page_num_content.begin_text();
    page_num_content.set_font(Name(b"SimSong"), pt5_size);
    page_num_content.show(Str(content_str.as_bytes()));
    page_num_content.end_text();
    pdf.stream(page_number_id, &page_num_content.finish());


    // "本期编辑"和抬头
    let mut head_content=Content::new();
    let mut head_str=page.title.clone()+" 本版编辑："+page.editors.as_str();
    head_content.rect(a4_width-((head_str.len() as f32)*pt5_size), a4_height-up_white,
                      (head_str.len() as f32)*pt5_size, pt5_size);
    head_content.begin_text();
    head_content.set_font(Name(b"SimSong"), pt5_size);
    head_content.show(Str(head_str.as_bytes()));
    head_content.end_text();
    pdf.stream(head_content_id, &head_content.finish());
    let mut refs:Vec<Ref>= Vec::new();

    //下面那条横线
    let mut headline_content=Content::new();
    headline_content.set_line_width(1.0);
    headline_content.move_to(left_white, a4_height-up_white-pt5_size-1.0);
    headline_content.line_to(a4_width-left_white,a4_height-up_white-pt5_size-1.0);
    pdf.stream(headline_content_id, &headline_content.finish());

    // 如果有头版
    if page.has_top {
        print_top(&page, &mut pdf, &page_id, i);

    }
    {
        print_body(&page, &mut pdf,&page_id, i);
    }

    pdf.page(page_id).finish();
    let _ = file.write(&*pdf.finish());
    ()
}

fn save_article_as_pdf(article:&Document, path: &Path) {
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
    for article in &page.page {
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

mod html;

use std::fs::{copy, File};
use std::io::Write;
use pdf_writer::types::{BorderType, AnnotationFlags, AnnotationType};
use docx_rs::{*};
use std::path::{Path, PathBuf};
use html_to_pdf_lib::html_to_pdf;
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str, TextStr};
use pdf_writer::types::FieldType;
use pdf_writer::writers::Annotation;
use tauri::utils::tokens::path_buf_lit;
use crate::html::clean_and_set_song_font;
// Create a new handle

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Document{
    pub title: String,
    pub text: String,
    pub from_who: String,
    pub picture: Vec<String>
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Top{
    pub title: String,
    pub text: String
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Page{
    pub date_and_num:String,
    pub title: String,
    pub has_top: bool,
    pub top:Top,
    pub editors: String,
    pub page: Vec<Document>
}
#[tauri::command]
fn save_typesetting_as_pdf(page:&Page,path: &Path) {
    let mut file = std::fs::File::create(path).unwrap();
    let mut pdf = Pdf::new();
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let mut i=4;
    let mut headline_id:Ref;
    let mut title_annotation_id:Ref;
    let mut date_annotation_id:Ref;

    let mut title_id:Ref;
    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.pages(page_tree_id).kids([page_id]).count(1);
    let mut refs:Vec<Ref>= Vec::new();
    //如果有头版
    if page.has_top {
        //标题行
        title_annotation_id=Ref::new(i);
        i=i+1;
        let mut title_annotation = pdf.annotation(title_annotation_id);
        title_annotation.subtype(AnnotationType::Text);
        title_annotation.color_rgb(100.0, 0.0, 0.0);
        title_annotation.contents(TextStr("天津科技大学学报"));
        title_annotation.rect(Rect::new(161.5, 240.0, 100.0, 20.0));
        title_annotation.finish();

        //日期和版数
        date_annotation_id=Ref::new(i);
        i=i+1;
        let mut date_annotation:Annotation = pdf.annotation(date_annotation_id);
        date_annotation.subtype(AnnotationType::Text);
        date_annotation.color_rgb(100.0, 0.0, 0.0);
        date_annotation.contents(TextStr(page.date_and_num.as_str()));
        date_annotation.rect(Rect::new(161.5, 210.0, 200.0, 20.0));
        date_annotation.finish();
        title_id=Ref::new(i);

        //头版
        headline_id = Ref::new(i);
        i=i+1;
        refs.push(headline_id);
        let mut headline_field=pdf.form_field(headline_id);
        headline_field
            .partial_name(TextStr("headline"))
            .field_type(FieldType::Text)
            .text_value(TextStr(
                (page.top.text.len().to_string()+page.top.title.as_str()).as_str(),
            ));
        let mut headline_annot = headline_field.into_annotation();
        headline_annot.rect(Rect::new(793.7, 1133.80, 249.4, 340.1));
        headline_annot.border_style().style(BorderType::Solid);
        headline_annot.appearance_characteristics().border_color_rgb(0.0, 0.0, 0.0);
        headline_annot.flags(AnnotationFlags::PRINT);
        headline_annot.finish();
    }
    let mut y1:f32=1400.0;
    let x1:f32=161.5;
    let x2:f32=442.2;

    let num_of_articles=page.page.len();
    let y2:f32=637.7/(num_of_articles as f32) - 10.0;
    //文章
    for doc in page.page {
        let mut myref=Ref::new(i);
        i=i+1;
        let mut page_annotation:Annotation=pdf.annotation(myref);
        page_annotation.subtype(AnnotationType::Text);
        page_annotation.contents(TextStr(
            ((doc.title.len()+doc.text.len()).to_string()+doc.title.as_str()).as_str()
        ));
        page_annotation.rect(Rect::new(x1,y1-10.0,x2,y2));
        page_annotation.border_style().style(BorderType::Solid);
        page_annotation.appearance_characteristics().border_color_rgb(0.0, 0.0, 0.0);
        page_annotation.flags(AnnotationFlags::PRINT);
        y1+=637.7/(num_of_articles as f32);
    }
    let _ = file.write(&*pdf.finish());
    ()
}

fn save_article_as_pdf(article:&Document, path: &Path) {
    let html=clean_and_set_song_font(article.title.as_str(),
                                     article.from_who.as_str(), article.text.as_str());
    html_to_pdf(html.as_str(), path).unwrap();
}

#[tauri::command]
fn save(page: Page, path: &str){
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
            path_buf.set_extension(
                article.picture[i].split(".")[-1]
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

use crate::{save_typesetting_as_pdf, NewsPage, Top};

#[test]
fn test_write_typesetting(){
    let page:NewsPage=NewsPage{
        num_of_pages: 1,
        date_and_num: "2025-01-01".to_string(),
        title: "aaa".to_string(),
        has_top: true,
        top: Top{ 
            title: "aaa".to_string(),
            text: "eee".to_string(),
            words: 6
        },
        editors: "ccc ddd".to_string(),
        articles: vec![]
    };
    let path= std::path::Path::new("aaa.pdf");
    save_typesetting_as_pdf(&page, path);
}
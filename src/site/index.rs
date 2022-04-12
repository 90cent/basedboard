use super::utils::messageboxtemplate;
use crate::site::Template;

#[derive(serde::Serialize)]
struct TestContext {
    boxes: String,
} 

#[get("/")]
pub fn index() -> Template {
    let mut test_v = String::new();
    for bruh in 1..3 {
        let _box = messageboxtemplate::MessageBox::new("test test das ist ein test weh".into(),2);
        
        test_v.push_str(_box.to_html().as_str());
        println!("{}",test_v);
    }

    let context = TestContext {boxes: test_v};
    
    Template::render("index", &context)
}
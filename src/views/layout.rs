use askama::Template;

#[derive(Template)]
#[template(path = "template.html")]
pub struct LayoutTemplate {
    pub title: &'static str,
    pub rows: usize,
    pub cols: usize,
}

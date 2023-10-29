use askama::Template;

#[derive(Template)]
#[template(path = "template.html")]
pub struct TableSolution {
  pub header_text: String,
  pub solutions: Vec<[[(bool, usize); 8]; 8]>,
}
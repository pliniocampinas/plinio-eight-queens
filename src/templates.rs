use askama::Template;

#[derive(Template)]
#[template(path = "template.html")]
pub struct TableSolution {
  pub header_text: String,
  pub columns_positions: [isize; 8],
}
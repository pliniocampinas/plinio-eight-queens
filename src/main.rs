mod queens_solver;
mod templates;
use queens_solver::{
    MovesStack,
    fill_table,
    solve_queens,
};

use axum::{
    extract::Path,
    routing::get,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Router,
};
use askama::Template;
use std::net::SocketAddr;
use templates::TableSolution;

static THEME_CSS: &str = include_str!("../assets/theme.css");

async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "theme.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, THEME_CSS)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handle_main))
        .route("/table", get(table))
        .route("/_assets/*path", get(handle_assets));

    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn table() -> String {
    let mut table: [u8; 64] = [0; 64];

    solve_and_fill_table(&mut table);

    print_table(&table)
}

async fn handle_main() -> impl IntoResponse {
    let mut moves_stack: queens_solver::MovesStack = queens_solver::MovesStack::new();
    let mut results: Vec<MovesStack> = Vec::new();
    solve_queens(0, &mut moves_stack, &mut results);

    let template = TableSolution {
        header_text: String::from("Eight Queens"),
        columns_positions: results[0].columns_positions
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

fn solve_and_fill_table(table: & mut [u8; 64]) {
    let mut moves_stack: queens_solver::MovesStack = queens_solver::MovesStack::new();
    let mut results: Vec<MovesStack> = Vec::new();
    solve_queens(0, &mut moves_stack, &mut results);
    fill_table(table, &results[0]);

    println!("result_count {}", results.len());
}

fn print_table(&table: &[u8; 64]) -> String {
    println!("Eight Queens");
    let mut table_text = String::from("Eight Queens\r\n");

    println!("-------------- Table -------------- ");
    table_text += format!("-------------- Table -------------- \r\n").as_str();

    print!("x ");
    table_text += "x ";
    
    for i in 1..9 {
        table_text += format!("| {} ", i).as_str();
        print!("| {} ", i);
    }
    println!("|");
    table_text += "|\r\n";
    print!("-------------- ----- -------------- ");
    table_text += "-------------- ----- -------------- ";

    let mut index = 0;
    for elem in table {
        if index % 8 == 0 {
            println!();
            table_text+="\r\n";
            print!("{} |", (index/8 + 1));
            table_text+= format!("{} |", (index/8 + 1)).as_str();
        }
        index+=1;
        print!(" {} ", if elem > 0 { "Q" } else { "." });
        table_text+= format!(" {} ", if elem > 0 { "Q" } else { "." }).as_str();
        print!("|");
        table_text += "|";
    }
    println!();
    table_text += "\r\n";
    println!("----------- ---------- ------------ ");
    table_text += "----------- ---------- ------------ \r\n";
    return table_text;
}
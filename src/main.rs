mod queens_solver;
use queens_solver::{
    MovesStack,
    fill_table,
    solve_queens,
};

use axum::{
    routing::{get, post},
    http::StatusCode,
    // response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> String {
    let mut table: [u8; 64] = [0; 64];

    solve_and_fill_table(&mut table);

    print_table(&table)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
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
extern crate todo_cli;

fn main() {
    let app = todo_cli::TodoApp {};
    let args = std::env::args().collect();
    let output = app.run(args);

    println!("{}", output);
}

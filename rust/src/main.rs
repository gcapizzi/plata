extern crate todo_cli;

fn main() {
    todo_cli::TodoApp {}.run(std::env::args().collect());
}

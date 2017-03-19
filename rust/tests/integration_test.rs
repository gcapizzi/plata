extern crate todo_cli;

#[test]
fn it_adds_a_todo() {
    let writer_app = todo_cli::TodoApp {};
    writer_app.run(vec!["clear".to_string()]);
    writer_app.run(vec!["add".to_string(), "foo".to_string()]);
    writer_app.run(vec!["add".to_string(), "bar".to_string()]);

    let reader_app = todo_cli::TodoApp {};
    let list_output = reader_app.run(vec!["list".to_string()]);

    assert_eq!(list_output, "foo\nbar\n");
}

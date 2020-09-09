use hello::Task;

fn main() {
    let mut task = Task::new(1);
    println!("{}", task.index);

    task.index += 1;
}
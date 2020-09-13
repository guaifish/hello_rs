use deque;

fn main() {
    let (worker, stealer) = deque::new();

    // Only the worker may push/pop
    worker.push(1);
    println!("{:?}", worker);
    worker.pop();
    println!("{:?}", worker);

    // Stealers take data from the other end of the deque
    worker.push(1);
    println!("{:?}", worker);

    stealer.steal();
    println!("{:?}", worker);

    // Stealers can be cloned to have many stealers stealing in parallel
    worker.push(1);
    println!("{:?}", worker);

    let stealer2 = stealer.clone();
    stealer2.steal();
    println!("{:?}", worker);

}

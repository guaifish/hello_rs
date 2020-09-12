use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_vec: Rc<RefCell<Vec<isize>>> =
        Rc::new(RefCell::new(vec![1, 2, 3]));
    let shared1 = shared_vec.clone();
    let shared2 = shared1.clone();

    shared1.borrow_mut().push(4);
    shared2.borrow_mut().push(5);

    println!("{:?}", shared_vec);
}
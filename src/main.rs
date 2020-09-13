use fixed_vec_deque::FixedVecDeque;

fn main() {
    let mut q = FixedVecDeque::<[i32; 3]>::new();
    *q.push_back() = 1;
    *q.push_back() = 2;
    *q.push_back() = 3;
    *q.push_front() = 4;
    println!("{:?}", q);
}
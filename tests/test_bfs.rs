use std::collections::VecDeque;

#[test]
fn bfs(){
    let mut vec = vec![1, 2, 3, 4, 5];
    let mut queue = VecDeque::new();
    queue.push_back(11);
    queue.push_back(22);
    queue.extend(&vec);
    queue.pop_front();
    queue.pop_front();
    queue.pop_front();
    queue.pop_front();
    queue.pop_front();
}

#[test]
fn dfs(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.pop();
    vec.pop();
    vec.pop();
}
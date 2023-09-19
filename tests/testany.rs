#[test]
fn any(){
    let my_list = vec![1, 2, 3, 4, 5,6];
    let a = 3;
    let b = 6;
    
    let contains_a_or_b = my_list.iter().any(|&x| x == a || x == b);
    
    if contains_a_or_b {
        println!("列表中包含 a 或 b");
    } else {
        println!("列表中不包含 a 或 b");
    }
}

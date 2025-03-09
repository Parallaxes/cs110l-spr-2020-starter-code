use linked_list::{ComputeNorm, LinkedList};
pub mod linked_list;

fn main() {
    let mut list: LinkedList<String> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    list.push_front("a".to_string());
    list.push_front("b".to_string());
    list.push_front("c".to_string());
    list.push_front("d".to_string());

    // Test generics
    println!("===== Test generics =====");
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
    println!();

    // Test Clone trait
    println!("===== Test Clone trait =====");
    let mut list_copy = list.clone();
    list_copy.push_front("z".to_string());
    println!("original list : {}", list);
    println!("copied list : {}", list_copy);
    println!();

    // Test PartialEq
    println!("===== Test PartialEq trait =====");
    list.push_front("z".to_string());
    println!("original list : {}", list);
    println!("copied list : {}", list_copy);
    println!("original list == copied list is {}", list == list_copy);
    println!();

    // Test Iterator trait
    // If you implement iterator trait:
    println!("===== Test Iterator trait =====");
    for val in &list {
       println!("{}", val);
    }
    println!();

    // Test ComputeNorm
    println!("===== Test ComputeNorm =====");
    let mut f64_list: LinkedList<f64> = LinkedList::new();
    f64_list.push_front(5.0);
    f64_list.push_front(6.0);
    println!("{}", f64_list.compute_norm());
    println!();
}

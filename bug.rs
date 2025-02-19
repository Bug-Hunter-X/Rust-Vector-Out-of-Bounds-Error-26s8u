fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Length: {}", vec.len());
    let first = vec[0];
    println!("First element: {}", first);
    let second = vec[1];
    println!("Second element: {}", second);
    let third = vec[2]; //Error: index out of bounds
    println!("Third element: {}", third);
}
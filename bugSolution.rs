fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Length: {}", vec.len());
    let first = vec[0];
    println!("First element: {}", first);
    let second = vec[1];
    println!("Second element: {}", second);
    // Safe access using get()
    match vec.get(2) {
        Some(third) => println!("Third element: {}", third),
        None => println!("Index out of bounds"),
    };
    
    //Alternative using if let
    if let Some(third) = vec.get(2){
        println!("Third element: {}", third)
    } else {
        println!("Index out of bounds")
    }
} 
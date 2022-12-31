

fn main() {
    let x: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    for element in x.iter() {
        println!("x: {}", element);
    }

    for element in (1..10).rev() {
        println!("x: {}", element);
    }
}

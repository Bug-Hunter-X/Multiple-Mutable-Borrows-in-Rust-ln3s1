fn main() {
    let mut x = 5;
    {  //This block limits the scope of the borrow y
        let y = &mut x; 
        *y = 10; 
    }

    let z = &mut x; //This will be valid
    *z = 15;
    println!("x = {}", x);
}
fn is_even(num: i8) -> bool{
    if num%2 ==0{
        return true;
    }
    return false;
}

fn main() {
    let num = -17;
    println!("{}",is_even(num));
}

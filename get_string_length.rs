fn get_string_length(str: String) -> usize{
    //Note - we can run this even my keeping the argument &str instead of String . &str is a reference to the main string instead of sending the actual String.
    //Method 1
    // let mut length = 0;
    // for _char in str.chars(){
    //     length = length + 1;
    // }
    // return length;

    // Method 2: 
    // When we don't write return and semicolon , it is implicitky returning us the desired output.
    str.chars().count()
}

fn main(){
    let s = String::from("This is");
    println!("length of the string is: {} ",get_string_length(s));
}

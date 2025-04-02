//Just pass the address
fn main(){
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{}",s2);
    println!("{}",s1);
}

//Passing address/reference to the function
fn main(){
    let mystr = String::from("hello");
    borrows_variable(&mystr);
    println!("Initial variable mystr still has ownersip : {}",mystr);
}

fn borrows_variable(some_string: &String){
    println!("ownership is not transderred, it's just the reference: {}",some_string);
}


//Mutable References - Hanky Panky
fn main(){
    let mut s1 = String::from("Hello");
    println!("Original: {}",s1);
    update_word(&mut s1);
    println!("Updated: {}",s1);
}

fn update_word(s: &mut String){
    s.push_str("Rust");
}


//Case 4 : 
fn main(){
    let mut s1 = String::from("hello");
    println!("original : {}", s1);
    let mut s2 = &mut s1;
    update_word(s2);
    println!("after 1st mutable ref : {}",s2);
    let s3 = &mut s2;
    s3.push_str(" can we do this");
    println!("after 1st mutable ref of the 1st mutable ref: {}", s3);
}

fn update_word(s: &mut String){
    s.push_str(" RUST");
}

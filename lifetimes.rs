//Lifetimes

//An example that has nothing to do with lifetimes
// a function that takes 2 strings and returns the bigger string

// fn largest(a: String, b:String) -> String{
//     if a.len()>b.len(){
//         a
//     }
//     else{
//         b
//     }
// }

// fn main(){
//     let longest_str;  //just defining
//     let s1 = String::from("small");
//     {
//     let s2 = String::from("longer");
//     longest_str = largest(s1,s2);
//     }
//     //longest_Str was defined before the scope so it is still valid
//     //s2 is invalid as it is in the scope and gets vanished after it
//     //but we don't need s2 as s2 has now moved to longest_str.
//     println!("{}",longest_str);
// }


//Changing Syntax: 
// fn largest<'a>(str1: &'a str, str2: &'a str) -> &'a str{
//     //<'a> - generic lifetime syntax
//     // since all the variables have lifetime 'a , it does not means that they have the same lifetime
//     // when we give the same lifetime to all variables, then we force the compiler that
//     // " the return type will only be valid where both the parameters are valid , i.e. the intersection of both the vars."
//     if str1.len()>str2.len(){
//         str1
//     }
//     else{
//         str2
//     }
// }

// fn main(){
//     let longest_str;  //just defining
//     let s1 = String::from("small");
//     {
//     let s2 = String::from("longer");
//     longest_str = (&s1,&s2);
//     }
//     //longest_Str was defined before the scope so it is still valid
//     //s2 is invalid as it is in the scope and gets vanished after it
//     //but we don't need s2 as s2 has now moved to longest_str.
//     println!("{:?}",longest_str);
// }


//Lifetimes with Struct. Struct with references
// struct User <'a>{
//     name: &'a str,
// }


// fn main(){
//     let user;
//     {
//         let first_name = String::from("hello world");
//         user = User{name: &first_name};
//     }
//     //user should be valid here but now since we have defined a relationship between lifetimes of struct and variable
//     //user struct's lifetime is now attached to the lifetime of name variable. and since
//     //name variable points to first_name which gets cleared, so user also cannot be used after that.
//     println!("name: {}",user.name);
// }


use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display,
{
    println!("announcement! {ann}");
    if x.len()>y.len(){
        return x;
    }
    else{
        return y;
    }
}

fn main(){
    let s1 = String::from("Hello");
    let result;
    let s2 = String::from("World");
    result = longest_with_announcement(&s1, &s2, "Finding the longest string");
    println!("The longest string is: {}", result);
    //Currently this works but you can change the scope if you want and it will not work.
}

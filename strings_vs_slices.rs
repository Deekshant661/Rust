//Slices
//Write a function that takes a string and returns the first word

//1) this takes double memeory as we return a new string
//2) if the initial string gets cleared, the new string still has the first word in it.
// fn main(){
//     let str = String::from("Hello world");
//     let first = get_first_word(str);
//     println!("first word is : {}",first);
// }

// fn get_first_word(str: String) -> String{
//     let mut ans = String::new();
//     // for ch in str.chars(){
//     //     ans.push_str(ch.to_string().as_str());
//     //     if ch == ' '{
//     //         break;
//     //     }
//     // }

//     for ch in str.chars(){
//         if ch == ' '{
//             break;
//         }
//         ans.push_str(&ch.to_string());
//     }
//     return ans;
// }

//Method 2 : Slices
//We want a view to the part of the original string , we do not want the return a new string.
// fn main(){
//    let str = String::from("Hello world");
//    let first = get_first_word(&str);
//    println!("FIrst word : {}",first);
//    println!("original String: {}",str);
//    // the ownership is passed from str to first as it is like a immutable reference. 
//    //so ownership rules apply from now, we cannot have a mutable reference to string str.
//    // let second = get_first_word(&mut str);
// }

// fn get_first_word(str: &String) -> &str{
//     let mut index: usize = 0;
//     for (_, i) in str.chars().enumerate(){
//         if i == ' '{
//             break;
//         }
//         index = index + 1;
//     }
//     return &str[0..index];
// }


//Types of Strings
// fn main(){
//     let s1 = String::from("Hello World");
//     let s_slice = &s1; 
//     let string_literal = "Hello World"; //it is a literal so it is also an &str and it  directly points to the address 
//     //in the binary file.
// }

Also applicable to other types of collections as well like arrays, vectors etc etc.
fn main(){
    let arr = [1,2,3];
    let arr_slice = &arr[0..1];
    print!("{:?}",arr_slice);
}

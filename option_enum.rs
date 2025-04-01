// fn find_first_a(s: String) -> Option<i32>{
//     for (index,character) in s.chars().enumerate(){
//         if character == 'a'{
//             return Some(index as i32);
//         }
//     }
//     return None;
// }


// fn main(){
//     let my_str = String::from("deekshantaa");
//     let res = find_first_a(my_str);

//     match res{
//         Some(Index) => println!("The letter a is found at: {}",Index),
//         None => println!("there is no a in the string"),
//     }
// }


//Applying our own custom Enum.
enum CustomOption{
    Some(i32),
    None,
}

fn find_first_a(str: String) -> CustomOption{
    for (index,char) in str.chars().enumerate(){
        if char == 'a' {
            return CustomOption::Some(index as i32);
            //return the some variant of the enum Option
        }
    }
    return CustomOption::None;
}


fn main(){
    let index: CustomOption = find_first_a(String::from("Deekshant"));

    match index{
        CustomOption::Some(value) => println!("index is {}",value),
        CustomOption::None => println!("a not found"),
    }
}


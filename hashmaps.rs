//Hashmaps
use std::collections::HashMap;

// fn main(){
//     let mut hm: HashMap<String,u32> = HashMap::new();
    
//     hm.insert(String::from("deekshant"),20);
//     hm.insert(String::from("Harkirat"),22);

//     let first_user_age = hm.get("deekshant");

//     match first_user_age {
//         Some(age) => println!("age is {}",age),
//         None => println!("Key was not found in the hashmap"),
//     }
// }

//Create a function that takes a vector of a tuple containing String and number, and returns a hashmap with key as the string and value as the number of the hashmap.
fn get_values_from_vector(vec: Vec<(String,i32)>) -> HashMap<String,i32>{
    let mut hm: HashMap<String,i32> = HashMap::new(); 
    for (key,value) in vec{
        hm.insert(key,value);
    }
    return hm;
}

fn main(){
    let vec: Vec<(String,i32)> = vec![(String::from("Deekshant"),20),(String::from("Harkirat"),22)];
    let ans = get_values_from_vector(vec);
    println!("the hashmap with the values of vector of a tuple: {:?}",ans);
}

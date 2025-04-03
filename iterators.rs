//Iterators
// fn main(){
//     let vec: Vec<i32>  = vec![1,2,3];
//     //This is actually same as the normal for loop but under the hood it creates a type of iterator and then iterates over it.
//     let vec_iter = vec.iter();

//     for val in vec_iter{
//         println!("{}",val);
//     }

//     //iterator only borrows the values, it does not takes the ownership so the original vector is still valid.
//     print!("{:?}",vec);
// }

//Mutable Iterator
// fn main(){
//     let mut v1: Vec<i32> = vec![1,2,3];
//     let v1_iter = v1.iter_mut();

//     for val in v1_iter{
//         *val = *val + 1;
//     }
    
//     println!("new vector: {:?}",v1);
// }


// //Iterator actual working under the hood
// fn main(){
//     let v1: Vec<i32> = vec![1,2,3];
//     let mut v1_iter = v1.iter();

//     while let Some(value) = v1_iter.next() {
//         println!("{}",value);
//     }
// }

//Consuming Adaptors
// fn main(){
//     let v1: Vec<i32> = vec![1,2,3];
//     let v1_iter = v1.iter();
//     let sum: i32 = v1_iter.sum();

//     println!("{}",sum);

//     println!("Original vector iterator gets removed: {:?}",v1_iter);
// }


//Iterative Adapters
// 1. map
// fn main(){
//     let v1: Vec<i32> = vec![1,2,3];
//     let v1_iter = v1.iter();
//     let v1_iter2 = v1_iter.map(|x| x+1);
    
//     for i in v1_iter2{
//         println!("{}",i);
//     }
    
//     print!{"{:?}",v1};
// }

// 2. Filter
// fn main(){
//     let v1: Vec<i32> = vec![1,2,3];
//     let v1_iter = v1.iter();
//     let v1_iter2 = v1_iter.filter(|x| *x%2 ==0);  //returns even elements of the vector

//     for i in v1_iter2{
//         println!("{}",i);
//     }

//     println!("{:?}",v1);
//     //v1_iter does get consumed here
// }


//Assignment : Logic to first filter odd values then double the value and create a new vector

//Approach 1 : BUt new vector is not created here.
// fn main(){
//     let v1: Vec<i32> = vec![1,2,3];
//     let v1_iter = v1.iter();
//     let v1_iter2 = v1_iter.filter(|x| *x%2 ==1).map(|x| x*2);  //returns odd elements of the vector


//     for i in v1_iter2{
//         println!("{}",i);
//     }

//     println!("{:?}",v1);
// }

//Approach 2 : New vector is created. 

// fn filter_and_map(v: Vec<i32>) -> Vec<i32>{
//     let iter = v.iter().filter(|x| *x%2 == 1).map(|x| x*2);
//     let v2: Vec<i32> = iter.collect();
//     return v2;
// }
// fn main(){
//     let v1: Vec<i32> = vec![1,2,3,4,5];
//     let ans = filter_and_map(v1);
//     println!("{:?}",ans);
// }


//Vector ->  hashmap -> iterator  ->  hashmap -> vector
use std::collections::HashMap;
fn main(){
    let v1: Vec<(String,i32)> = vec![(String::from("Deekshant"),21),(String::from("Harkirat"),22)];
    println!("Vector of tuples: {:?}",v1);

    let v1_iter = v1.into_iter();

    let mut hm: HashMap<String,i32> = HashMap::new();

    for (key,value) in v1_iter{
        hm.insert(key,value);
    }

    println!("HashMap created from vector : {:?}",hm);

    //HashMap can only be generated using into_iter()
    let hm_iter = hm.into_iter();
    let hm2: HashMap<String,i32> = hm_iter.collect();
    println!("Hashmap from iterator: {:?}",hm2);

    //Vector can be generated with normal iter
    let hm2_iter = hm2.iter();
    let v2: Vec<(String,i32)> = hm2_iter.map(|(k,v)| (k.clone(),*v)).collect();
    println!("Vector from Hashmap: {:?}",v2);
}



//Vector ->  hashmap -> iterator  ->  Vector -> Hashmap
// use std::collections::HashMap;
// fn main(){
//     let v1 = vec![(String::from("Deekshant"),20),(String::from("Harkirat"),22)];
//     println!("Intial Vector: {:?}",v1);

//     //for converting vec to hashmap, we use into_iter() to transfer ownership
//     let v1_iter = v1.into_iter();
//     let hm: HashMap<String,i32> = v1_iter.collect();
//     println!("Hashmap generated from vector: {:?}",hm);

//     // now we convert hashmap to iterator
//     let hm_iter = hm.iter();

//     //iterator to vector
//     let v2: Vec<(String, i32)> = hm_iter.map(|(k,v)| (k.clone(),*v)).collect();
//     println!("Vector generated from iterator: {:?}",v2);

//     //Converting vector to hashmap , we convert vector to iterator to hashmap
//     let hm2: HashMap<String,i32> = v2.into_iter().collect();
//     println!{"Hashmap again from vector: {:?}",hm2};
// }

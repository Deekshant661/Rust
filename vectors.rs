//Vectors
// fn main(){
    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);

    //Intialising using macros
    // Explicity specifying the type
    // let vec: Vec<i32> = vec![1,2,3];
    // println!("{:?}",vec);
    // let ans = even_filter(&vec);
    // println!("even numbers of vec: {:?}",ans);
    // println!("can we use vec again without passing &vec to tthe even_filter? No: {:?}",vec);
// }

//Write a function that takes a vector as input and returns a vevtor with even values

//Approach 1:
// fn even_filter(vec: &Vec<i32>) -> Vec<i32>{
//     let mut vec2 = Vec::new();
//     for val in vec{
//         if val%2 == 0{
//             vec2.push(*val);
//         }
//     }
//     return vec2;
// }


//Approach 2
fn main(){
    let mut vec: Vec<i32> = vec![1,2,3];
    let ans = even_filter2(&mut vec);
    println!("only even elements: {:?}",ans);
}

fn even_filter2(v: &mut Vec<i32>) -> Vec<i32>{
    let mut i = 0;
    while i < v.len(){
        if v[i] %2 !=0{
            v.remove(i);
        }
        else{
            i += 1;
        }
    }
    return v.to_vec();
}

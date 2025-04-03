//Generics 
// struct Point<T>{
//     x: T,
//     y: T,
// }

// fn main(){
//     let integer_point = Point{x: 10, y:5};
//     let float_point = Point{x: 5.00,y: 6.00};
//     println!("Integer Point: {} {}",integer_point.x, integer_point.y);
//     println!("Float Point: {} {}",float_point.x, float_point.y);
// }


//Generics
fn main(){
    let bigger = largest(1,2);
    let bigger2 = largest("harkirat","deekshant");
    println!("{}",bigger);
    println!("{}",bigger2);
}


fn largest<T: std::cmp::PartialOrd>(a:T, b:T) -> T{
    if(a>b){
        return a;
    }
    else{
        return b;
    }
}
  

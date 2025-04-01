// struct User{
//     active: bool,
//     username: String,
//     age: i8,
//     sign_in_count: i32,
// }

// fn main(){
//     let user1 =  User {
//         active : true,
//         username : String::from("some username"),
//         age : 30,
//         sign_in_count : 10,
//     };

//     println!("User1 username: {:?}",user1.username);
// }


//Implementing Structs

struct Rect{
    width: i32,
    height: i32,
}

impl Rect{
    // this is like the methods inside the class in JS
    // &self is like the this keyword, it contains the Struct which is currently being used.
    fn area(&self) -> i32{
        self.width * self.height
        //return self.width * self.height;
    }
    
    fn perimeter(&self,num: i32) -> i32{
        2*(self.width+self.height)
    }

    //static functions : functions without arguments, and these are called using Structs and not their objects.
    fn debug() -> i32{
        return 1;
    }
    
}

fn main(){
    let rect1 = Rect{
        width: 12,
        height: 10,
    };

    //We don't have to mention self as paramter as it is already reserved as the first parameter.
    println!("The area of the rectangle is: {}",rect1.area());
    // Here the first argument is already reserved for self and hence it only takes the second argument which is the number 1.
    println!("the perimeter func takes 2 arguments {}",rect1.perimeter(1));
    println!("static function debug: {}",Rect::debug());
}

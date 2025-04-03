//Traits

trait Summary{
    fn summarize(&self) -> String{
        return String::from("some output from summarize");
    }
}

trait Fix{
    fn fix(&self) -> String{
        return String::from("some output from fix");
    }
}

struct User{
    name: String,
    age: u32,
}


impl Summary for User{
    // fn summarize(&self) -> String {
    //     return format!("Name is {} and age is {}",self.name,self.age);
    // }
}
impl Fix for User{}

impl Summary for String{}


//traits as parameters
//notify only accepts input that implement summary
fn notify<T: Summary + Fix>(u: T){
    println!("Anyone who implements Summary and Fix: {} {}",u.summarize(),u.fix());
}

fn main(){
    let user = User{
        name : String::from("harkirat"),
        age : 22,
    };

    notify(user);
    //notify(String::from("Any String as I have implemented SUmmary for the String struct"));
    //gives error as String only satisfies the summary trait.
}

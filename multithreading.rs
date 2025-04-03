//Multithreading
// use std::thread;
// use std::time::Duration;

// fn main(){
//     thread::spawn(|| {
//         for i in 1..10{
//             println!("hi number {i} from the spawed thread");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5{
//         println!("hi number {i} from the main thread");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// fn main(){
//     let handle = thread::spawn(||{
//         for i in 1..10{
//             println!("hi number {i} from the spawned thread");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
    
//     handle.join().unwrap();

//     // this make the main thread wait until the spawned thread is finished. 
//     for i in 1..5{
//         println!("Hi number {i} from the main thread");
//         thread::sleep(Duration::from_millis(1));
//     }
// }


//move closure
// fn main(){
//     let x = 1;
//     {
//         let v = vec![1,2,3];
//         thread::spawn(move || {
//             println!("{:?}",v);
//         });
//         //println!("{:?}",v);
//     }
//     println!("{}",x);

//     //when the last line is running, it means that v has already gone out of scope.
//     //we don't know when the code in the thread will run.
//     // that's why we pass the ownerhsip of v to the thread, so that it runs with the thread. only thread can use v now.
// }



//Message Passing
// use std::{
//     sync::mpsc, //multiple producer single consumer
//     thread::{self,spawn},
// };

// fn main(){
//     let (tx,rx) = mpsc::channel();
//     spawn(move ||{
//         tx.send(String::from("Hello World"));
//     });

//     let value = rx.recv();
//     match value{
//         Ok(value) => println!("value : {}",value),
//         Err(err) => println!("Error while reading the value : {}",err),
//     }
// }



use std::sync::mpsc;
use std:: thread::{self,spawn};
fn main(){
    let (tx,rx) = mpsc::channel();

    for i in 1..10{
        let producer = tx.clone();
        spawn(move ||{
            let mut sum: u64 = 0;
            for j in i*10000000..(i+1)*10000000-1{
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);

    let mut final_val = 0;
    for val in rx{
        println!("receiving value from thread");
        final_val = final_val + val;
    }
    println!("final sum: {}",final_val);
    //receiver does not know that the producer has finished.
    // it knows that the cloned producers are finished. but the original producer tx when we defined was never moved.
    //tx ( first transmitter) was never moved to a different thread. which means it is still active.
    //so here we need to explicity drop this tx.
    //or we can try to move the first one to a new thread.
}

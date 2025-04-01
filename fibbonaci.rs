fn fib(n: u32) -> u32{
    if n==0 || n==1{
        return n;
    }
    let mut prev1 = 0;
    let mut prev2 = 1;
    let mut curr = 0;
    for _i in 2..n+1{
        curr = prev1+ prev2;
        prev1 = prev2;
        prev2 = curr;
    }
    return curr;
}

fn main(){
    let num = 4;
    println!("{}",fib(num));
}

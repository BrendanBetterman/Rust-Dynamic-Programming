fn fib(num: u64, memo: &mut [u64])-> u64{
    if num <=1{
        return 1
    }else {
        if memo[num as usize] ==0{
            memo[num as usize]= fib(num -1,memo) + fib(num -2,memo);
        }
    }
    return memo[num as usize];
}
fn main() {
    let mut memo: [u64;100] = [0;100];

    println!("{}",fib(5,&mut memo));
    println!("{}",fib(8,&mut memo));
    println!("{}",fib(10,&mut memo));
    println!("{}",fib(15,&mut memo));
    println!("{}",fib(30,&mut memo));
    println!("{}",fib(90,&mut memo));
    println!("Hello, world!");
}

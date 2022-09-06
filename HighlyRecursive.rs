//https://www.codechef.com/submit/MICS2022_P2
fn h(n: i32, memo: &mut [i32])->i32{
    if memo[(n+75) as usize] ==0{
        if n < -5{
            memo[(n+75) as usize] =  h(n+4,memo) + h(n+2,memo);
        }else if n<2{
            memo[(n+75) as usize] =  n*2;
        }else{
            memo[(n+75) as usize] =  h(n-8,memo) - h(n-4,memo) + h(n-3,memo);
        }
    }
    return memo[(n+75) as usize];
}
fn main() {
    let mut memo: [i32; 171] = [0;171];
    println!("{}",h(-75,&mut memo));
    println!("{}",h(95,&mut memo));
    println!("Hello, World!");
}

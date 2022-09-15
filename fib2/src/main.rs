use std::time;

fn fib(helper: &mut [u128; 3], mut n: u32) -> u128 {
    // We could just assign helper in the following way
    // helper = [1, 1, 2];
    // But this might mean creating a new array and assigning the memory address to helper.
    // This would mean waiting for memory to free a new slot in memory.
    // Which is a lot slower than just letting the cpu assign the value directly.

    helper[0] = 1;
    helper[1] = 1;
    helper[2] = 2;

    if n == 1 { 
        return 1;
    };

    if n == 2 {
        return 1
    };

    if n == 3 { 
        return 2; 
    };

    loop {
        helper[0] = helper[1] + helper[2];
        // We're reusing n memory spot for the counter counting backwards
        n -= 1;
        
        // We are counting after 1 1 2. So we are 3 steps ahead already.
        if n == 3 { return helper[0]; }

        helper[1] = helper[0] + helper[2];
        n -= 1;

        if n == 3 { return helper[1]; }

        helper[2] = helper[0] + helper[1];
        n -= 1;

        if n == 3 { return helper[2]; }
    }
}


// Just a quick reminder that this implementation is not just calculating fib(n).
//
// It calculates all the fibonacci numbers starting from 1 to n.
// 
// So if n = 5 it prints 1 1 2 3 5 instead of just 5.
fn main() {
    let mut helper: [u128; 3] = [1, 1, 2];

    // More than 186 will cause an integer overflow in u128.
    let max: u32 = 186;

    // Jump 2 lines
    println!("\n");

    let now = time::Instant::now();

    // Count the first {max} fibonacci numbers.
    for i in 1..(max + 1) {
        print!("{} ", fib(&mut helper, i));
    }

    let elapsed_time = now.elapsed().as_nanos();

    println!("\n");

    println!("Finished counting {} fibonacci numbers in {} nanoseconds.", max, elapsed_time);
}

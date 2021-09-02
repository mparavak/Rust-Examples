use simple::fact;

use simple::fib;

use simple::vector;

fn main() {
    //2. Write a function to find factorials for a given n 
    let n = 10 ; 
    println!("Factorial of {} = {} ",n,fact::factorial(n)) ; 

    // 3. Write a function to generate 'n' numbers from fibonacci series
    
    let k = 10 ;
    fib::fibonacci(k);


    // Mean using vector 
    let v = vec!(1,2,3,4,5);
    println!("Mean  of list of numbers ==> {}",vector::get_mean(v));
}

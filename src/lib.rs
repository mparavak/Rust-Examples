pub mod fact {
    pub fn factorial(mut i : i64) -> i64 {
        let mut result : i64 = 1 ; 
        while i > 1 {
            result = result*i ;
            i = i -1 ;
        }
        
        return result
    }
}


pub mod fib ;

pub mod vector; 
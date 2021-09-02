
    pub fn fibonacci(k : i64) {
        let mut first = 0 ;
        let mut second = 1 ;
        let mut out = 0 ;
        let mut count = 0 ;
        println!("Geberating {} numbers from Fib series",k);
        while count < k {
            count = count + 1 ;
            
            println!("{}",first) ;
            
            out = first + second ; 
            
            first = second ;
            second = out ;
        }
    }
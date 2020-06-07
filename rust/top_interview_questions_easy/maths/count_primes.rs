/******************************************************************************

Count Primes

Count the number of prime numbers less than a non-negative number, n.

 ******************************************************************************/

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if (n == 0 || n == 1){
            return 0;
        }
        
        let mut sieve: Vec<i32> = (0..n).collect();
        
        'outer: for i in 2..((n as f64).sqrt().ceil() as usize){
            if sieve[i] != -1 {
                let mut count = 0;
                'inner: loop {
                    if (i*i + count*i < sieve.len()){
                        sieve[i*i + count*i] = -1;
                        count = count + 1;
                    }
                    else {
                        break 'inner;
                    }
                }
            }
        }
        return sieve.iter().filter(|&c| *c != -1).count() as i32 - 2;        
    }
}

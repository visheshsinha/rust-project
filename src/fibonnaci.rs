
use std::collections::HashMap;
pub struct Fibonnaci;
impl Fibonnaci {
    pub fn fib(&self, n: i32) -> i32 {
        let mut cache = HashMap::new();        
        fn solve(num: i32, cache: &mut HashMap<i32, i32>) -> i32 {
            
            if num < 2 {
                return num
            }
            
            let value = cache.get(&num);
            
            match value {
                Some(found) => return *found,
                None => {
                    let temp = solve(num - 1, cache) + solve(num - 2, cache);
                    cache.insert(num, temp);
                    return temp
                }
            }
        }

        let answer: i32 = solve(n, &mut cache);
        
        return answer
    }
}
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Instant;

fn main(){

    // the gcd of 2 nums (x and y) is the same as the product of the intersection between (the prime factorials of x) and (the prime factorials of y)
    // obviously this will take more computational time than gcd, if the set of "the intersection" is rep with map
    let now = Instant::now();
    let p1:Prime_factorials = simple_trial_division_factorization(17060040);
    let p2:Prime_factorials = simple_trial_division_factorization(82068744090);
    let res: Prime_factorials = p1.intersect(&p2);
    let elapsed = now.elapsed().as_nanos();
    println!("{:?}", res);
    println!("The avg time elapsed is: {} Ns.", elapsed);
}


#[derive(Eq, PartialEq, Debug)]
struct Prime_factorials(
    HashMap<u8, u8>,   // map of all the prime factorials which can be represented with u8 ( nr less than 256)
     HashMap<u32, u8>, // map of all the prime factorials which can be represented with u32 ( nr less than 2^32)
     HashMap<u64, u8>  // map of all the prime factorials which can be represented with u64 ( nr less than 2^64)

     /* Example:
        Prime_factorials of 10      ({2:1 , 5:1},{},{})  2 x 5
        Prime_factorials of 6500    ({2:2 , 5:3 , 13:1},{},{}) 2 x 2 x 5 x 5 x 5 x 13
        Prime_factorials of 1421670 ({2:1 , 3:1 , 5:1},{47389:1},{}) 2 x 3 x 5 x 47389

      */
);
impl Prime_factorials{

    fn new() -> Self {
        Self (  HashMap::new(), HashMap::new(), HashMap::new() )
    }

    fn intersect(&self , other: &Prime_factorials) -> Prime_factorials {
        let mut res = Prime_factorials::new();
        res.0 = get_intersection(&self.0, &other.0);
        res.1 = get_intersection(&self.1, &other.1);
        res.2 = get_intersection(&self.2, &other.2);
        res
    }

}

fn simple_trial_division_factorization(
    n: u64,
) -> Prime_factorials {
    let mut n = n;

    let mut res: Prime_factorials =
    Prime_factorials::new();

    while n % 2 == 0 {
        let count = res.0.entry(2).or_insert(0);
        *count += 1;

        n /= 2;
    }

    let mut inc: u64 = 3;
    while inc * inc <= n {
        while n % inc == 0 {
            match inc {
                x if inc < (u8::max_value() as u64 + 1) => {
                    let count = res.0.entry(inc as u8).or_insert(0);
                    *count += 1;
                }
                x if inc < (u32::max_value() as u64 + 1) => {
                    let count = res.1.entry(inc as u32).or_insert(0);
                    *count += 1;
                }
                x if inc < (u64::max_value() + 1) => {
                    let count = res.2.entry(inc).or_insert(0);
                    *count += 1;
                }
                _ => (),
            }

            n /= inc;
        }
        inc += 2;
    }

    // This condition is to handle the case when n is a prime number
    if n > 1 {
        match n {
            x if n < (u8::max_value() as u64 + 1) => {
                let count = res.0.entry(n as u8).or_insert(0);
                *count += 1;
            }
            x if n < (u32::max_value() as u64 + 1) => {
                let count = res.1.entry(n as u32).or_insert(0);
                *count += 1;
            }
            _ => {
                //x if n <= (u64::max_value())
                let count = res.2.entry(n).or_insert(0);
                *count += 1;
            }
        }
    }

    res
}


fn get_intersection<K, V>(a: &HashMap<K, V>, b: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Debug + Eq + Hash + Copy,
    V: Debug + Ord + Copy,
{
    let shortest: &HashMap<_, _>;
    let other: &HashMap<_, _>;
    let mut intersection: HashMap<K, V> = HashMap::new();

    if a.len() < b.len() {
        shortest = a;
        other = b;
    } else {
        shortest = b;
        other = a;
    }

    for (key, value) in shortest {
        if let Some(val) = other.get(key) {
            let _v = std::cmp::min(*val, *value);
            intersection.insert(*key, _v);
        }
    }

    intersection
}

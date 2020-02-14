use std::time::Instant;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    // Read all the test cases at once
    let arr: Vec<(Vec<u64>, Vec<usize>)> = read_lines_fm_file("src/test_data.rs");

    let mut avg_time = 0;

    for _ in 0..1000 {
        let now = Instant::now();
        for (data, max_gcd_indicies) in arr.iter() {
            for max_gcd_ind in max_gcd_indicies {

                // IMPORTANT: what if there is more than one num with the same value of max_gcd i.e. [3, 6, 2, 2, 6, 2] 
                // in the above example we have 2 elms that are candidate to be a start point. 
                // if we start at index 1 the fst 6     ==> that will yield magic gcd = 1 x 6 or 3 x 2 = 6
                // but if we start at index 4 the snd 6 ==> that will yield magic gcd = 2 x 4 = 8 which is the right answer.
                // sol: we can find magic gcd of all possible start points and take the largest,, see the use of multithreading to optimize this solution.  
                println!("{:?}", find_mg(&data, *max_gcd_ind));
            }
        }
        let elapsed = now.elapsed().as_nanos();
        avg_time += elapsed;
    }

    println!("The avg time elapsed is: {} Ns.", avg_time / 1000);
}
fn read_lines_fm_file(path: &str) -> Vec<(Vec<u64>, Vec<usize>)> {
    // the path can be "src/tarifa/examples/tarifa.2.ans"
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    let mut test_cases: Vec<(Vec<u64>, Vec<usize>)> = vec![];

    for (i, line) in f.lines().enumerate() {
        if i % 2 != 0 {
            continue;
        }

        if i == 0 {
            continue;
        }

        let line = line.expect(&format!("Unable to read the {}. line to string", i));
        let mut max_gcd_value: u64 = 0;
        let mut max_gcd_ind: Vec<usize> = vec![];

        let v = line
            .trim()
            .split_whitespace()
            .enumerate()
            .map(|(i, num)| {
                let num: i64 = num.parse().unwrap();
                let num: u64 = num.abs() as u64;
                if num > max_gcd_value {
                    max_gcd_value = num;
                    max_gcd_ind.clear();
                    max_gcd_ind.push(i);
                } else if num == max_gcd_value {
                    max_gcd_ind.push(i);
                }
                num
            })
            .collect();

        test_cases.push((v, max_gcd_ind));
    }
    test_cases
}


pub fn find_mg(data: &[u64], max_gcd_ind: usize) -> (u64, usize, usize) {
    // println!("max_gcd_ind:{}", max_gcd_ind);
    let mut p: usize = max_gcd_ind;
    let mut q: usize = max_gcd_ind;
    let mut next_ind: usize;

    let mut mg_m = data[max_gcd_ind];
    let mut mg_r = data[max_gcd_ind];
    let mut mg_l = data[max_gcd_ind];

    let mut mg_so_far = data[max_gcd_ind];
    let mut mg_len = 1;

    let mut next_ind_r: usize = max_gcd_ind;
    let mut next_ind_l: usize = max_gcd_ind;

    /**************************************************************/

    for i in 1..data.len() {
        // Starts at "max_gcd_ind" and "data[next_ind]"; next_ind starts form the right-adjecent elm and keep moving right until end of list, and after that moves to the left-adjecent elm of the "max_gcd_ind" and keep moving left to the start of the list
        if i + max_gcd_ind < data.len() {
            next_ind_r += 1;
        } else {
            next_ind_r = data.len() - 1 - i;
        }
        mg_r = gcd(mg_r, data[next_ind_r]);
        /**************************************************************/
        // Starts at "max_gcd_ind" and "data[next_ind]"; next_ind starts form the left-adjecent elm and keep moving left until end of list, and after that moves to the right-adjecent elm of the "max_gcd_ind" and keep moving right to the start of the list
        if max_gcd_ind + data.len() - i >= data.len() {
            // eqv to (max_gcd_ind - i  >= 0); but when (max_gcd_ind - i)< 0 the code will panic due to "compairing neg" index and usize
            next_ind_l -= 1;
        } else {
            next_ind_l = i;
        }
        mg_l = gcd(mg_l, data[next_ind_l]);
        /**************************************************************/

        // Starts at "max_gcd_ind" and "data[next_ind]"; next_ind can be either (to the left) or (to the right), so choose to go in the direction of the largest adjecent next value
        if p > 0 && q < data.len() - 1 {
            if data[p - 1] > data[q + 1] {
                p -= 1;
                next_ind = p;
            } else {
                q += 1;
                next_ind = q;
            }
        } else if q < data.len() - 1 {
            q += 1;
            next_ind = q;
        } else {
            p -= 1;
            next_ind = p;
        }
        mg_m = gcd(mg_m, data[next_ind]);

        /**************************************************************/

        mg_len += 1;
        let mg_temp = my_max(mg_m, mg_r, mg_l) * mg_len;

        if mg_temp >= mg_so_far {
            mg_so_far = mg_temp;
        }
    }

    return (mg_so_far, 0, 0);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

fn my_max(a: u64, b: u64, c: u64) -> u64 {
    max(max(a, b), c)
}
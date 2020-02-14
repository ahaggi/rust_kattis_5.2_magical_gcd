use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread::{self, JoinHandle};

// The idea was:
// 1- Read the "input_list" with len N, and calc a gcd for each adjecent numbers that will produce a list "gcd_list" with len (N-1)
// 2- To produce a magic gcd, we start with the largest value in "gcd_list" "X" with index "i", and from there :
//     a- fold to the left from "i" to "gcd_list.len -1" and continue with fold to the right from "i" to the index = 0.
//     b- fold to the right from "i" to the index = 0 and continue with fold to the left from "i" to "gcd_list.len -1".
//     c- starting from index "i" take the largest adjecent elem (either left or right) and after that take the next largest elm.
// 3- magic gcd = max (2.a, 2.b, 2.c)

// The problem with starting from (largest value in "gcd_list") "X", is that if "gcd_list"  a contiguous subarray with a lesser value "Y" AND the product of "Y" and the len of that subarray is the magic gcd. 
// * consider  
// input_list = [60, 30, 20, 20, 20, 20, 20] will produce
// gcd_list =     [30, 10, 20, 20, 20, 20]
// the largest value in "gcd_list" is "X" = 30 with index 0
// if we start from 30 this will give us a result = 70

// BUT
// the magic gcd is 20 * 5 = 100
fn read_input() -> (Vec<u64>, Vec<usize>) {

    
    let stdin = io::stdin();
    let mut seq_len = String::new();

    stdin.read_line(&mut seq_len).expect("Failed to read line");
    let seq_len: usize = seq_len.trim().parse().unwrap();

    let mut data_str = String::new();
    stdin.read_line(&mut data_str).expect("Failed to read line");

    let mut data: Vec<u64> = Vec::with_capacity(seq_len);
    let mut max_gcd_value: u64 = 0;
    let mut max_gcd_indicies: Vec<usize> = vec![];

    let mut iter = data_str.trim().split_whitespace();

    let mut temp: u64 = iter.next().unwrap().parse().unwrap();

    if seq_len > 1 {
        for i in 0..seq_len - 1 {
            let input: u64 = iter.next().unwrap().parse().unwrap();
            let num = gcd(temp, input);

            data.push(num);

            temp = input;

            if num > max_gcd_value {
                max_gcd_value = num;
                max_gcd_indicies.clear();
                max_gcd_indicies.push(i);
            } else if num == max_gcd_value {
                max_gcd_indicies.push(i);
            }
        }
    } else {
        data.push(temp);
        max_gcd_indicies.push(0);
    }

    // data
    //     .trim()
    //     .split_whitespace()
    //     .enumerate()
    //     .map(|(i, num)| {
    //         num
    //     })
    //     .collect();

    (data, max_gcd_indicies)
}

fn main() {
    let stdin = io::stdin();

    let mut nr_of_test_cases = String::new();
    stdin
        .read_line(&mut nr_of_test_cases)
        .expect("Failed to read line");
    let nr_of_test_cases: usize = nr_of_test_cases.trim().parse().expect("Failed to usize");

    for _ in 0..nr_of_test_cases {
        let (data, max_gcd_indicies) = read_input();
        let mut mg: u64 = 0;
        if max_gcd_indicies.len() == data.len() {
            let g: u64 = data[0];
            let len: u64 = 1 + data.len() as u64;
            match g.checked_mul(len) {
                Some(res) => mg = res,
                _ => infinite_loop(),
            }
        } else {
            let data_arc = Arc::new(data);
            let mg_arc = Arc::new(Mutex::new(0u64));
            let mut i = 0;

            while i < max_gcd_indicies.len() {
                let mut join_handles: Vec<JoinHandle<()>> = Vec::with_capacity(50);
                let mut j = 0;
                while i < max_gcd_indicies.len() && j < 50 {
                    if i != 0 && max_gcd_indicies[i] == max_gcd_indicies[i - 1] + 1 {
                        i += 1;
                        continue;
                    }
                    let _data_arc = Arc::clone(&data_arc);
                    let _mg_arc_clone: Arc<Mutex<u64>> = Arc::clone(&mg_arc);

                    let max_gcd_ind = max_gcd_indicies[i];
                    // println!("{:?}", _data_arc);
                    // println!("{:?}", max_gcd_indicies);

                    let t = thread::spawn(move || {
                        find_mg_flag(&_data_arc, max_gcd_ind, _mg_arc_clone);
                    });
                    join_handles.push(t);
                    i += 1;
                    j += 1;
                }

                for handle in join_handles {
                    handle.join().unwrap();
                }
            }

            match mg_arc.lock() {
                Ok(n) => mg = *n,
                _ => infinite_loop(),
            };
        }
        println!("{}", mg);
    }
}
fn infinite_loop() {
    let mut t = 10;

    while t != 0 {
        t = gcd(t, 10);
    }
}

/***************************************************************************************/
/***************************************************************************************/
pub fn find_mg_flag(data: &[u64], max_gcd_ind: usize, shared_mg: Arc<Mutex<u64>>) {
    let mut p: usize = max_gcd_ind;
    let mut q: usize = max_gcd_ind;
    let mut next_ind: usize;

    let mut next_ind_r: usize = max_gcd_ind;
    let mut next_ind_l: usize = max_gcd_ind;

    let mut mg_m = data[max_gcd_ind];
    let mut mg_r = data[max_gcd_ind];
    let mut mg_l = data[max_gcd_ind];

    let mut mg_so_far = 0;
    let mut mg_len = 1 + 1;

    let mut mg_m_flag: bool = true;
    let mut mg_r_flag: bool = true;
    let mut mg_l_flag: bool = true;

    let data_len = data.len();
    let input_len: u64 = 1 + data.len() as u64;
    let mut mg_so_far_updated = false;

    /*-----------------------*/

    for i in 1..data_len {
        mg_len += 1;

        if mg_r_flag {
            if i + max_gcd_ind < data_len {
                next_ind_r += 1;
            } else {
                next_ind_r = data_len - 1 - i;
            }
            mg_r = gcd(mg_r, data[next_ind_r]);
            let mg_r_temp = mg_r * mg_len;

            if mg_r_temp >= mg_so_far {
                mg_so_far = mg_r_temp;
                mg_so_far_updated = true;
            } else if mg_r * input_len < mg_so_far {
                mg_r_flag = false;
            }
        }
        /*-----------------------*/
        if mg_l_flag {
            if max_gcd_ind + data_len - i >= data_len {
                next_ind_l -= 1;
            } else {
                next_ind_l = i;
            }
            mg_l = gcd(mg_l, data[next_ind_l]);
            let mg_l_temp = mg_l * mg_len;

            if mg_l_temp >= mg_so_far {
                mg_so_far = mg_l_temp;
                mg_so_far_updated = true;
            } else if mg_l * input_len < mg_so_far {
                mg_l_flag = false;
            }
        }
        /*-----------------------*/

        if mg_m_flag {
            if p > 0 && q < data_len - 1 {
                if data[p - 1] > data[q + 1] {
                    p -= 1;
                    next_ind = p;
                } else {
                    q += 1;
                    next_ind = q;
                }
            } else if q < data_len - 1 {
                q += 1;
                next_ind = q;
            } else {
                p -= 1;
                next_ind = p;
            }
            mg_m = gcd(mg_m, data[next_ind]);
            let mg_m_temp = mg_m * mg_len;

            if mg_m_temp >= mg_so_far {
                mg_so_far = mg_m_temp;
                mg_so_far_updated = true;
            } else if mg_m * input_len < mg_so_far {
                mg_m_flag = false;
            }
        }

        /*-----------------------*/
        if mg_so_far_updated {
            let potential_mg = (mg_so_far / mg_len) * input_len;
            {
                match shared_mg.lock() {
                    Ok(l) => {
                        let mut locked_shared_mg = l;
                        if mg_so_far > *locked_shared_mg {
                            *locked_shared_mg = mg_so_far;
                        } else if potential_mg <= *locked_shared_mg {
                            break;
                        }
                    }
                    _ => infinite_loop(),
                };
            }
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

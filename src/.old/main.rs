use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread::{self, JoinHandle};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Mg {
    gcd: u64,
    mg_value: u64,
    len: u64,
}

fn read_input() -> (Vec<Mg>, Vec<usize>) {
    // let stdin = io::stdin();
    // let mut seq_len = String::new();

    // stdin.read_line(&mut seq_len).expect("Failed to read line");
    // let seq_len: usize = seq_len.trim().parse().unwrap();

    // let mut data_str = String::new();
    // stdin.read_line(&mut data_str).expect("Failed to read line");

    // let mut data: Vec<Mg> = Vec::with_capacity(seq_len);
    // let mut max_mg_indicies: Vec<usize> = vec![];

    // let mut iter = data_str.trim().split_whitespace();

    let seq_len = 15;
    let mut data: Vec<Mg> = Vec::with_capacity(seq_len);
    let mut max_mg_indicies: Vec<usize> = vec![];

    let mut iter = "60 30 70  120 60 30 30 30 20 20 20 20 20 20 10 10"
        .trim()
        .split_whitespace();
    let mut temp: u64 = iter.next().unwrap().parse().unwrap();
    let mut prev = Mg {
        gcd: 0,
        mg_value: 0,
        len: 0,
    };

    let mut max_mg_value: u64 = 0;

    if seq_len > 1 {
        for _ in 1..seq_len {
            let input: u64 = iter.next().unwrap().parse().unwrap();
            let num = gcd(temp, input);
            temp = input;
            if num == prev.gcd {
                let ind = data.len() - 1;
                data[ind].mg_value += num;
                data[ind].len += 1;
                prev = data[ind];
            } else {
                let curr = Mg {
                    gcd: num,
                    mg_value: 2 * num,
                    len: 2,
                };
                data.push(curr);

                prev = curr;
            }

            if prev.mg_value > max_mg_value {
                max_mg_value = prev.mg_value;
                max_mg_indicies.clear();
                max_mg_indicies.push(data.len() - 1);
            } else if prev.mg_value == max_mg_value {
                max_mg_indicies.push(data.len() - 1);
            }
        }
    }else{
        //TODO IF LEN =1
    }

    (data, max_mg_indicies)
}

fn rec() {
    let mut data: Vec<Mg> = vec![];
    let mut curr_mg = data[0];
    let adjecent_r = Some(0);
    let adjecent_l = Some(0);


    let mut temp_mg_r = None ;
    let mut temp_mg_l = None ;

    if let Some(next) = adjecent_r {
        temp_mg_r = Some(sub_mg(curr_mg, &data, next));
    }


    if let Some(next) = adjecent_l {
         temp_mg_l  = Some(sub_mg(curr_mg, &data, next));
    }

    if temp_mg_r.is_some() && temp_mg_l.is_some(){

    }else{
        
    }
}

fn sub_mg(curr_mg: Mg, data: &Vec<Mg>, next: usize) -> Mg {
    let other = data[next];

    let _g = gcd(curr_mg.gcd, other.gcd);
    let _len = curr_mg.len + other.len;
    let _mg = _g * _len;

    return Mg {
        gcd: _g,
        mg_value: _mg,
        len: _len,
    };
}
fn main() {
    // let stdin = io::stdin();

    // let mut nr_of_test_cases = String::new();
    // stdin
    //     .read_line(&mut nr_of_test_cases)
    //     .expect("Failed to read line");
    // let nr_of_test_cases: usize = nr_of_test_cases.trim().parse().expect("Failed to usize");

    // for _ in 0..nr_of_test_cases {
    let (data, max_mg_indicies) = read_input();
    println!("{:#?} {:?}", data, max_mg_indicies);
    //     let mut mg: u64 = 0;
    //     if max_mg_indicies.len() == data.len() {
    //         let g: u64 = data[0];
    //         let len: u64 = 1 + data.len() as u64;
    //         match g.checked_mul(len) {
    //             Some(res) => mg = res,
    //             _ => infinite_loop(),
    //         }
    //     } else {
    //         let data_arc = Arc::new(data);
    //         let mg_arc = Arc::new(Mutex::new(0u64));
    //         let mut i = 0;

    //         while i < max_mg_indicies.len() {
    //             let mut join_handles: Vec<JoinHandle<()>> = Vec::with_capacity(50);
    //             let mut j = 0;
    //             while i < max_mg_indicies.len() && j < 50 {
    //                 if i != 0 && max_mg_indicies[i] == max_mg_indicies[i - 1] + 1 {
    //                     i += 1;
    //                     continue;
    //                 }
    //                 let _data_arc = Arc::clone(&data_arc);
    //                 let _mg_arc_clone: Arc<Mutex<u64>> = Arc::clone(&mg_arc);

    //                 let max_gcd_ind = max_mg_indicies[i];
    //                 // println!("{:?}", _data_arc);
    //                 // println!("{:?}", max_mg_indicies);

    //                 let t = thread::spawn(move || {
    //                     find_mg_flag(&_data_arc, max_gcd_ind, _mg_arc_clone);
    //                 });
    //                 join_handles.push(t);
    //                 i += 1;
    //                 j += 1;
    //             }

    //             for handle in join_handles {
    //                 handle.join().unwrap();
    //             }
    //         }

    //         match mg_arc.lock() {
    //             Ok(n) => mg = *n,
    //             _ => infinite_loop(),
    //         };
    //     }
    //     println!("{}", mg);
    // }
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

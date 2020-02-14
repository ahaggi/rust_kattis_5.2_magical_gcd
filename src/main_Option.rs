use std::io;
use std::cmp::{max, Ordering};

#[derive(Eq, Debug, Copy, Clone)]
struct Mg {
    gcd: u64,
    mg_value: u64,
    len: u64,
    sh: u64,
}

impl Ord for Mg {
    fn cmp(&self, other: &Self) -> Ordering {
        self.mg_value.cmp(&other.mg_value)
    }
}

impl PartialOrd for Mg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Mg {
    fn eq(&self, other: &Self) -> bool {
        self.mg_value == other.mg_value
    }
}

fn read_input_seq(input_len: usize) -> Vec<Option<Mg>> {
    let stdin = io::stdin();

    let mut data_str = String::new();
    stdin.read_line(&mut data_str).expect("Failed to read line");

    let mut data: Vec<Option<Mg>> = Vec::with_capacity(input_len);

    let mut iter = data_str.trim().split_whitespace();

    let mut temp: u64 = iter.next().unwrap().parse().unwrap();
    let mut prev = 0;

    for _ in 1..input_len {
        let input: u64 = iter.next().unwrap().parse().unwrap();
        let num = gcd(temp, input);
        temp = input;
        if num == prev {
            let ind = data.len() - 1;

            let mut last = data[ind].unwrap();

            last.mg_value += num;
            last.len += 1;
            data[ind] = Some(last);
        } else {
            let curr = Some(Mg {
                gcd: num,
                mg_value: 2 * num,
                len: 2,
                sh: 1,
            });
            data.push(curr);

            prev = num;
        }
    }
    data
}

fn sub_mg(a_mg: Option<Mg>, b_mg: Option<Mg>) -> Option<Mg> {
    let mut res: Option<Mg> = None;

    let a_mg = a_mg.unwrap();
    let b_mg = b_mg.unwrap();
    let _gcd = gcd(a_mg.gcd, b_mg.gcd);
    let _len = a_mg.len + b_mg.len - a_mg.sh;
    let _mg_value = _gcd * _len;
    res = Some(Mg {
        gcd: _gcd,
        mg_value: _mg_value,
        len: _len,
        sh: b_mg.len,
    });
    res
}

fn main() {
    let stdin = io::stdin();

    let mut nr_of_test_cases = String::new();
    stdin
        .read_line(&mut nr_of_test_cases)
        .expect("Failed to read line");
    let nr_of_test_cases: usize = nr_of_test_cases
        .trim()
        .parse()
        .expect("Failed to parse string to usize");

    for _ in 0..nr_of_test_cases {
        let mut input_len = String::new();

        stdin
            .read_line(&mut input_len)
            .expect("Failed to read line");
        let input_len: usize = input_len.trim().parse().unwrap();
        let mg: u64;

        if input_len == 1 {
            let mut data = String::new();
            stdin.read_line(&mut data).expect("Failed to read line");
            let data: u64 = data.trim().parse().expect("Failed to parse string to u64");
            mg = data;
        } else {
            let mut data = read_input_seq(input_len);
            let data_len = data.len();

            // println!("{:#?} ", data);
            if data_len == 1 {
                mg = data[0].unwrap().mg_value;
            } else {
                mg = find_mg_flag(input_len, &mut data);
            }
        }

        println!("{}", mg);
    }
}
fn find_mg_flag(input_len: usize, data: &mut [Option<Mg>]) -> u64 {
    let input_len: u64 = input_len as u64;

    let data_len = data.len();
    let mut new_len = data_len;
    let mut mg_so_far = data[0];
    // println!("{:?}", data);

    for _ in 0..(data_len as u64) {
        let mut prev: Option<Mg> = data[0];

        for ind in 1..new_len {
            if prev.is_none() || data[ind].is_none() {
                data[ind - 1] = None;
                prev = data[ind];
            } else {
                let res = sub_mg(prev, data[ind]);
                // println!("{:?}", res);
                mg_so_far = max(mg_so_far, max(res, data[ind]));
                prev = data[ind];

                if res.unwrap().gcd * input_len < mg_so_far.unwrap().mg_value {
                    data[ind - 1] = None;
                } else {
                    data[ind - 1] = res;
                }
            }
        }
        new_len -= 1
    }

    // println!("{:?}", mg_so_far);
    mg_so_far.unwrap().mg_value
}

fn calc_mg(mg_so_far: &Mg, input_len: &u64, res_mg: &Arc<Mutex<u64>>) -> bool {
    let potential_local_mg = mg_so_far.gcd * input_len;
    let mut calc_next = true;
    match res_mg.lock() {
        Ok(num) => {
            let mut global_shared_mg = num;
            if mg_so_far.mg_value > (*global_shared_mg) {
                *global_shared_mg = mg_so_far.mg_value;
            } else if potential_local_mg <= *global_shared_mg {
                calc_next = false;
            }
        }
        _ => panic!("Err: could not get the lock!"),
    };
    calc_next
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

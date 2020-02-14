use std::cmp::{max, Ordering};
use std::io;

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

fn read_input_seq(input_len: usize) -> (Vec<Mg>, u64) {
    let stdin = io::stdin();
    let mut data_str = String::new();
    stdin.read_line(&mut data_str).expect("Failed to read line");
    let mut data: Vec<Mg> = Vec::with_capacity(input_len);
    let mut iter = data_str.trim().split_whitespace();
    let mut temp: u64 = iter.next().unwrap().parse().unwrap();
    let mut prev = Mg {
        gcd: 0,
        mg_value: 0,
        len: 0,
        sh: 0,
    };

    let mut init_mg = temp;

    for _ in 1..input_len {
        let input: u64 = iter.next().unwrap().parse().unwrap();
        init_mg = max(init_mg, input);
        let _gcd = gcd(temp, input);
        temp = input;
        if _gcd == prev.gcd {
            let last_ind = data.len() - 1;
            data[last_ind].mg_value += _gcd;
            data[last_ind].len += 1;
            prev = data[last_ind];
        } else {
            let curr = Mg {
                gcd: _gcd,
                mg_value: 2 * _gcd,
                len: 2,
                sh: 1,
            };
            data.push(curr);
            prev = curr;
        }
    }
    (data, init_mg)
}

fn sub_mg(a_mg: Mg, b_mg: Mg) -> Mg {
    let _gcd = gcd(a_mg.gcd, b_mg.gcd);
    let _len = a_mg.len + b_mg.len - a_mg.sh;
    let _mg_value = _gcd * _len;
    return Mg {
        gcd: _gcd,
        mg_value: _mg_value,
        len: _len,
        sh: b_mg.len,
    };
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
            let (mut data, init_mg) = read_input_seq(input_len);
            let data_len = data.len();
            if data_len == 1 {
                mg = data[0].mg_value;
            } else {
                mg = calc_mg(&mut data, init_mg);
            }
        }

        println!("{}", mg);
    }
}

fn calc_mg(data: &mut [Mg], init_mg: u64) -> u64 {
    let data_len = data.len();
    let mut new_len = data_len;
    let mut mg_so_far = if init_mg > data[0].mg_value {
        Mg {
            gcd: init_mg,
            mg_value: init_mg,
            len: 1,
            sh: 0,
        }
    } else {
        data[0]
    };

    for _ in 0..(data_len as u64) {
        // println!("{:#?}", &data[0..new_len]);

        let mut prev: Mg = data[0];
        for ind in 1..new_len {
            let res = sub_mg(prev, data[ind]);
            data[ind - 1] = res;
            prev = data[ind];
            // println!("{:?}", res);
            mg_so_far = max(mg_so_far, max(res, data[ind]));
        }
        new_len -= 1
    }
    mg_so_far.mg_value
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

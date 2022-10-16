

fn main() {
    let n = 19;
    let fib = n_th_fib(n);
    println!("{n}th Fibonacci is {fib}");
}

fn f_to_c(f: f64 ) -> f64  {
    let c = (f-32.0) * (5.0/9.0);
    c
}

fn n_th_fib(n: u32) -> u32 {
    let mut fib_min_1 = 1;
    let mut fib_min_2 = 0;
    let mut fib = 0;
    let mut index = 0;
    if n == 0 {
        fib_min_2
    }
    else if n == 1 {
        fib_min_1
    }
    else {
        while index < (n-1) {
            fib = calc_fib(fib_min_1, fib_min_2);
            fib_min_2 = fib_min_1;
            fib_min_1 = fib;
            index += 1;
        }
        fib
    }
}

fn calc_fib(f_min_1: u32, f_min_2: u32) -> u32 {
    let fib = f_min_1 + f_min_2;
    fib
}
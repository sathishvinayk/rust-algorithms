fn myst() {
    let mysteriousfunction = |n: usize| {
        n & n - 1 == 0
    };
    for n in 1..10000 {
        if mysteriousfunction(n) {
            println!("{:?}", n)
        }
    }
}

fn main() {
    myst();
}
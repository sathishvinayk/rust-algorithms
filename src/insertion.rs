fn insertion(initial: &str, secondary: &str, rat: usize, cat: usize) -> String {
    let mut initial: Vec<char> = initial.chars().collect();
    let secondary: Vec<char> = secondary.chars().collect();

    let initiallength = initial.len() - 1;
    let secondarylength = secondary.len() - 1;

    for index in 0..cat - rat + 1 {
        initial[initiallength - (rat + index)] = secondary[secondarylength - index];
    }
    let result: String = initial.into_iter().collect();

    result
}

fn main() {
    insertion("10000000000", "10011", 2, 6);
}

#[test]
fn test() {
    assert_eq!(insertion("10000000000", "10011", 2, 6), "10001001100");
}
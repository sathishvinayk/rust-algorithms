fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

fn flipbittowin(number: u32) -> usize {
    let binarystring = format_radix(number, 2);
    let arrones: Vec<&str> = binarystring.split('0').collect();

    let mut longest = 0;

    for i in 0..arrones.len() - 1 {
        if arrones[i].len() + arrones[i + 1].len() > longest {
            longest = arrones[i].len() + arrones[i + 1].len();
        }
    }
    longest += 1;

    longest
}

fn main() {
    flipbittowin(1775);
}

#[test]
fn test() {
    assert_eq!(flipbittowin(1775), 8);
}
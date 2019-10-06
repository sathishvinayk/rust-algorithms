use std::cmp;

// string comparision to perform counts of repeated characters
fn strcomp(string: &str) -> String {
    let mut compressed_value: String = "".to_owned();
    let mut current_character = ' ';
    let mut current_count = 0;
    let mut max_count = 1;

    for n in string.chars(){
        if n != current_character {
            if current_count != 0 {
                let data = current_character.to_string() 
                        + &current_count.to_string();
                compressed_value.push_str(&data);
            }

            max_count = cmp::max(max_count, current_count);   
            
            current_character = n;

            current_count = 1;
            
        } else {
            current_count += 1;
        }
    }

    let data = current_character.to_string() 
                        + &current_count.to_string();
    compressed_value.push_str(&data);
    let max_count = cmp::max(max_count, current_count);

    if max_count == 1 { string.to_string() } else { compressed_value.to_string() }
}

#[test]
fn test() {
    assert_eq!(strcomp("aaaaaa"), "a6");
    assert_eq!(strcomp("aabcccccaaa"), "a2b1c5a3");
}

fn main() {
    let a = strcomp("aabb");
    println!("{}", a);
}
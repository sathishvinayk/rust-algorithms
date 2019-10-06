// Check permutation of two strings
fn checkpermute(string1: &str, string2: &str) -> bool {
     if string1.len() != string2.len() {

          false
     } else {
          let mut sortedstringone: Vec<char> = string1.chars().collect();
          let mut sortedstringtwo: Vec<char> = string2.chars().collect();
          
          sortedstringone.sort_by(|a, b| a.cmp(b));
          sortedstringtwo.sort_by(|a, b| a.cmp(b));

          sortedstringone == sortedstringtwo
     }
}

#[test]
fn test() {
    assert_eq!(checkpermute("aba", "aab"), true);
    assert_eq!(checkpermute("aba", "aaba"), false);
    assert_eq!(checkpermute("aba", "aa"), false);
}
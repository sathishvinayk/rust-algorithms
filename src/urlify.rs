fn urlify(string: &str) -> String {
     let stringstr = string.replace(" ", "%20");

     stringstr
}

#[test]
fn test() {
    assert_eq!(urlify("Mr John Smith    "), "Mr%20John%20Smith%20%20%20%20");
}
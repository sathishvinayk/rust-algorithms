fn stringrotation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false
    }
    let data = string2.to_owned() + string2;
    let result = data.contains(string1);

    result
}

#[test]
fn test() {
    assert_eq!(stringrotation("waterbottle", "erbottlewat"), true);
    assert_eq!(stringrotation("waterbottle", "erbotlewatt"), false);
    assert_eq!(stringrotation("aaata", "aataa"), true);
}

fn main() {
    stringrotation("waterbottle", "erbottlewat");
}


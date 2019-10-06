use std::collections::HashMap;

fn palinperm(string: &str) -> bool {
     let mut palin = HashMap::new();

     let mut mulligan: bool = false;
     let mut permanent: bool = true;

     for value in string.to_lowercase().chars() {
          if value != ' ' {
               if !palin.contains_key(&value) {

                    palin.insert(value, 0);
               }
               *palin.get_mut(&value).unwrap() += 1;
          }
     }

     for value in palin.values() {
          if value % 2 > 0 {
               if mulligan {
                    permanent = false;
               } else {
                    mulligan = true;
               }
          }
     }
     permanent
}

#[test]
fn test() {
    assert_eq!(palinperm("coco coco"), true);
    assert_eq!(palinperm("bozo zobo"), true);
    assert_eq!(palinperm("mbn bmn"), true);
}
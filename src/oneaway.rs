// Oneaway inserts, removes replaces character. Checks edits away
fn oneaway(string1: &str, string2: &str) -> bool {
     let checkonemissing = |first: &str, second: &str| -> bool {
          if first.len() != second.len() - 1 {
               false
          } else {
               let mut mulligan: bool = false;
               let mut f_p = 0;
               let mut s_p = 0;
               while f_p < first.len() {
                    let str1 = first.chars().nth(f_p).unwrap();
                    let str2 = second.chars().nth(s_p).unwrap();
                    if str1 != str2 {
                         if mulligan {
                              return false
                         } else {
                              mulligan = true;
                              s_p += 1;
                         }
                    } else {
                         f_p += 1;
                         s_p += 1;
                    }
               }
               true
          }
     };

     let checkonediff = |first: &str, second: &str| -> bool {
          if first.len() != second.len() {
               false
          } else {
               let mut mulligan = false;
               let mut f_p = 0;
               let mut s_p = 0;
               while f_p < first.len() {
                    let str1 = first.chars().nth(f_p).unwrap();
                    let str2 = second.chars().nth(s_p).unwrap();

                    if str1 != str2 {
                         if mulligan {
                              return false
                         } else {
                              mulligan = true
                         }
                    }
                    f_p += 1;
                    s_p += 1;
               }
               true
          }
     };
     checkonemissing(string1, string2)
     || checkonemissing(string2, string1)
     || checkonediff(string1, string2)
}

#[test]
fn test() {
     assert_eq!(oneaway("ple", "pale"), true);
     assert_eq!(oneaway("pales", "pale"), true);
     assert_eq!(oneaway("pale", "bale"), true);
     assert_eq!(oneaway("pale", "bake"), false);
}
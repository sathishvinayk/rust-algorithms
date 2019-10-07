// Using algo for zero matrix. Find zero in matrix and fill 0 for row and column
use std::collections::HashMap;
fn checkzeros(matrix: &mut [[u8; 4]; 6]) {
    if matrix.len() == 0 {
        return
    }
    
    let mut rowstozeroify: HashMap<_, bool> = HashMap::new();
    let mut colstozeroify: HashMap<_, bool> = HashMap::new();
    {
        let matrixheight = matrix.len();
        let matrixwidth = matrix[0].len();

        for i in 0..matrixheight {
            for j in 0..matrixwidth {
                if matrix[i][j] == 0 {
                    rowstozeroify.insert(i as usize, true);
                    colstozeroify.insert(j as usize, true);
                }
            }
        }
    }
    
    {
        for col in colstozeroify.keys(){
            for j in 0..matrix.len() {
                matrix[j][*col] = 0;
            }
        }
    }

    {
        for row in rowstozeroify.keys(){
            for j in 0..matrix[*row].len() {
                matrix[*row][j] = 0;
            }
        }
    }
    println!("{:?}", matrix);
}
fn main() {
    let mut vector = [
        [1,1,1,1],
        [1,1,1,1],
        [1,0,1,1],
        [1,1,1,1],
        [1,1,1,1],
        [1,1,1,1]
    ];

    checkzeros(&mut vector);
}
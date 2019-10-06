fn rotatematrix(matrix: [[u8; 4]; 4]) {
    let mut mat = matrix;
    let edge = matrix.len() - 1;

    let mut movepixels = |row: usize, col: usize| {
        let mut fromrow;
        let mut fromcol;
        let mut frompixel;

        let mut torow = row;
        let mut tocol = col;
        let mut topixel = mat[row][col];

        for _i in 0..4 {
            fromrow = torow;
            fromcol = tocol;
            torow = fromcol;
            tocol = edge - fromrow;

            frompixel = topixel;
            topixel = mat[torow][tocol];
            mat[torow][tocol] = frompixel;
        }
    };

    for n in 0..matrix.len() / 2 {
        for m in n..edge - n {

            println!("{} {}", n, m);
            movepixels(n, m);
        }
    }
}

fn main() {
    let matrix = [
        [1, 2, 3, 4],
        [0, 1, 2, 3],
        [0, 0, 1, 2],
        [1, 0, 0, 1]
    ];
    rotatematrix(matrix);
}

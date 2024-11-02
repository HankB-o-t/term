use std::process;
// array:
//   c    w   h
// [[""; 4]; 6]
// a[y][x]

fn main() {
    let w = 30; let h = 10;
    let mut arr = vec![vec!["#"; w]; h];
            
    loop {
        // check key presses.


        //print
        for i in 0..h {
            for j in 0..w {
                print!("{}", arr[i][j]);
            }
            println!("");
        }
        process::Command::new("clear").status().unwrap();
    }
}

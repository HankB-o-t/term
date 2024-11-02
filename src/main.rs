use std::process;
use std::thread;
use std::time;
// array:
//   c    w   h
// [[""; 4]; 6]
// a[y][x]

fn main() {
    let w = 30; let h = 10;
    let mut arr = vec![vec!["_"; w]; h];
    let ms10 = time::Duration::from_millis(250);
    let mut now = time::Instant::now();

    loop {
        let e = now.elapsed();
        //print
        for i in 0..h {
            for j in 0..w {
                arr[i][j] = ".";
                arr[i][i*2] = "o";
                arr[i][i*3] = "a";
                arr[i][i/2] = "#";
                arr[i][i/4] = "[";
                arr[i][i] = "@";
                print!("{}", arr[i][j]);
            }
            println!("");
        }
        println!("{}",e.as_secs());
        thread::sleep(ms10);
        process::Command::new("clear").status().unwrap();
    }
}

use std::{process,thread,time};fn main(){let w=30;let h=
10;let mut arr =vec![vec!["_";w];h];let ms10=time::Duration
::from_millis(250);let mut now=time::Instant::now();let mut
r=0;let mut v=0;let mut g=0;loop {let e =now.elapsed();for i
in 0..h {for j in 0..w{arr[i][j] =" ";arr[i][r] ="|";arr[v]
[j]= "-";arr[v][r] ="+";arr[i][i*g] = "o";print!("{}",arr[i]
[j]);}println!("");}if g>=3{g=0}g+=1;if r>=w-1{r=0}else{r+=1}
if v>=h-1{v=0}else{v+=1}println!("x: {} y: {}",r,v);println!(
"s: {}",e.as_secs());thread::sleep(ms10);process::Command
::new("clear").status().unwrap();}}

use std::fs::File;
use std::io::prelude::*;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    }


fn main() {
    let mut total_lines = 0;
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("No path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
        };
    let mut file = File::open(&args.path).unwrap();
    let mut read_buf = std::io::BufReader::new(file);
    let mut vector: Vec<i32> = vec![];
    for line in read_buf.lines(){
        match line{
            Err(why) => panic!("{:?}", why),
            Ok(string) => match string.trim().parse::<i32>(){
                Err(_) => panic!("Not a number!"),
                Ok(number) => vector.push(number)
            }
        }
    }
    let mut depths = vector.into_iter().peekable();
    let mut increase_counter = 0;
    let mut decrease_counter = 0;
    let mut unchanged_counter = 0;
    while let Some(depth) = depths.next(){
        let maybe_depth_next = depths.peek();
        let depth_next = match maybe_depth_next {
            Some(n) => n,
            None => continue
        };
        if depth_next - depth > 0{
            increase_counter+=1;
        } else if depth_next - depth < 0 {
            decrease_counter +=1;
        } else if depth_next - depth == 0 {
            unchanged_counter +=1;
        }   else {
            println!("wonky math at play");
        }
    };
    //println!("{:?}", vector);
    println!("{:?}", increase_counter);
}

// let depth = match maybe_depth {
//             Some(n) => n,
//             None => {
//                 println!("End of file");
//                 continue
//             } 

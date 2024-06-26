use std::env;
use std::process;
use r_jparser::{run_jparser};

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() > 2{
        println!("[usage] r-jparser jsonfile");
        process::exit(1);
    }else if args.len() == 2{
        run_jparser(&args[1]);
    }else{
        println!("Need atleast one argument [json file]");
        process::exit(1);
    }
}
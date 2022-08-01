use std::env;
use std::process::exit;
use std::thread;
pub mod rules;

fn test(nbr: i32) 
{
    for i in 1..10
    {
        println!("test {} {} ", nbr, i);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 || args.len() > 6
        {
            println!("error argument");
            exit(1);
        }
    let _stru = rules::init_rules(args);
    thread::spawn(|| test(0));
    let handle = thread::spawn(|| test(1));
    handle.join().unwrap();
}

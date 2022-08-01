use std::env;
use std::process::exit;
use std::thread;
use std::sync::{Arc, Mutex};
pub mod rules;

fn test(nbr: i32, data: Arc<Mutex<i32>>)
{
    data.lock().unwrap();
    for i in 1..10
    {
        println!("test {} {} ", nbr, i);
    }
}

fn main() {
    let mut handles = vec![];
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 || args.len() > 6
        {
            println!("error argument");
            exit(1);
        }
    let data = Arc::new(Mutex::new(0));
    let _stru = rules::init_rules(args);
    for i in 0..2
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || test(i,data));
        handles.push(handle);
    }
    for handle in handles
    {
        handle.join().unwrap();
    }
}

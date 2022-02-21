use std::env;

pub fn exec() -> String {
    let args: Vec<String> = env::args().skip(1).collect();
    
    args[0].clone()
}

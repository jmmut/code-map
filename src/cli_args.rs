

pub fn get_args() -> (String, f32) {
    let args: Vec<String> = std::env::args().collect();
    let folder = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };
    let padding = if args.len() > 2 {
        args[2].parse::<f32>().unwrap()
    } else {
        0.0
    };
    (folder, padding)
}

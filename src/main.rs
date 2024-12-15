use::std::env;
use::std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("searching for {} in {}", config.query, config.file_path);
    let file_contents = fs::read_to_string(&config.file_path).expect("should've been able to read file");
    println!("with contents:\n{}", file_contents);
}

struct Config{
    query: String,
    file_path:String,
}

fn parse_config(args: &[String])->Config{
    let query = args[1].clone();
    let file_path = args[2].clone();
    return Config { query, file_path }
}

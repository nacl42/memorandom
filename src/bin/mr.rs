use memorandom:: {Memo, parse};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args.get(1).expect("expected file name of .mr file");
    let input = std::fs::read_to_string(file_path).expect("failed to read .mr file");
    let result = parse(&input);
    println!("{:#?}", result);
}

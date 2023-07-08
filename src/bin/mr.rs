use memorandom::{parse, Memo};

fn print_usage(executable: &str) {
    println!(r#"=== MemoRandom ==

Usage: {} <cmd> [<args>]

Available commands:
  list <filename>    -- list all memos in the file with the given `filename`
  info               -- display information about this software
  help               -- display this help
"#, executable);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut args = args.iter().map(|s| s.as_str());

    let executable = args.next().map(|s| std::path::PathBuf::from(s)).unwrap();
    let executable = executable.file_name().unwrap();

    match args.next() {
        Some("list") => {
            let file_path = args.next().expect("expected file name of .mr file");
            let input = std::fs::read_to_string(file_path).expect("failed to read .mr file");
            let result = parse(&input).expect("failed to parse .mr file");
            for memo in result {
                println!("@{} {}", memo.collection(), memo.title())
            }
        }
        Some("info") => {
            println!("memorandom, version ...")
        }
        Some("help") => print_usage(&executable.to_string_lossy()),
        Some(x) => {
            println!("unknown command '{}'", x)
        }
        None => print_usage(&executable.to_string_lossy()),
    }
}

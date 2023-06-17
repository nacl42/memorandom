use memorandom:: {Memo, parse_line};

fn main() {
    let memo = Memo::new("book", "The Lord of the Rings")
        .with("author", "J.R.R. Tolkien")
        .with("language", "English")
        .with_many("character", ["Bilbo", "Samwise", "Gandalf"])
        .with("character", "Aragon")
        .with_many("genre", ["high fantasy", "adventure"]);

    println!("{}", memo);

    let _ = parse_line("@foo   aa");
}

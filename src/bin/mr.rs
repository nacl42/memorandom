use memorandom:: {Memo, parse_line, parse};

const RECIPE: &str = r"
@recipe Spaghetti Napoli
.ingredient spaghetti
.ingredient onion
.ingredient tomatoes
.ingredient, oregano, basil
.ingredient olive oil
";

fn main() {
    let memo = Memo::new("book", "The Lord of the Rings")
        .with("author", "J.R.R. Tolkien")
        .with("language", "English")
        .with_many("character", ["Bilbo", "Samwise", "Gandalf"])
        .with("character", "Aragon")
        .with_many("genre", ["high fantasy", "adventure"]);

    println!("{}", memo);

    let result = parse(RECIPE);
    println!("{:#?}", result);
}

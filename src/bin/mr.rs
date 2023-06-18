use memorandom:: {Memo, parse_line, parse};

const RECIPE: &str = r"
@recipe Spaghetti Napoli
.ingredient spaghetti
.ingredient onion
.ingredient tomatoes
.ingredient, oregano, basil
.ingredient; salt; pepper
 patience; whatever
.ingredient olive oil
.howto
 put the spaghetti into a pot of salted, boiling water,
 don't forget to stir from time to time,
 and so on
~

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

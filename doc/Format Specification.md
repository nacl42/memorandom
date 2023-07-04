# Format Specification

## Introduction

**Memo**: A memo consists of a header (`@collection label`) and several nodes (`.key value`):

    @collection label
    .key1 value
    .key2 value
    .key3 value

**Header**: The memo header specifies the *collection* and the *label*: The collection is useful for grouping different memos. Examples are `@contact`, `@recipe` or `@book`. The label can be anything you like, e.g. `Alice`, `pasta` or `The Lord of the Rings`.

**Nodes**: The actual data of the memo is stored in form of key/value pairs called **nodes**. The value is always interpreted as text, therefore no quotation characters such as `"` or `'` are needed.

The following is a simple example for a memo in a contact database, that describes Alice, an old school friend who lives in a quiet town somewhere in Great Britain:

    @contact Alice
    .address Privet Drive, Little Whinging
    .phone 1357-975246
    .last-update 2023-07-02
    .keyword school friend
    .keyword muggle

The data consists of the nodes `address`, `phone`, `last-update` and `keyword`. It is possible to specify multiple nodes with the same key, which is illustrated for the `keyword` node.

## Comments

The Memorandom format allows for comments. Any line starting with a hash (`#`) is ignored by the parser.

    # My Personal Contact Database
    @contact Alice
    .address Privet Drive, Little Whinging
    [..]

Please note that the hash mark must be the first character of the line.

    # This is a valid comment
     # This is not.
     @contact Alice # neither is this


## Multi-Node Notation

There is also a shorter notation for specifying multiple nodes with the same key:

    # multi-node notation
    .key, value1, value2, value3

    # equivalent to
    .key value1
    .key value2
    .key value3

So the above example for `@contact Alice` might be written more concisely as:

    .keyword, school friend, muggle

There is an alternative multi-node notation using a semicolon:

    .key; value1; value2; value3

The separator indicator, which can be either a comma (`,`) or a semicolon (`;`) is placed directly after the keyword. 

Please be aware, that with this compact notation, it is not possible to include the separator character, i.e. the comma (,) or the semicolon (;). If you need to specify such a character, then simply use the default notation for two different nodes:

    # this:
    .separator, comma (,), semicolon (;)
    
    # would be interpreted as:
    .separator comma (
    .separator )
    .separator semicolon (;)

    # you might want to use this instead:
    .separator comma (,)
    .separator semicolon (;)

## Multi-Line Values

A common problem with text formats is specifying **multi-line values**. The memorandom format supports two variants, **folded text** and **literal text**.

**Folded Text**: Folding, which is the default, merges subsequent lines that start with a whitespace (` `) and replaces the newline by a single space character. Real newlines can be added by inserting lines that start with a single whitespace and contain only whitespace afterwards:

    .notes>
     Alice is a very polite person that lives
     in Privet Drive. She has never been in
     contact with the magical world and it
     is believed she never ever realized what
     happened around her.

     Alice is a good friend of Bob.

The folding indicator (`>`) is put directly after the keyword. If there is no indicator at all, folding is implied.

**Literal text**: The literal indicator (`|`) is very similar, but newlines are preserved. This can be useful for source code, poems or other kinds of literal text:

    @poem A Poison Tree
    .author William Blake
    .poem|
     I was angry with my friend;
     I told my wrath, my wrath did end.
     I was angry with my foe:
     I told it not, my wrath did grow.
     [...]


## Multi-Line Multi-Node notation

It is also possible to treat each line in a multi-line value as separate value:

    # the asterisk is the multi-line indicator
    .color*
     red
     blue
     green

    # equivalent to:
    .color red
    .color blue
    .color green

If the value separator is specified, then subsequent lines will be treated and split as separate values. The following notations are equivalent:

    # verbose notation
    .color red
    .color blue
    .color green
    .color yellow
    
    # compact, single line notation with value separator
    .color, red, blue, green, yellow
    
    # equivalent to
    .color, red, blue
    .color, green, yellow
    
    # equivalent to
    .color,
     red, blue
     green, yellow

    # equivalent to
    .color, red, blue
     green, yellow

## References (Links)

By default, any node value is considered a simple text string. It is of course possible to let a value refer to an external url, using the standard url notation

    @bookmark Google
    .url https://www.google.com
    .url <https://www.google.com>

It is then up to the user of the memo to properly interpret this value.

Memorandom will also implement a notation for references to other memos. This could be useful for creating interconnected graphs. However, at the current development stage, the precise format is not yet settled.

Ideas include:

    # memo to refer to 
    @author Tolkien

    # this book was written by Tolkien
    @book The Lord of the Rings
    
    # implicit link
    .author Tolkien
    .written-by:author Tolkien

    # URI-style links
    .author Tolkien
    .author mr:Tolkien
    .author <mr:Tolkien>
    .author <mr://Tolkien>

    # wiki-like syntax
    .author [Tolkien]
    .written-by [author:Tolkien]


## Reserved Names

The collection name `mr:` is reserved for internal use.

Examples include:

    # Template Definition
    @mr:tpl

    # Documentation of the current .mr file
    @mr:doc

    # Include another .mr file
    @mr:include

    # Filter Definition
    @mr:filter


## Sample Memos

The following examples showcase the given features:

    @book The Lord of the Rings
    .author J.R.R. Tolkien
    .genre, high fantasy, adventure
    .character; Bilbo Baggins; Samwise Gamgee; Gandalf the Gray

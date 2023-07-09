
# TODO

- Convert data to json...?

    @book foo
    .author bar
    .keywords, drama, modern

    compact notation:

    { "$schema": "book", "$id": "foo", "author": "bar", "keywords": ["drama", "modern"]}

    or verbose notation:

    { "$schema": "book", "$id": "foo",
      [ {"author", { "text": "bar", "attr": {}} }
        {"keywords", { "text": "drama", "attr": {}}},
        {"keywords", { "text": "modern", "atr": {}}}]}

    (skip 'attr' if not available)

Idea: Serialize map like structure either as single value or as list

  "one"  or ["one", "two"]

  Have "$id" and "$schema" as key as well.

  "$schema": "book",
  "$id": "The Book of ..."
  "author": "..."
  "keywords": ["drama", "modern"],

- Actually use data by integrating a template mechanism.
  => custom format
  @{{$type}} {{$label}} -- {{slogan}} ({keyword* | join ',' }) 

- Rewrite parser to have a proper Context instead of several variables.
- Skip attributes for now? Or parse them anyway?


## Support Qualifiers for key names

**Request**:

Support the following syntax:

```
.key:qualifier value
```

**Examples**:

```
# Use qualifier to specify the collection type of a reference

@book The Lord of the Rings

@character Gandalf
.appears-in:book The Lord of the Rings
```

```
# Use qualifier to specify language

@element H
.name:en hydrogen
.name:de Wasserstoff
```

**Discussion**:

Should the qualifier be simply part of the key?

```
'name:en' = ['hydrogen']
'name:de' = ['Wasserstoff']
```

Or should the qualifier be part of the value itself?

```
'name' =
    [ 
        Value { text: 'hydrogen', qualifier: 'en'},
        Value { text: 'Wasserstoff', qualifier: 'de' }
    ]
```

If it was part of the value, then we would need to assign the qualifier to all values of a multi-value field:
```
.name:en, hydrogen, whatever

# equivalent to:
.name:en hydrogen
.name:en whatever
```



## Implement Syntax Highlighting for Visual Studio Code

**Request**: Implement a simple syntax highlighting for VSCode for .mr files

## Attributes

Attributes are currently just an idea and not yet supported.

They are additional meta data for each key/value pair. One possible use is information about the source of the information:

    .density 7.874
    +source wikipedia
    +date 2023-06-27
    .name iron
    +lang en
    .name Eisen
    +lang de

Attributes are separate entities and stop the parsing of a value:

    .recipe>
     do this, then that
     and finally that
    +date 2023-06-27
    +source Wikipedia
     this line belongs to the +source attribute!

Therefore, this notation does not make sense:

     .recipe>
    +date 2023-06-27
      do this, then that
      and finally that
      this line and the two above belong the the date attribute, not to the recipe!


While this one makes sense:

    .recipe>
      do this, then that
      and finally that
      and it is clear that this line belongs to the others.
    +date 2023-06-27


Inline attributes would be nice, but will not be supported yet:

    .ingredients*
     pasta         |+qty 500 g
     tomatoes      |+qty 2
     olive oil     |+qty 1 bottle


It is possible to define multiple values in one data entry using the asterisk (*), comma (,) or semicolon (;) notation. In this case, the attributes defined later on should belong to all values defined before:

    .ingredients*
     pasta
     tomatoes
     olive oil
    +source wikipedia
    +date 2023-06-27


# DONE

## DONE: high-level testing of parser

**Request**: Test on a higher level that all test cases yield the same memo:

```
@case
.color red
.color blue
.color green

@case
.color, red, blue, green

@case
.color, red,blue,green

@case
.color, red   ,   blue   ,  green  

@case
.color red
.color, blue, green
```

**Realization**: `parser.rs::tests::test_parse_memo`

**Date of Realization**: 2023-06-26

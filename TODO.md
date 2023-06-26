
# TODO

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

## Implement Syntax Highlighting for Visual Studio Code

**Request**: Implement a simple syntax highlighting for VSCode for .mr files

## Attributes

Attributes are currently just an idea and not yet supported.



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

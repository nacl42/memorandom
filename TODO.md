
# TODO

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

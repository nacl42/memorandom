# Format Specification

## Specification

memo header:

```
@schema label
```

single key/value pair:
```
.key value
```

several key/value pairs (multi-value line):
```
.key, value1, value2, value3
.key; value1; value2; value3
```

multi-line value:

```
.key>
 This is a folded multi-line string. The lines
 are folded. Each new line starts with a
 single space as indentation.
.key|
 This is a literal multi-line string,
 this is the second line
 and this is the third.
```

Abstract definition of a field:

```
.key[.,>|] value
```

Indicator char:
- Multi-Value Line (,) or (;)
- Multi-Line Value (folded >) or (literal |)

## Sample Memo

```
@book The Lord of the Rings
.author J.R.R. Tolkien
.genre, high fantasy, adventure
.character Bilbo Baggins, Samwise Gamgee, Gandalf the Gray
```

## Simplified Format

The simplified format has some limitations with respect to the original data:
- sort order of the keys is not preserved
- all values belonging to a certain key are listed subsequently
- value separators are not included

The simplified output of the above example might look like this:

```
@book The Lord of the Rings
.author J.R.R. Tolkien
.character Bilbo Baggins
.character Samwise Gamgee
.character Gandalf the Gray
.genre high fantasy
.genre adventure
```

## Attributes

Attributes are additional information related to one or more data fields. Examples include sources for the information or the specific conditions under which this data might apply.

They are similar to XML attributes.

```xml
<mail id="42">
  <from>Alice</from>
  <to>Bob</to>
  <body>Dear Bob, ...</body>
</mail>
```

```
@mail
+id 42
.from Alice
.to Bob
.body Dear Bob, ...
```

or with the in-line notation for attributes:

```
@mail |+id 42
.from Alice
.to Bob
.body Dear Bob, ...
```

Attributes may only span one line.

Attributes themselves provide one additional level of nesting.

It is recommended to use attributes mostly for metadata.

## Links

Values may refer to other memos.

```
@book The Lord of the Rings
.author J.R.R. Tolkien

@author J.R.R. Tolkien
```

By default, the name of the data field is also the collection name of the memo.

In some cases, the field name and the collection type do not match. It is therefore possible to explicitely specify the collection type by the field qualifier:

```
@book The Lord of the Rings
.protagonist:character Frodo Baggins

@character Frodo Baggins
```



## Reserved Names

The collection name `mr:` is reserved for internal use.

Examples include:

```
@mr:tpl
@mr:doc
@mr:include
@mr:filter
```
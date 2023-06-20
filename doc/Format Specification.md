# Format Specification

## Specification

```
@schema label
.key1 value
.key2:qualifier value
.key3, value1, value2, value3
.key4 multi-line value
 second line of multi-line value
 third line
.key5 value
.key6<<EOF
another multi-line value
without indentation
EOF
```

- The **collection**, which corresponds to a database **schema**.
- The **title**, which corresponds to an **id**.
- Multiple **data nodes**, each with **key** and **value**.

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
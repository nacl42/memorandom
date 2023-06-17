# Format Specification

## Specification

```
@schema id
.key value
.key value
.key value
````

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

## Reserved Names

The collection name `mr:` is reserved for internal use.

Examples include:

```
@mr:tpl
@mr:doc
@mr:include
@mr:filter
```

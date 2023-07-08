>
> **DISCLAIMER: This is work in progress and not yet production-ready!**
>

# memorandom

A plain-text database format that can be read by computers and edited by humans.

Memorandom files are suitable for semi-structured data, that is not yet well defined, i.e. there is no definite structure (yet) and the kind of information is subject to change.

The database consists of units of information called Memos with an
editor-friendly syntax. A single `memo` consists of a mandatory header
node and optional data nodes.

A very simple example looks like this:

```
@app memorandom
.url https://github.com/nacl42/memorandom
.tag, software, database, plain-text
.doc a plain-text database format that can be read by computers and edited by humans
.license MIT or Apache 2.0
```

This defines the memo with the title `memorandom` and which belongs to the
collection `app`. It has a `url`, a `doc` string and three different
`tag` nodes ("software", "database" and "plain-text").

## Features and Limitations

Features are:
* editor-friendly, human-editable (easy to input)
* simple insertion of multiple nodes
* PLANNED: each node can have optional attributes
* PLANNED: simple yet effective query language (mql)
* PLANNED: IDE support available

Limitations are:
* only suitable for small database
* no nesting of fields

## Goals and Non-Goals of this Software

Goals:
- read and parse .mr files properly
- write data in the simplified form (see specification)
- provide a template mechanism to write data from existing .mr files

Non-Goals:
- manipulate existing .mr files

## Roadmap and Development

This is highly experimental software, which is used by the author and
probably no one else. Even the syntax itself is subject to change.

You have been warned.

If you have any suggestions or feedback, you are welcome to do so via
the github page.

The location of the source directory is on
[github](https://github.com/nacl42/memorandom).

The former implementation of the .mr format can also be found on
[github](https://github.com/nacl42/merula). This first version is currently far
more feature-complete.


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
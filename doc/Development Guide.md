# Development Guide

## General Guidelines

- Use as little dependencies as possible.
- Solve existing problems, not theoretical ones.

## Other text-file formats

- [RFC822 (E-Mail)](https://datatracker.ietf.org/doc/html/rfc822)
- [YAML](https://yaml.org/)
- [Recutils](https://www.gnu.org/software/recutils/)
- [Nested Text](https://nestedtext.org/en/latest/)
- [HJson](https://hjson.github.io/)


## Features

Easy to write for humans:
  - Special chars should be common to write, e.g. the at sign (`@`), a period
    (`.`) or a comma (`,`). Avoid programmer specific chars such as curly braces (`{`, `}`)
  - No requirement for quoting of string (`"quote"`)
  - Simple format for writing multi-line strings

Easy to parse:
  - first char in a line specifies content type: ("`@`" for new memo, "`.`" for new
    node, "` `" for continuing text, "`#`" for comment)
 
## Limitations

No nesting:
  - If you really need nested content, then try any of the other available text
    formats.

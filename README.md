# decompress_regex
A tool which lists all sentences which match the inputted regex.

## How to use
Run `deregex` and type regex from stdin.

## Currently supported regex symbols
| Symbol | Description                                  | ${regex} | `echo ${regex} \| deregex` |
|--------|----------------------------------------------|----------|--------------------------|
| ?      | Repeat just before character 0 times or once | abc?     | abc ab                   |
| \|     | Match before \| or after \|                  | foo\|bar | foo bar                  |

** No ascii characters are not supported! **

## Example
```shell
$deregex
abc
abc
abc?
abc
ab
foo|bar
bar
foo
f?o?o?|b?a?r?
ar
br
ba
fo
a
r
f
bar
oo

o
b
foo
^D
$
```
## License
The same license as Rust.

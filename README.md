# CHS (WIP)

- [Introduction](#introduction)
- [Development Milestones](#development-milestones)
- [How to use](#how-to-use)

## Introduction

CHS is a versatile programing language based on virtual machine written in Rust. It is designed for learning, experimenting.
This is a fist version of the languag, the current syntax resemble [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)) but with some c-like elements.

## How to use

(Not stable yet! you may have some troubles.)

```console
make chsc
make test
cd tmp/
chsc run <file.chs>
```

## Using CHS

- Hello world

```pascal
"Hello, world!\n" pstr
```

- Variables

```sh
var <name> <type> <value>;
set <name> <value>;
```

```pascal
var interger int 10;
var unsigned-interger uint 10;
var text str "Some text";
var mylist list[4] (1 2 3 4);

set interger 20;
set unsigned-interger 20 10 +;
set text "Other text";
set mylist[0] 2;
```

- if-statmets

```sh
<condition> if {
  <truly-block>
  else
  <false-block>
}
```

```c
1 1 = if {
  "Everything is fine\n" pstr
  else
  "Some thing is wrong\n" pstr
}
```

- while-loops

```sh
while <condition> {
  <while-block>
}
```

```c
0 while dup 100 < {
  dup print
  1 +
}
```

- stack operations

```sh
dup # [a] -> [a a]
dup2 # [a b] -> [a b a b]
over # [a b] -> [a b a]
pop # [a] -> []
swap # [a b] -> [b a]
```

- logical operations

```sh
= # equal
!= # not equal
< # less than
<= # less or equal than
> # greater than
>= # greater or equal than
|| # Logical or

```

- Bitwise operations

```sh
>> # right bit shift
<< # left bit shift
| # bit or
& # bit and
```

- Special operations

```sh
print # Dumps the top of the stack
pstr # prints the top of the stack after convertig to str
debug # show the current state of the CHSVM
```

## Development Milestones

- [X] [Turing Complete](exemples/rule110.chs)
- [ ] Automatic memory management with Reference Counting
- [ ] Macros Support
- [ ] Multthred Support
- [ ] Just-in-time

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## References & Inspirations

- BM: [GitHub - tsoding/bm](https://github.com/tsoding/bm)
- Porth: [GitLab - tsoding/porth](https://gitlab.com/tsoding/porth)
- SmallVM: [GitHub - tarekwiz/smallvm](https://github.com/tarekwiz/smallvm)
- IridiumVM: [GitHub - fhaynes/iridium](https://github.com/fhaynes/iridium)
- WebAssembly: [Rc memory management](https://binji.github.io/posts/webassembly-type-checking/)
- Inko: [GitHub - inko-lang/inko](https://github.com/inko-lang/inko)
- Boson-lang: [GitHub - Narasimha1997/boson-lang](https://github.com/Narasimha1997/boson-lang)

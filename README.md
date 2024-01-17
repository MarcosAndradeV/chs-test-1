# CHS (WIP)

- [Introduction](#introduction)
- [Development Milestones](#development-milestones)
- [How to use](#how-to-use)

## Introduction

CHS is a versatile programing language based on virtual machine written in Rust. It is designed for learning, experimenting.
The syntax resemble [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)) but with some c-like elements.

## How to use

(Not stable yet! you may have some troubles.)

```console
git clone git@github.com:MarcosAndradeV/chs-lang.git
cd chs-lang
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

```pascal
var <name> <value>;
set <name> <value>;
```

```pascal
var interger 10;
var text "Some text";
var mylist (1 2 3 4);

set interger 20;
set text "Other text";
set mylist[0] 2;
```

- Procedures

```pascal
func <name> (<args>) {
  <block>
}
```

```pascal
func sayhi() {
  "Hi" println
}

func add(a b) {
  a b +
}

sayhi
1 1 add
```

- if-statmets

```pascal
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

```pascal
while <condition> {
  <while-block>
}
```

```pascal
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

```c
= # equal
!= # not equal
< # less than
<= # less or equal than
> # greater than
>= # greater or equal than
|| # Logical or

```

- Bitwise operations

```c
>> # right bit shift
<< # left bit shift
| # bit or
& # bit and
```

- Special operations

```sh
print # prints the top of the stack
println # prints the top of the stack with a newline
len # get the length of the top of the stack
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

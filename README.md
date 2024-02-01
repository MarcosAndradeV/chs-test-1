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
"Hello, world!" println
```

- Variables

```pascal
var <name> <value>;
<value> := <name>
```

```pascal
var interger 10;
20 := another_interger

var text "Some text";
var mylist (1 2 3 4);

20 := interger
10 := another_interger
"Other text" := text
```

- if-statmets

```pascal
if <condition> {
  <truly-block>
  else
  <false-block>
}
```

```c
if 1 1 = {
  "Everything is fine\n" print
  else
  "Some thing is wrong\n" print
}
```

- while-loops

```pascal
while <condition> {
  <while-block>
}
```

```c
0 while dup 100 < {
  dup println
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

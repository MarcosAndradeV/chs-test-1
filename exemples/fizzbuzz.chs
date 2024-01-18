func main() {
    1 while dup 100 < {
        dup 15 mod 0 = if {
        "FizzBuzz\n" print
        else
        dup 3 mod 0 = if {
        "Fizz\n" print
        else
        dup 5 mod 0 = if {
        "Buzz\n" print
        else 
        dup println
        }}}
        1 +
    } pop
}
main
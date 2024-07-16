// Basic FizzBuzz program

fn main {
    1 while dup 100 < {
        dup 15 mod 0 = if {
            "FizzBuzz\n" print
        }
        dup 3 mod 0 = if {
            "Fizz\n" print
        }
        dup 5 mod 0 = if {
            "Buzz\n" print
        } else { 
            dup print "\n" print
        } 1 +
    } pop
}
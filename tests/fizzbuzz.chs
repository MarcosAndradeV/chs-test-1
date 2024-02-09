1 while dup 100 < {
    if dup 15 mod 0 = {
    "FizzBuzz\n" print
    }
    if dup 3 mod 0 = {
    "Fizz\n" print
    }
    if dup 5 mod 0 = {
    "Buzz\n" print
    else 
    dup print "\n" print
    }
    1 +
} pop
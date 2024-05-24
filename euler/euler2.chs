var LIMIT := 4000000;

var acc := 0;

var fib1 := 1;
var fib2 := 1;
var tmp  := 0;

fn println { print "\n" print }

while fib2 LIMIT < { 
    fib2 2 mod 0 = if {
        acc fib2 + := acc
    }
    fib1 fib2 + := tmp
    fib2 := fib1
    tmp := fib2
}

acc println 
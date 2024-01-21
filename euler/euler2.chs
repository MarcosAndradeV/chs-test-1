%def LIMIT 4000000

var acc 0;

var fib1 1;
var fib2 1;

while fib2 LIMIT < { 
    if fib2 2 mod 0 = {
        set acc acc fib2 +;
    }
    var tmp fib1 fib2 +;
    set fib1 fib2;
    set fib2 tmp;
}

acc println 
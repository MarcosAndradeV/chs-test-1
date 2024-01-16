%def LIMIT 4000000

var acc int 0;

var fib1 int 1;
var fib2 int 1;

while fib2 LIMIT < { 
    fib2 2 mod 0 = if {
        set acc acc fib2 +;
    }
    var tmp int fib1 fib2 +;
    set fib1 fib2;
    set fib2 tmp;
}

acc println 
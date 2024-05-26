fn println { print "\n" print }

LIMIT := 4000000;

acc  := 0;
fib1 := 1;
fib2 := 1;
tmp  := 0;

while (< fib2 LIMIT) {
    (= (mod fib2 2) 0) if { (+ fib2 acc) := acc }
    (+ fib1 fib2) 
    := tmp
    fib2 := fib1
    tmp := fib2
}


acc println
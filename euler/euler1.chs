fn println { print "\n" print }

acc := 0;

i := 0;
while (< i 1000) {
    (|| (= (mod i 3) 0)
        (= (mod i 5) 0))
    if { i acc + := acc }

    i 1 + := i
}

acc println

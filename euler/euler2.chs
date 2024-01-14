%def LIMIT 4000000

var acc int 0;

1 2 while over LIMIT < {
    over dup 2 mod 0 = if {
        acc load + acc swap store
        else
        pop
    }
    swap over +
}

acc load print 
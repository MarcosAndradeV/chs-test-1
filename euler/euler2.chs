%def LIMIT 4000000
%def acc 1

acc 0 store

1 2 while over LIMIT < {
    over dup 2 mod 0 = if {
        acc load + acc swap store
        else
        pop
    }
    swap over +
}

acc load print 
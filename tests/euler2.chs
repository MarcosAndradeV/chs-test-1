%def LIMIT 4000000

1 0 store


1 2 while over LIMIT < {
    over dup 2 mod 0 = if {
        1 load + 1 swap store
        else
        pop
    }
    swap over +
}

1 load print 
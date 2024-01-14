var acc int 0;

1 while dup 1000 < {
    dup 3 mod 0 = 
    over 5 mod 0 = || if {
        acc over acc load + store
    }
    1 +
}

acc load print
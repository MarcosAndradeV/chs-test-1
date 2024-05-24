// Finds the elememt in the array and returns its index, otherwise returns -1

fn find { // [any] any : int
    peek list elem {
        0 while dup list len < {
            list over idxget
            elem = if {
                list len 1 +
                else
                1 +
            }
        } dup list len > if {
            pop
            else
            -1
        }
    }
}

[1 5 8 2]
8
find
print
"\n" print
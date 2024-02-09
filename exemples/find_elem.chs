# Finds the elememt in the array and returns its index, otherwise returns -1

fn find {
    peek list elem {
        0 while dup list len < {
            if list over idxget
            elem = {
                list len 1 +
                else
                1 +
            }
        } if dup list len > {
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
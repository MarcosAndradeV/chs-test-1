// Finds the elememt in the array and returns its index, otherwise returns nil

fn find {
    peek list elem {
        0 while (< dup (len list)) {
            (= list (idxget over) elem) if { (len list) 1 + } else { 1 + }
        } 
        (> (len list)) if {} else { nil }
    }
}

[1 5 8 2]
8
find
print
"\n" print
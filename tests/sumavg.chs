// [1 5 8 2]
// Sum
// Length
// Divide

fn sum { // [int] -> int
    := xs
    0 := i
    0 := x
    while i xs len < {
        x xs i idxget + := x
        i 1 + := i
    }
    x
}

fn main {
    [1 5 8 2]
    . sum :
    len
    /
    
    debug
}

fn length { // int [a] -> int
    := xs
    := i
    xs head nil = if {
        i
    } else {
        i 1 + xs tail length
    }
}

fn main {
   0 [1 2 3 4] length print
}

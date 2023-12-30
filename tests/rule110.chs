# WIP

func main {
     1 1 store
    28 1 store

    0 while dup 10 > {
        0 while dup 10 > {
            dup load if {
                    30 42 store
                else
                    30 32 store
            }
            0 30 write
            inc
        } pop
        30 10 store
        0 30 write
        inc
    } pop

}
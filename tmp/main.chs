# WIP

func main {
   1 1 store
   28 1 store
   0 while dup 30 > {
    0 while dup 30 > {
        dup load if {
            30 42 store
            else
            30 32 store
        }
        1 30 write
        inc
    } pop
    30 10 store
    1 30 write
    1 while dup 28 > {

    }
    inc
   } pop

}
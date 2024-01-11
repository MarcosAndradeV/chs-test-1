%def BOARD_SIZE 100

mem 1 + 1 store

0 while dup BOARD_SIZE 2 - < {
    0 while dup BOARD_SIZE < {
        dup mem + load if {
            mem BOARD_SIZE + 42 store
            else
            mem BOARD_SIZE + 32 store
        }
        1 mem BOARD_SIZE + write
        1 + 
    } pop
    # newline
    "\n" pstr
    # pattern
    mem load 1 << mem 1 + load |
    1 while dup BOARD_SIZE 1 - < {
        # idx (((pat << 1) & 7) | cell)
        swap 1 << 7 & over mem + load |
        dup2 110 swap >> 1 & swap mem + swap store
        swap
        1 +
    } pop pop
    1 +
} pop
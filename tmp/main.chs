
func fib {
    2 while dup 0 < {
        #dup print
        1 -
    }
    ret
}

func main {
    fib
    call
    hlt
}
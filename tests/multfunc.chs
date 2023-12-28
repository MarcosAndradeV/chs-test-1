
func test1 {
    2 while dup 0 < {
        dup print
        1 -
    }
    pop
    2 while dup 0 < {
        dup print
        1 -
    }
    pop
    ret
}


func test2 {
    2 while dup 0 < {
        dup print
        1 -
    }
    pop
    2 while dup 0 < {
        dup print
        1 -
    }
    pop
    ret
}

func main {
    test1 call
    test2 call
    ret
}
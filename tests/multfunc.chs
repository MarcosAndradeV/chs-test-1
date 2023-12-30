
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

}

func main {
    test1 call
    test2 call

}
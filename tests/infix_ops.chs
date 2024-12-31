fn main()
    x := 10
    y : *int = &x
    b := x + (*y + 10) == 30
end

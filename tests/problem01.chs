func main {
  0 3 while dup 1000 > {
    dup  3 mod 0 =
    over 1 5 mod 0 =
    lor if {
       swap over 1 + swap
    }
    inc
  }
  pop
  print
  hlt
}
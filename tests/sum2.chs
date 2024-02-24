fn println {
  print "\n" print
}
fn main {
  1 0 while over 100 <= {
    peek idx acc {
      if idx 2 mod 0 = {
        idx 1 + acc idx +
        else
        idx 1 + acc
      }
    }
  }
  println
  pop

  0 0 while over 100 <= {
    peek idx acc {
      idx 2 + acc idx +
    }
  }
  println
  pop

  0 0 while over 100 <= {
    over + swap 2 + swap
  }
  println
  pop
}
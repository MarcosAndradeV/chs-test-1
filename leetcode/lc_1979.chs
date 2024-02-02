fn min {
  peek l {
    l len 1 -
    l 0 idxget
    while over 0 >= { 
      peek idx acc {
        idx 1 -
        if
        acc
        l idx idxget < {
          acc
          else
          l idx idxget
        }
      }
    } swap pop
  }
}

fn max {
  peek l {
    l len 1 -
    l 0 idxget
    while over 0 >= { 
      peek idx acc {
        idx 1 -
        if
        acc
        l idx idxget > {
          acc
          else
          l idx idxget
        }
      }
    } swap pop
  }
}

fn gcd {
  while dup 0 != {
    peek a b {
      b a b mod
    }
  } pop
}

fn gcd_rec {
  if dup 0 = {
    pop
    else
    peek a b { b a b mod } gcd_rec
  }
}

@(2 5 6 9 10) dup min swap max gcd println
@(7 5 6 8 3) dup min swap max gcd println
@(3 3) dup min swap max gcd println
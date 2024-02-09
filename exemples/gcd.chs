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

10 2 gcd print "\n" print
10 2 gcd_rec print "\n" print

fn min {
  peek l {
    (- (len l) 1)
    (idxget l 0)
    while (>= over 0) { 
      peek idx acc {
        (- idx 1) 
        (< acc (idxget l idx)) if { acc } else { (idxget l idx) }
      }
    } : drop
  }
}

fn max {
  peek l {
    (- (len l) 1)
    (idxget l 0)
    while (>= over 0) { 
      peek idx acc {
        (- idx 1) 
        (> acc (idxget l idx)) if { acc } else { (idxget l idx) }
      }
    } : drop
  }
}

fn gcd {
  while dup 0 != {
    (mod over rot)
  } pop
}

fn gcd_rec {
  dup 0 = if { pop } else { (mod over rot) gcd_rec }
}

fn println { print (print "\n") }

[2 5 6 9 10] dup min : max gcd println
[7 5 6 8 3] dup min : max gcd println
[3 3] dup min : max gcd println
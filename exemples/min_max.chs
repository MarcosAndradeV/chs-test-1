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

[1 2 3] min print "\n" print
[1 2 3] max print "\n" print

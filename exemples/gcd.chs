fn gcd {
  while dup 0 != {
    (mod over rot)
  } pop
}

fn gcd_rec {
  dup 0 = if { pop } else { (mod over rot) gcd_rec }
}

10 2 gcd print "\n" print
10 2 gcd_rec print "\n" print

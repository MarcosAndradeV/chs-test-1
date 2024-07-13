fn println { print (print "\n") }

fn to_list { peek x { [x] } }

fn foreach { : . (!= head nil) if { over over head : call tail : foreach } else { drop drop } }

fn map { : . (!= head nil) if { over over head : [call] : tail rot map ++ } else { drop drop [] } }

fn repeat {
    peek ls count {
        ls len 1 < if { nil } else {
            ls 0 while dup count < {
                peek ls i {
                    ls ls head to_list ++
                    i 1 +
                }
            } drop
        }
    }
}

fn show {
    peek ls {
        ls ls len
        0 while over over : < {
            peek ls ls_len i {
                ls i idxget print
                ls
                ls_len
                i 1 +
            }
        } drop drop drop
    }
}

fn dip {
    peek elem f {
        f call
        elem
    }
}


fn diptest {
    3 2 1 { [+] } dip to_list : ++ [1 5] != if { error "dip test failed" }
}


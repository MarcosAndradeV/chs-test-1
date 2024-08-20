// Checks if a number is palindrome

fn is_palindrome { // int -> bool
    := x
    (< x 0) if {
        false
    } else {
        0 // remainder
        0 // reverse
        x // temp
        while (!= dup 0) {
            peek rem rev temp {
                temp 10 mod dup
                (* rev 10) +
                temp 10 /
            }
        } drop swap drop
        x =
    }

}

fn main {
    101 is_palindrome if { "True\n" puts } else { "False\n" puts }
}

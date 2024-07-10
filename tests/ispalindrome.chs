// Checks if a number is palindrome
101 := Palindrome

Palindrome := x

0 := reverse
0 := remainder
Palindrome := temp

(< x 0) if { "False\n" print } else {
    while (!= temp 0) {
        temp 10 mod := remainder
        (+ (* reverse 10) remainder) := reverse
        temp 10 / := temp
    }
    (= reverse x) if { "True\n" print } else { "False\n" print }
}
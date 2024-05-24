# Checks if a number is palindrome
var Palindrome := 101;

var x := Palindrome;

var reverse := 0;
var remainder := 0;
var temp := Palindrome;

x 0 < if {
    "False\n" print
    else
    while temp 0 != {
        temp 10 mod := remainder
        reverse 10 * remainder + := reverse
        temp 10 / := temp
    }

    reverse x = if {
        "True\n" print
        else
        "False\n" print
    }
}
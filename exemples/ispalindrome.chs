# Is Palindrome
%def Palindrome 101

var x Palindrome;

var reverse 0;
var remainder 0;
var temp Palindrome;

if x 0 < { "False" println hlt }

while temp 0 != {
    temp 10 mod := remainder
    reverse 10 * remainder + := reverse
    temp 10 / := temp
}

if reverse x = {
    "True" println
    else
    "False" println
}
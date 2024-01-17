# Is Palindrome
%def Palindrome 101

var x Palindrome;

var reverse 0;
var remainder 0;
var temp Palindrome;

x 0 < if { "False" println hlt }

while temp 0 != {
    set remainder temp 10 mod;
    set reverse reverse 10 * remainder + ;
    set temp temp 10 /;
}

reverse x = if {
    "True" println
    else
    "False" println
}

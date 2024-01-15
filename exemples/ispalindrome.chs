# Is Palindrome
%def Palindrome 101

var x int Palindrome;

var reverse int 0;
var remainder int 0;
var temp int Palindrome;

x 0 < if  { "False\n" pstr hlt }

while temp 0 != {
    set remainder temp 10 mod;
    set reverse reverse 10 * remainder + ;
    set temp temp 10 /;
}

reverse x = if {
    "True\n" pstr
    else
    "False\n" pstr
}

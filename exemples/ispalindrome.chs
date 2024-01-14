# Is Palindrome
%def Palindrome 10201

var x int Palindrome;

var reverse int 0;
var remainder int 0;
var temp int Palindrome;

x load 0 < if  { "False" pstr hlt }

while temp load 0 != {
    remainder temp load 10 mod store
    reverse reverse load 10 * remainder load + store
    temp temp load 10 / store
}

reverse load x load = if {
    "True\n" pstr
    else
    "False\n" pstr
}

* declaring a function with specific return type (e.g a string)
then assigning that functions return to a variable with a different type gives a type mismatch error = correct
then trying to declare that variable again with the correct type gives duplicate declaration message (should not have been declared in the first place)
then trying to print the wrongfully declared variable panics

fn test() { return "abc" ;}
let c: int -> test(); // type mismatch
let c -> test(); // duplicate declaration messsage
print c; // panics

* declaring and calling function with keyword gives semantic error: Undeclared variable "function_name"
* calling function with print also prints previous results of that function call
    - e.g. fn test(x, y) { return x * y; }
            print test(2, 5) => 10
            print (2, 6) => 10, 12
            ..
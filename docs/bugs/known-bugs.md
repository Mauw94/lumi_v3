--------------------------------------------------------------------------

    r#"
        fn test(a) { 
            return test(a - 1);
        }
    "#

    this results in error: Undeclared variable 'test'

-----------------------------------------------------------------------------

lumi> let x -> 5;
lumi> if (x < 7) { print "hehexd"; }

Semantic error: Type mismatch: expected boolean, found Undefined at line 1, column 38

-----------------------------------------------------------------------------
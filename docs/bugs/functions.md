    r#"
        fn add(a, b) {
            return a + b;
        }

        let result: int -> add(3, 4);
        result * 2;
    "#,

    this results in a value of String("7undefined") on the stack.
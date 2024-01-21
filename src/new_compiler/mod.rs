/*
    lexer -> parser -> (AST | IR) -> Intrs

    Source:
        if 1 1 + 2 = { "Hello" println }
    TokenStream:
        Token::If("if")
        Token::IntLiteral("1")
        Token::IntLiteral("1")
        Token::AddOp("+")
        Token::IntLiteral("2")
        Token::EqOp("=")
        Token::CurlyOpen("{")
        Token::StrLiteral("Hello")
        Token::Identifier("println")
        Token::CurlyClose("}")

    Ir:
        If:
            cond:
                PushLiteral(int : 1)
                PushLiteral(int : 1)
                Add(int int : int)
                PushLiteral(int : 2)
                Eq(any any : bool)
            If-block:
                PushLiteral(str : "Hello")
                OpWord(println)
    Instr:
        0 -> PushLabel(1)
        1 -> Const(0)
        2 -> Const(1)
        3 -> Add
        4 -> Const(2)
        5 -> Eq
        6 -> JmpIf(9)
        7 -> Const(3)
        8 -> Println
        9 -> DropLabel(1)
    
*/


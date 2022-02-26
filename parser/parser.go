package parser

import (
	"alterae/lang/lexer"
	"fmt"
)

func Parse(s lexer.TokenStream) Ast {
	// todo
	fmt.Println(s)
	return Ast{}
}

// Ast is an abstract syntax tree
type Ast struct {
}

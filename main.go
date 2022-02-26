package main

import (
	"alterae/lang/compiler"
	"alterae/lang/lexer"
	"alterae/lang/parser"
	"alterae/lang/vm"
	"os"
)

func main() {
	file := os.Args[1]
	tokens := lexer.Lex(file)
	ast := parser.Parse(tokens)
	bytes := compiler.Compile(ast)
	vm.Run(bytes)
}

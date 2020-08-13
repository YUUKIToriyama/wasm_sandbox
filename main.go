package main
import "fmt"
import "syscall/js"

func add(this js.Value, i []js.Value) interface{} {
	return js.ValueOf(i[0].Int()+i[1].Int())
}

func registerCallbacks() {
	js.Global().Set("add", js.FuncOf(add))
}

func main() {
	fmt.Println("Hello, WebAssembly!")
	c := make(chan struct{}, 0)
	registerCallbacks()
	<-c
}

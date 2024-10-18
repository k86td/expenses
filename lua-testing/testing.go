package main

// this folder contains a simple test to do an HTTP request and decode
// JSON as a proof of concept.

import (
	"github.com/yuin/gopher-lua"
	"gitlab.com/megalithic-llc/gluasocket"
	luajson "layeh.com/gopher-json"
)

func main() {
	L := lua.NewState()
	defer L.Close()

	// load Luasocket Go implementation
	gluasocket.Preload(L)

	// load JSON module
	luajson.Preload(L)

	if err := L.DoFile("./test.lua"); err != nil {
		panic(err)
	}
}

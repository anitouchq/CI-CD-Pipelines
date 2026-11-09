package main

import "testing"

func TestGreeting(t *testing.T) {
	greeting := "Hello from Go in Docker!"
	if greeting == "" {
		t.Fatal("greeting must not be empty")
	}
}

package fixtures

// Expected metrics:
// add: cognitive=0, cyclomatic=1, sloc=TBD
func add(a int, b int) int {
	result := a + b
	return result
}

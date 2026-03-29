package fixtures

// Expected metrics:
// factorial: cognitive=TBD, cyclomatic=TBD, sloc=TBD
func factorial(n int) int {
	if n <= 1 {
		return 1
	} else {
		return n * factorial(n-1)
	}
}

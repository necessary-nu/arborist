package fixtures

// Expected metrics:
// checkAll: cognitive=TBD, cyclomatic=TBD, sloc=TBD
func checkAll(a bool, b bool, c bool) bool {
	if a && b && c {
		return true
	}
	return false
}

// checkMixed: cognitive=TBD, cyclomatic=TBD, sloc=TBD
func checkMixed(a bool, b bool, c bool) bool {
	if a && b || c {
		return true
	}
	return false
}

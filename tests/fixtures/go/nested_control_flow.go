package fixtures

// Expected metrics:
// process: cognitive=TBD, cyclomatic=TBD, sloc=TBD
func process(items []int) int {
	total := 0
	if len(items) > 0 {
		for _, item := range items {
			if item > 0 {
				total += item
			}
		}
	}
	return total
}

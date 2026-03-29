package fixtures

// Expected metrics:
// applyFilter: cognitive=TBD, cyclomatic=TBD, sloc=TBD
func applyFilter(items []int) []int {
	threshold := 10
	result := make([]int, 0)
	for _, item := range items {
		if item > threshold {
			result = append(result, item)
		}
	}
	return result
}

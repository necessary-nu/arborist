// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

function classify(x: number): string {
    if (x > 100) {
        return "high";
    } else if (x > 50) {
        return "medium";
    } else if (x > 0) {
        return "low";
    } else {
        return "negative";
    }
}

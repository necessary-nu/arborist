// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

function applyFilter(items: number[]): number[] {
    const threshold = 10;
    const filtered = items.filter((x) => {
        if (x > threshold) {
            return true;
        } else {
            return false;
        }
    });
    return filtered;
}

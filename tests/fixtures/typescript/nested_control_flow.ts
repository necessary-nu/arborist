// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

function process(items: number[]): number {
    let total = 0;
    if (items.length > 0) {
        for (let i = 0; i < items.length; i++) {
            if (items[i] > 0) {
                total += items[i];
            }
        }
    }
    return total;
}

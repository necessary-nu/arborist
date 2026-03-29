// Expected metrics:
// process: cognitive=TBD, cyclomatic=TBD, sloc=TBD
function process(items) {
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

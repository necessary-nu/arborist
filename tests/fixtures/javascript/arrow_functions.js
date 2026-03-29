// Expected metrics:
// applyFilter: cognitive=TBD, cyclomatic=TBD, sloc=TBD
function applyFilter(items) {
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

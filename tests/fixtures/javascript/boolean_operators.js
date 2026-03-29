// Expected metrics:
// checkAll: cognitive=TBD, cyclomatic=TBD, sloc=TBD
function checkAll(a, b, c) {
    if (a && b && c) {
        return true;
    }
    return false;
}

// checkMixed: cognitive=TBD, cyclomatic=TBD, sloc=TBD
function checkMixed(a, b, c) {
    if (a && b || c) {
        return true;
    }
    return false;
}

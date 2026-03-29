// Expected metrics:
// checkAll: cognitive=TBD, cyclomatic=TBD, sloc=TBD
bool checkAll(bool a, bool b, bool c) {
    if (a && b && c) {
        return true;
    }
    return false;
}

// checkMixed: cognitive=TBD, cyclomatic=TBD, sloc=TBD
bool checkMixed(bool a, bool b, bool c) {
    if (a && b || c) {
        return true;
    }
    return false;
}

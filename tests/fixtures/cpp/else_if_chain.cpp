// Expected metrics:
// classify: cognitive=TBD, cyclomatic=TBD, sloc=TBD
const char* classify(int x) {
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

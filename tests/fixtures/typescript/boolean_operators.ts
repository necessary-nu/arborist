// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

function checkAll(a: boolean, b: boolean, c: boolean): boolean {
    if (a && b && c) {
        return true;
    }
    return false;
}

function checkMixed(a: boolean, b: boolean, c: boolean): boolean {
    if (a && b || c) {
        return true;
    }
    return false;
}

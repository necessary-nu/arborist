// Expected metrics:
// applyFilter: cognitive=TBD, cyclomatic=TBD, sloc=TBD
#include <vector>
#include <algorithm>

std::vector<int> applyFilter(const std::vector<int>& items) {
    int threshold = 10;
    std::vector<int> result;
    std::copy_if(items.begin(), items.end(), std::back_inserter(result),
        [threshold](int x) { return x > threshold; });
    return result;
}

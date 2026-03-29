// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

using System.Linq;
using System.Collections.Generic;

class LambdaDelegates {
    List<int> ApplyFilter(List<int> items) {
        int threshold = 10;
        return items.Where(x => x > threshold).ToList();
    }
}

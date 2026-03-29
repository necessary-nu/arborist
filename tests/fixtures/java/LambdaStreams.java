// Expected metrics:
//   cyclomatic_complexity: TBD
//   cognitive_complexity: TBD
//   lines_of_code: TBD
//   nesting_depth: TBD

import java.util.List;
import java.util.stream.Collectors;

class LambdaStreams {
    List<Integer> applyFilter(List<Integer> items) {
        int threshold = 10;
        return items.stream()
            .filter(x -> x > threshold)
            .collect(Collectors.toList());
    }
}

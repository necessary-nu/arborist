<?php
// Expected metrics:
// process: cognitive=TBD, cyclomatic=TBD, sloc=TBD
function process($items) {
    $total = 0;
    if (count($items) > 0) {
        for ($i = 0; $i < count($items); $i++) {
            if ($items[$i] > 0) {
                $total += $items[$i];
            }
        }
    }
    return $total;
}

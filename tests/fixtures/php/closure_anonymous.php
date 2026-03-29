<?php
// Expected metrics:
// applyFilter: cognitive=TBD, cyclomatic=TBD, sloc=TBD
function applyFilter($items) {
    $threshold = 10;
    $filtered = array_filter($items, function($x) use ($threshold) {
        if ($x > $threshold) {
            return true;
        } else {
            return false;
        }
    });
    return $filtered;
}

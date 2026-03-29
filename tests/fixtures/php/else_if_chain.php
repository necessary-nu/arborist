<?php
// Expected metrics:
// classify: cognitive=TBD, cyclomatic=TBD, sloc=TBD
function classify($x) {
    if ($x > 100) {
        return "high";
    } elseif ($x > 50) {
        return "medium";
    } elseif ($x > 0) {
        return "low";
    } else {
        return "negative";
    }
}

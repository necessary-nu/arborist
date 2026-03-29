<?php
// Expected metrics:
// factorial: cognitive=TBD, cyclomatic=TBD, sloc=TBD
function factorial($n) {
    if ($n <= 1) {
        return 1;
    } else {
        return $n * factorial($n - 1);
    }
}

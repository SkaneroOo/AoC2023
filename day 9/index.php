<?php

$data = file_get_contents('./input.txt');

$lines = explode("\n", $data);
$numbers = [];

for ($i = 0; $i < count($lines); $i++) {
    $numbers[$i] = explode(" ", $lines[$i]);
    for ($j = 0; $j < count($numbers[$i]); $j++) {
        $numbers[$i][$j] = (int)$numbers[$i][$j];
    }
}

function get_prediction($line) {
    $temp = [];
    for ($i = 0; $i < count($line)-1; $i++) {
        $temp[$i] = $line[$i+1] - $line[$i];
    }
    if (empty(array_filter($temp ))) {
        return $line[count($line)-1];
    }
    return get_prediction($temp) + $line[count($line)-1];
}

$total = 0;
$reversed = 0;

for ($i = 0; $i < count($numbers); $i++) {
    $total += get_prediction($numbers[$i]);
    $reversed += get_prediction(array_reverse($numbers[$i]));
}

print($total . "\n");
print($reversed . "\n");

?>
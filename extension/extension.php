<?php

// Example inputs
$currentTime = "14:54:20";
$currentTerminal = "Kungsängen";

$startTime = microtime(true);
// Call the Rust function
$result = find_departing_trains_php($currentTime, $currentTerminal);
$endTime = microtime(true);
$elapsedTime = $endTime - $startTime;

// Example: Print the result
print_r($result);
print_r($elapsedTime."\n");

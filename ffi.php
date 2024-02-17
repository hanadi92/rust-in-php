<?php
extension_loaded('ffi') or die('FFI extension is not enabled.');

// Load the Rust library
$ffi = FFI::cdef("
    char* find_departing_trains(char* current_time, char* current_terminal);
    void free_rust_string(char* ptr);
", __DIR__ . "/target/release/libmytrain.dylib");

// Example inputs
$currentTime = "14:54:20";
$currentTerminal = "Kungsängen";

$startTime = microtime(true);
// Call the Rust function
$result = $ffi->find_departing_trains($currentTime, $currentTerminal);
$endTime = microtime(true);
$elapsedTime = $endTime - $startTime;
print_r($elapsedTime);

// Convert the result to a PHP string
$resultStr = FFI::string($result);

// Free the memory allocated by Rust
$ffi->free_rust_string($result);

// Deserialize the JSON string into a PHP array
$departingTrains = json_decode($resultStr, true);

// Example: Print the result
print_r($departingTrains);

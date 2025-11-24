#!/bin/bash
# Shell script example

# Function to greet user
greet() {
    echo "Hello, $1!"
}

# Function to calculate sum
function calculate_sum() {
    local a=$1
    local b=$2
    echo $((a + b))
}

# Check if file exists
file_exists() {
    if [ -f "$1" ]; then
        return 0
    else
        return 1
    fi
}

# Main execution
greet "World"
result=$(calculate_sum 10 20)
echo "Sum: $result"

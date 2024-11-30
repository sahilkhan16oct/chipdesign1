#!/bin/bash

# Get the absolute path of the directory where the script is located
SCRIPT_DIR=$(dirname "$(realpath "$0")")

# Function to find the project root by searching upwards for 'chipdesign1'
find_project_root() {
    local dir="$1"
    while [ "$dir" != "/" ]; do
        if [ -d "$dir/chipdesign1" ]; then
            echo "$dir/chipdesign1"
            return
        fi
        dir=$(dirname "$dir")
    done
    echo "Error: chipdesign1 directory not found."
    exit 1
}

# Call the function to find the project root
PROJECT_ROOT=$(find_project_root "$SCRIPT_DIR")

# Now construct the absolute path to the command_line executable
COMMAND_LINE_PATH="$PROJECT_ROOT/verifire.command_line_14/target/release/command_line"

# Print the resolved path for debugging
echo "Resolved command_line path: $COMMAND_LINE_PATH"

# Check if the file exists before trying to run it
if [ -f "$COMMAND_LINE_PATH" ]; then
    echo "Running command_line with DRC_deck.json"
    $COMMAND_LINE_PATH --file DRC_deck.json
else
    echo "Error: $COMMAND_LINE_PATH not found."
    exit 1
fi


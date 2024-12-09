#!/bin/bash

# Define the absolute path to your project directory
PROJECT_ROOT="/home/ubuntu/innoveotech/backend/chipdesign1"
TARGET_DIR="$PROJECT_ROOT/verifire.command_line_14/test_runner/sahil"
LOG_FILE="$PROJECT_ROOT/delete_rev_files.log"

# Log script start
echo "Script started at $(date)" >> "$LOG_FILE"

# Find and delete .rve files older than 3 minutes, log deleted files
find "$TARGET_DIR" -type f -name "*.rve" -mmin +1440 -exec echo "Deleting: {}" >> "$LOG_FILE" \; -exec rm -f {} \;

# Log script completion
echo "Script completed at $(date)" >> "$LOG_FILE"


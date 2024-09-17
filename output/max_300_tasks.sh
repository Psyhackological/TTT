#!/bin/bash

INPUT_FILE="to_todoist.csv"
OUTPUT_FILE="to_todoist_300.csv"

MAX_TASKS=300
LINES_TO_READ=$MAX_TASKS
TASK_COUNT=$(head -n $LINES_TO_READ "$INPUT_FILE" | grep -c 'task')

while [[ $TASK_COUNT -ne $MAX_TASKS ]]; do
    TASKS_TO_FILL=$((MAX_TASKS - TASK_COUNT))
    LINES_TO_READ=$((LINES_TO_READ + TASKS_TO_FILL))
    TASK_COUNT=$(head -n $LINES_TO_READ "$INPUT_FILE" | grep -c 'task')

    if [[ $TASK_COUNT -ge $MAX_TASKS ]]; then
        break
    fi
done

head -n $LINES_TO_READ "$INPUT_FILE" >"$OUTPUT_FILE"

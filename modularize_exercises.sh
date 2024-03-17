#!/bin/bash

# Clear the file
truncate -s 0 exercises/exercises.rs

# Write the initial line to the file
echo '// This file was auto-generated with ../modularize_exercises.sh script' >> exercises/exercises.rs

# Loop through all the files in the exercises directory and add them to the exercises.rs file
find ./exercises -type f -name "*.rs" ! -name "exercises.rs" -print0 | sort -z | while IFS= read -r -d '' file; do
    # Remove the 'exercises/' prefix from the file path
    path=${file#./exercises/}

    # Write the module path only if the file is in a subdirectory
    if [[ -n "$path" && "$path" == */* ]]; then
        echo "#[path=\"$path\"]" >> exercises/exercises.rs
    fi
    # Write the module name to the file
    echo "mod $(basename "$file" .rs);" >> exercises/exercises.rs
done

echo 'Done'
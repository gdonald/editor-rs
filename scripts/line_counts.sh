#!/usr/bin/env bash
# Count lines in Rust source files and output sorted by line count (descending).
# Used for identifying files that may need refactoring.

# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."

# Workspace crates to check
CRATES=("editor-core" "editor-tui" "editor-gui")
SEARCH_DIRS=()

# Collect all src and tests directories from workspace crates
for crate in "${CRATES[@]}"; do
  crate_dir="$PROJECT_ROOT/$crate"
  if [ -d "$crate_dir/src" ]; then
    SEARCH_DIRS+=("$crate_dir/src")
  fi
  if [ -d "$crate_dir/tests" ]; then
    SEARCH_DIRS+=("$crate_dir/tests")
  fi
done

# Check if we found any directories
if [ ${#SEARCH_DIRS[@]} -eq 0 ]; then
  echo "Error: No src or tests directories found in workspace crates"
  exit 1
fi

# Find all .rs files and count lines
declare -a files
declare -a counts

while IFS= read -r file; do
  if [ -f "$file" ]; then
    line_count=$(wc -l < "$file")
    relative_path="${file#$PROJECT_ROOT/}"
    files+=("$relative_path")
    counts+=("$line_count")
  fi
done < <(find "${SEARCH_DIRS[@]}" -type f -name "*.rs" | sort)

# Check if any files were found
if [ ${#files[@]} -eq 0 ]; then
  echo "No Rust files found in workspace crates"
  exit 0
fi

# Sort arrays by line count (descending)
# Create array of indices sorted by count
indices=()
for i in "${!counts[@]}"; do
  indices+=("$i")
done

# Bubble sort indices based on counts (descending)
for ((i = 0; i < ${#indices[@]}; i++)); do
  for ((j = i + 1; j < ${#indices[@]}; j++)); do
    if [ "${counts[${indices[$i]}]}" -lt "${counts[${indices[$j]}]}" ]; then
      # Swap indices
      tmp="${indices[$i]}"
      indices[$i]="${indices[$j]}"
      indices[$j]="$tmp"
    fi
  done
done

# Output results
echo "================================================================================"
echo "LINE COUNT REPORT - Rust Workspace"
echo "================================================================================"
echo ""
printf "%-65s %10s\n" "File" "Lines"
printf "%s\n" "-------------------------------------------------------------------------------"

total_lines=0
total_src_lines=0
total_test_lines=0
files_shown=0

for idx in "${indices[@]}"; do
  file="${files[$idx]}"
  count="${counts[$idx]}"
  total_lines=$((total_lines + count))

  # Track src vs test lines
  if [[ "$file" == *"/tests/"* ]]; then
    total_test_lines=$((total_test_lines + count))
  else
    total_src_lines=$((total_src_lines + count))
  fi

  # Only show files with 50+ lines
  if [ "$count" -ge 50 ]; then
    printf "%-65s %10d\n" "$file" "$count"
    files_shown=$((files_shown + 1))
  fi
done

printf "%s\n" "-------------------------------------------------------------------------------"
printf "%-65s %10d\n" "Total Lines" "$total_lines"
printf "%-65s %10d\n" "  Source Lines (src/)" "$total_src_lines"
printf "%-65s %10d\n" "  Test Lines (tests/)" "$total_test_lines"
printf "\n"
printf "Total files: %d (showing %d files with 50+ lines)\n" "${#files[@]}" "$files_shown"
echo "================================================================================"

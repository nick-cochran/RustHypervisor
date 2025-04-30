import subprocess
import sys
import re
import pandas as pd
from datetime import datetime

def parse_output(output):
    instructions = []
    times = []
    lines = output.split('\n')

    current_instruction = None

    for line in lines:
        instr_match = re.match(r'^-> (alloc|free) (\d+)', line)
        time_match = re.match(r'^(Alloc|Free) ran in (\d+) microseconds and (\d+) nanoseconds', line)

        if instr_match:
            instr_type, number = instr_match.groups()
            current_instruction = f"{instr_type} {number}"
            instructions.append(current_instruction)

        elif time_match and current_instruction:
            microseconds, nanoseconds = map(int, time_match.groups()[1:])
            total_nanoseconds = microseconds * 1_000 + nanoseconds
            times.append(total_nanoseconds)
            current_instruction = None

    alloc_summary = {}
    free_summary = {}
    overall_summary = {}

    for idx, line in enumerate(lines):
        if line.strip() == "Alloc Time Stats:":
            min_line = lines[idx+1]
            avg_line = lines[idx+2]
            max_line = lines[idx+3]

            min_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', min_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', min_line).group(2))
            avg_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', avg_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', avg_line).group(2))
            max_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', max_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', max_line).group(2))
            alloc_summary = {"Min": min_time, "Avg": avg_time, "Max": max_time}

        if line.strip() == "Free Time Stats:":
            min_line = lines[idx+1]
            avg_line = lines[idx+2]
            max_line = lines[idx+3]

            min_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', min_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', min_line).group(2))
            avg_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', avg_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', avg_line).group(2))
            max_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', max_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', max_line).group(2))
            free_summary = {"Min": min_time, "Avg": avg_time, "Max": max_time}

        if line.strip() == "Overall Time Stats:":
            min_line = lines[idx+1]
            avg_line = lines[idx+2]
            max_line = lines[idx+3]

            min_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', min_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', min_line).group(2))
            avg_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', avg_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', avg_line).group(2))
            max_time = int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', max_line).group(1)) * 1_000 + int(re.search(r'(\d+) microseconds and (\d+) nanoseconds', max_line).group(2))
            overall_summary = {"Min": min_time, "Avg": avg_time, "Max": max_time}

    return instructions, times, alloc_summary, free_summary, overall_summary

def run_rust_program(input_file):
    result = subprocess.run(
        ['cargo', 'run', '--release', '--', input_file],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    return result.stdout

def main():
    if len(sys.argv) != 3:
        print("Usage: python script.py <num_runs> <input_file>")
        sys.exit(1)

    num_runs = int(sys.argv[1])
    input_file = sys.argv[2]

    all_runs_data = {}
    instruction_list = None
    alloc_summaries = []
    free_summaries = []
    overall_summaries = []

    for run_idx in range(num_runs):
        print(f"Running iteration {run_idx + 1}...")
        output = run_rust_program(input_file)
        instructions, times, alloc_summary, free_summary, overall_summary = parse_output(output)

        if instruction_list is None:
            instruction_list = instructions
        else:
            if instruction_list != instructions:
                print("Mismatch in instruction order between runs!")
                sys.exit(1)

        all_runs_data[f"Run {run_idx + 1}"] = times
        alloc_summaries.append(alloc_summary)
        free_summaries.append(free_summary)
        overall_summaries.append(overall_summary)

    expanded_instructions = instruction_list + ["", "Alloc Min", "Alloc Avg", "Alloc Max", "Free Min", "Free Avg", "Free Max", "Overall Min", "Overall Avg", "Overall Max"]

    df = pd.DataFrame({"Instruction": expanded_instructions})

    for run_idx in range(num_runs):
        run_times = all_runs_data[f"Run {run_idx + 1}"]
        run_times_expanded = run_times + [None,
                                          alloc_summaries[run_idx]["Min"], alloc_summaries[run_idx]["Avg"], alloc_summaries[run_idx]["Max"],
                                          free_summaries[run_idx]["Min"], free_summaries[run_idx]["Avg"], free_summaries[run_idx]["Max"],
                                          overall_summaries[run_idx]["Min"], overall_summaries[run_idx]["Avg"], overall_summaries[run_idx]["Max"]]
        df[f"Run {run_idx + 1}"] = run_times_expanded

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_filename = f"benchmark_results_{timestamp}.xlsx"

    with pd.ExcelWriter(output_filename) as writer:
        df.to_excel(writer, sheet_name="Benchmark", index=False)

    print(f"Spreadsheet saved to {output_filename}")

if __name__ == "__main__":
    main()

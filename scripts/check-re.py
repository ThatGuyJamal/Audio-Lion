import os
import rustpythonparser

# Take user input for directory path
dir_path = input("Enter directory path: ")

# Get all Rust files in the directory
rust_files = [f for f in os.listdir(dir_path) if f.endswith('.rs')]

# Iterate through each Rust file and check for missing semicolons
for rust_file in rust_files:
    with open(os.path.join(dir_path, rust_file), 'r') as file:
        rust_code = file.read()
        parsed_code = rustpythonparser.parse(rust_code)

    # Add semicolons to missing statements and write back to file
    with open(os.path.join(dir_path, rust_file), 'w') as file:
        for stmt in parsed_code.statements:
            if isinstance(stmt, rustpythonparser.Statement):
                if stmt.terminator != ';':
                    file.write(f"{stmt.to_code()};\n")
                else:
                    file.write(stmt.to_code())

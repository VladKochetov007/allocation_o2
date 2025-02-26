#!/usr/bin/env python3
import argparse
import os
import sys
import subprocess
import shutil
import tempfile
from pathlib import Path

def compile_rust_strategy(source_file, output_path=None):
    """
    Compile a Rust strategy file into a shared library (.so file)
    
    Args:
        source_file (str): Path to the Rust source file
        output_path (str, optional): Path where to save the compiled .so file.
            If None, the .so file will be saved in the same directory as the source file.
    
    Returns:
        bool: True if compilation was successful, False otherwise
    """
    source_path = Path(source_file).resolve()
    
    if not source_path.exists():
        print(f"Error: Source file {source_path} does not exist", file=sys.stderr)
        return False
        
    if not source_path.suffix == '.rs':
        print(f"Error: Source file must be a Rust file (.rs), got {source_path}", file=sys.stderr)
        return False
    
    # Get allocation_o2 package directory
    allocation_o2_dir = Path(__file__).parent.resolve()
    rust_backend_dir = allocation_o2_dir.parent / "rust_backend"
    
    # If rust_backend_dir is not found, try to locate it using installed package metadata
    if not rust_backend_dir.exists():
        # This means we're running from an installed package
        # We need to create a temporary Cargo project that depends on allocation_o2 crate
        temp_dir = None
        try:
            temp_dir = tempfile.mkdtemp(prefix="allocation_o2_compile_")
            temp_project_dir = Path(temp_dir)
            
            # Create a Cargo.toml file that depends on allocation_o2
            with open(temp_project_dir / "Cargo.toml", "w") as f:
                f.write("""
[package]
name = "allocation_o2_strategy"
version = "0.1.0"
edition = "2021"

[lib]
name = "strategy"
crate-type = ["cdylib"]

[dependencies]
allocation_o2 = { git = "https://github.com/VladKochetov007/allocation_o2" }
pyo3 = { version = "0.20.0", features = ["extension-module"] }
ndarray = "0.15.6"
""")
            
            # Create src directory
            os.makedirs(temp_project_dir / "src", exist_ok=True)
            
            # Copy the strategy file to src/lib.rs
            shutil.copy(source_path, temp_project_dir / "src" / "lib.rs")
            
            # Compile the strategy
            result = subprocess.run(
                ["cargo", "build", "--release"],
                cwd=temp_project_dir,
                capture_output=True,
                text=True
            )
            
            if result.returncode != 0:
                print(f"Error compiling strategy: {result.stderr}", file=sys.stderr)
                return False
            
            # Determine output file name and location
            strategy_name = source_path.stem
            if output_path is None:
                output_path = source_path.parent / f"lib{strategy_name}.so"
            else:
                output_path = Path(output_path)
                # If output_path is a directory, use the default file name inside it
                if output_path.exists() and output_path.is_dir():
                    output_path = output_path / f"lib{strategy_name}.so"
                # Otherwise, use the provided path directly, ensuring parent directories exist
                else:
                    os.makedirs(output_path.parent, exist_ok=True)
            
            # Copy the compiled .so file to the output location
            compiled_path = temp_project_dir / "target" / "release" / "libstrategy.so"
            shutil.copy(compiled_path, output_path)
            
            print(f"Successfully compiled strategy to {output_path}")
            return True
            
        finally:
            if temp_dir and os.path.exists(temp_dir):
                shutil.rmtree(temp_dir)
    else:
        # We have access to the rust_backend directory, use it for compilation
        # Determine strategy name from the source file
        strategy_name = source_path.stem
        
        # Create a temp dir to hold the strategy
        temp_dir = None
        try:
            temp_dir = tempfile.mkdtemp(prefix="allocation_o2_compile_")
            strategy_dir = Path(temp_dir)
            
            # Copy the strategy file to the temp directory
            strategy_path = strategy_dir / f"{strategy_name}.rs"
            shutil.copy(source_path, strategy_path)
            
            # Update Cargo.toml to include the new strategy as an example
            cargo_toml_path = rust_backend_dir / "Cargo.toml"
            with open(cargo_toml_path, "r") as f:
                cargo_content = f.read()
            
            # Check if this strategy is already registered
            if f'name = "{strategy_name}"' not in cargo_content:
                # Add the strategy as an example
                example_entry = f"""
[[example]]
name = "{strategy_name}"
path = "{strategy_path}"
crate-type = ["cdylib"]
"""
                with open(cargo_toml_path, "a") as f:
                    f.write(example_entry)
            
            # Compile the strategy
            result = subprocess.run(
                ["cargo", "build", "--release", "--example", strategy_name],
                cwd=rust_backend_dir,
                capture_output=True,
                text=True
            )
            
            if result.returncode != 0:
                print(f"Error compiling strategy: {result.stderr}", file=sys.stderr)
                return False
            
            # Determine output file name and location
            if output_path is None:
                output_path = source_path.parent / f"lib{strategy_name}.so"
            else:
                output_path = Path(output_path)
                # If output_path is a directory, use the default file name inside it
                if output_path.exists() and output_path.is_dir():
                    output_path = output_path / f"lib{strategy_name}.so"
                # Otherwise, use the provided path directly, ensuring parent directories exist
                else:
                    os.makedirs(output_path.parent, exist_ok=True)
            
            # Copy the compiled .so file to the output location
            compiled_path = rust_backend_dir / "target" / "release" / "examples" / f"lib{strategy_name}.so"
            shutil.copy(compiled_path, output_path)
            
            print(f"Successfully compiled strategy to {output_path}")
            return True
            
        finally:
            if temp_dir and os.path.exists(temp_dir):
                shutil.rmtree(temp_dir)
            
            # Remove the temporary entry from Cargo.toml if it was added
            if rust_backend_dir.exists():
                with open(cargo_toml_path, "r") as f:
                    lines = f.readlines()
                
                filtered_lines = []
                skip_section = False
                for line in lines:
                    if f'name = "{strategy_name}"' in line and '[[example]]' in ''.join(filtered_lines[-2:]):
                        skip_section = True
                        # Remove the [[example]] line too
                        filtered_lines.pop()
                        continue
                    
                    if skip_section and 'crate-type = ["cdylib"]' in line:
                        skip_section = False
                        continue
                        
                    if not skip_section:
                        filtered_lines.append(line)
                
                with open(cargo_toml_path, "w") as f:
                    f.writelines(filtered_lines)

def main():
    parser = argparse.ArgumentParser(
        description="Allocation O2 - Tactical asset allocation library with Rust backend"
    )
    
    subparsers = parser.add_subparsers(dest="command", help="Command to execute")
    
    # Compile command
    compile_parser = subparsers.add_parser("compile", help="Compile a Rust strategy file")
    compile_parser.add_argument("source_file", help="Path to the Rust strategy file")
    compile_parser.add_argument(
        "-o", "--output", 
        help="Path where to save the compiled .so file. If not specified, "
             "the .so file will be saved in the same directory as the source file."
    )
    
    args = parser.parse_args()
    
    if args.command == "compile":
        success = compile_rust_strategy(args.source_file, args.output)
        sys.exit(0 if success else 1)
    else:
        # No command specified, show help
        parser.print_help()
        sys.exit(1)

if __name__ == "__main__":
    main() 
import os
import shutil
import subprocess

def run_cargo_build(features=None):
    command = ['cargo', 'build', '--release']
    if features:
        command.extend(['--features', features])

    result = subprocess.run(command)
    if result.returncode != 0:
        print("Cargo build failed")
        exit(1)

def copy_files_to_out():
    os.makedirs('out', exist_ok=True)

    shutil.copy('./target/release/SimConnect.dll', 'out/')

    shutil.copy('./target/release/e-jet-systems.exe', 'out/')

def main():
    include_gui = input("Do you want to include GUI features? (y/n): ").lower()
    features = 'gui' if include_gui == 'y' else None

    run_cargo_build(features)

    copy_files_to_out()

if __name__ == "__main__":
    main()
import os
import toml
import sys

def convert_toml_to_txt(directory):
    for filename in os.listdir(directory):
        if filename.endswith('.toml'):
            toml_path = os.path.join(directory, filename)
            txt_path = os.path.join(directory, filename.replace('.toml', '.txt'))

            # Load TOML
            with open(toml_path, 'r', encoding='utf-8') as file:
                data = toml.load(file)

            # Extract strings
            string_list = data.get('strings', [])

            # Write to TXT
            with open(txt_path, 'w', encoding='utf-8') as file:
                for entry in string_list:
                    file.write(entry + '\n')

            print(f"Converted: {filename} → {os.path.basename(txt_path)}")

if not sys.argv[1]:
    print("missing required argument")
    exit(1)

convert_toml_to_txt(sys.argv[1])


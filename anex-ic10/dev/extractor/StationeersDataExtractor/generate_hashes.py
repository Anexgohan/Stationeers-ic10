import xml.etree.ElementTree as ET
import zlib
import shutil
import os

# --- SCRIPT START ---

# The dictionary to hold your data
d = {}
input_filename = './input/english.xml'
output_filename = './output/stationpedia.txt'
default_game_path = r"D:\Games\SteamLibrary\SteamApps\common\Stationeers\rocketstation_Data\StreamingAssets\Language\english.xml"

# ANSI color codes with Windows compatibility
import sys
import os

# Enable colors on Windows 10+ and all other platforms
try:
    if sys.platform == "win32":
        # Try to enable ANSI color support on Windows
        os.system('color')
    CYAN = '\033[96m'
    YELLOW = '\033[93m'
    GREEN = '\033[92m'
    RESET = '\033[0m'
except:
    # Fallback to no colors if there's any issue
    CYAN = YELLOW = GREEN = RESET = ""

# Ask user if they want to copy the latest english.xml from game files
print(f"\n{CYAN}═══════════════════════════════════════════════════════════════════════{RESET}")
copy_choice = input(f"{CYAN}🔄 Do you want to copy the latest {YELLOW}english.xml{CYAN} from game files? {YELLOW}[Default: No]{RESET} {CYAN}(y/N):{RESET} ").strip()

if copy_choice.lower() in ['y', 'yes']:
    # Ask for path or use default
    print(f"\n{CYAN}📁 Enter path to {YELLOW}english.xml{CYAN} (press Enter for default):{RESET}")
    print(f"{CYAN}Default: {YELLOW}{default_game_path}{RESET}")
    game_path_input = input(f"{CYAN}Path:{RESET} ").strip()
    game_path = game_path_input if game_path_input else default_game_path
    
    # Check if source file exists
    if os.path.exists(game_path):
        try:
            # Ensure input directory exists
            os.makedirs('./input', exist_ok=True)
            # Copy the file
            shutil.copy2(game_path, input_filename)
            print(f"✅ Successfully copied {game_path} to {input_filename}")
        except Exception as e:
            print(f"Error copying file: {e}")
            exit()
    else:
        print(f"Error: Game file not found at '{game_path}'")
        exit()

# 1. Load and parse your provided XML file
try:
    tree = ET.parse(input_filename)
    x = tree.getroot()
except FileNotFoundError:
    print(f"Error: '{input_filename}' not found. Make sure it's in the input directory or copy it from game files.")
    exit()

# 2. This is YOUR code snippet. It reads the XML and populates the dictionary.
for t in x.findall('./Things/RecordThing'):
    d[t.findtext('./Key')] = t.findtext('./Value', '')

# 3. This part now calculates both hash formats and writes them to the file.
with open(output_filename, 'w', encoding='utf-8') as file_out:
    # Iterate through the structure names (keys) in alphabetical order
    for prefab_name in sorted(d.keys()):
        # Calculate the standard unsigned hash
        unsigned_hash = zlib.crc32(prefab_name.encode('utf-8'))

        # Convert the unsigned hash to the signed decimal format
        signed_hash = (unsigned_hash ^ 0x80000000) - 0x80000000
        
        # Convert the unsigned hash to a hexadecimal string
        hex_hash = hex(unsigned_hash)

        # Get the display name from the dictionary
        display_name = d[prefab_name]

        # Write the new, extended line to the file
        file_out.write(f'"{prefab_name}" {signed_hash} {hex_hash} "{display_name}"\n')

print(f"{GREEN}✅ Success! The file was written to:{RESET}\n{YELLOW}{os.path.abspath(output_filename)}{RESET}")
import argparse as ap 

# Function to take value and convert it to Little Endian byte order
def little_endian(bytes):
    lo = bytes & 0xFF # Mask out highest byte to get lo byte
    hi = bytes >> 8 # shift byte 8 places right 
    return lo << 8 | hi # Shift lo left 8 places to make room for hi byte, then OR lo with hi byte to get lo,hi order
    





if __name__ == "__main__":
    # Argument parser setup
    parser = ap.ArgumentParser(description="Convert byte to Little Endian format")
    parser = ap.ArgumentParser(fromfile_prefix_chars='@') # Used for file input. Ex. python byteswap.py @list.txt
    parser.add_argument("bytes", type=str, nargs='+')
    args = parser.parse_args()

    # Try to convert
    try:
        for i, v in enumerate(args.bytes): # Iterate through list. i is index, v is value
            v = int(v, 16) # Convert str to int (args are received as strings)
            result = little_endian(v) # Pass int value to function to byteswap
            print(f"Little Endian Result {i + 1}: 0x{result:04X}") # Print result of byteswap in hex with padding
    except ValueError: # Error out if any conversion fails
        print("Error! Invalid byte entry, check values and try again.")










#!/usr/bin/env python3
"""
Test script to decode a simple string hex value.
"""

import os
import sys
import binascii
from pathlib import Path

# Add the project root to Python path for development
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root / "python"))

try:
    import alwayson
except ImportError as e:
    print(f"[ERROR] Failed to import alwayson: {e}")
    print("Make sure to run 'uv run maturin develop' first")
    sys.exit(1)


def main():
    """Test with simple string hex."""
    print("Testing Simple String Hex Decoding")
    print("=" * 40)
    
    # Read the simple string hex
    script_dir = Path(__file__).parent
    hex_file = script_dir / "simple_string.txt"
    
    with open(hex_file, 'r') as f:
        hex_string = f.read().strip()
    
    print(f"Hex string: {hex_string}")
    
    # Convert to binary
    binary_data = binascii.unhexlify(hex_string)
    print(f"Binary length: {len(binary_data)} bytes")
    print(f"Binary data: {binary_data}")
    
    # Decode as TwPrim
    try:
        prim = alwayson.TwPrim.from_bytes(binary_data)
        print(f"[OK] Decoded as TwPrim!")
        print(f"   Type: {prim.get_type()}")
        print(f"   Value: {prim.get_value()}")
        print(f"   JSON: {prim.to_json()}")
        
        # Test round-trip
        re_encoded = prim.to_bytes()
        print(f"   Round-trip check: {binary_data == re_encoded}")
        
    except Exception as e:
        print(f"[FAIL] TwPrim decode failed: {e}")


if __name__ == "__main__":
    main()
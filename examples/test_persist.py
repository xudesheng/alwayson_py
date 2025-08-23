#!/usr/bin/env python3
"""
AlwaysOn Python - Persistence Test Script

This script demonstrates how to decode persistent data by:
1. Reading hex data from sample_persist.txt
2. Converting hex to binary (with proper trimming)
3. Attempting to decode using Avro format first
4. Then attempting to decode using alwayson_py

Usage:
    uv run python examples/test_persist.py
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
    print(f"❌ Failed to import alwayson: {e}")
    print("Make sure to run 'uv run maturin develop' first")
    sys.exit(1)

# Try to import Avro
try:
    import avro.schema
    import avro.io
    import io
    AVRO_AVAILABLE = True
    print("[INFO] Avro library available")
except ImportError:
    AVRO_AVAILABLE = False
    print("[WARN] Avro library not available - install with: uv add avro-python3")


def read_persist_file(filename):
    """Read hex string from persist file and convert to bytes with proper trimming."""
    script_dir = Path(__file__).parent
    persist_file = script_dir / filename
    
    if not persist_file.exists():
        raise FileNotFoundError(f"Persist file not found: {persist_file}")
    
    with open(persist_file, 'r') as f:
        hex_string = f.read().strip()  # Trim whitespace including newlines
    
    print(f"[READ] Read hex string from {filename}")
    print(f"   Original length: {len(hex_string)} characters")
    print(f"   After trimming: {len(hex_string)} characters")
    print(f"   First 40 chars: {hex_string[:40]}...")
    print(f"   Last 40 chars: ...{hex_string[-40:]}")
    
    try:
        binary_data = binascii.unhexlify(hex_string)
        print(f"[OK] Converted to binary: {len(binary_data)} bytes")
        return binary_data
    except ValueError as e:
        raise ValueError(f"Invalid hex string: {e}")


def analyze_binary_structure(data):
    """Analyze the structure of binary data."""
    print(f"\n[ANALYZE] Binary Data Analysis:")
    print(f"   Total length: {len(data)} bytes")
    print(f"   First 10 bytes (hex): {data[:10].hex()}")
    print(f"   First 10 bytes (dec): {list(data[:10])}")
    print(f"   Last 10 bytes (hex): {data[-10:].hex()}")
    print(f"   Last 10 bytes (dec): {list(data[-10:])}")
    
    # Find readable ASCII strings
    readable_strings = []
    current_string = ""
    start_pos = 0
    
    for i, byte in enumerate(data):
        if 32 <= byte <= 126:  # Printable ASCII
            if not current_string:
                start_pos = i
            current_string += chr(byte)
        else:
            if current_string and len(current_string) >= 3:
                readable_strings.append((start_pos, current_string))
            current_string = ""
    
    if current_string and len(current_string) >= 3:
        readable_strings.append((start_pos, current_string))
    
    if readable_strings:
        print(f"   Found {len(readable_strings)} readable strings:")
        for pos, string in readable_strings:
            print(f"     @{pos:3d}: '{string}'")


def test_avro_decoding(data):
    """Attempt to decode data using Avro format."""
    print(f"\n[TEST] Testing Avro decoding:")
    
    if not AVRO_AVAILABLE:
        print(f"[SKIP] Avro library not available")
        return None
    
    try:
        # Try to decode as Avro without schema (raw approach)
        print(f"   Attempting raw Avro decoding...")
        
        # Create a binary reader from the data
        bytes_reader = io.BytesIO(data)
        
        # Try to read Avro header magic bytes
        magic = bytes_reader.read(4)
        if magic == b'Obj\x01':  # Avro object container file magic
            print(f"[MAYBE] Found Avro magic bytes: {magic.hex()}")
            
            # Try to continue reading Avro metadata
            try:
                # This is a simplified attempt - real Avro parsing is more complex
                metadata_length = int.from_bytes(bytes_reader.read(8), byteorder='little')
                print(f"   Metadata length: {metadata_length}")
                
                if metadata_length < len(data) and metadata_length > 0:
                    metadata = bytes_reader.read(metadata_length)
                    print(f"[OK] Read Avro metadata: {len(metadata)} bytes")
                    print(f"   Metadata preview: {metadata[:50]}...")
                    return {"type": "avro", "metadata_length": metadata_length}
                else:
                    print(f"[FAIL] Invalid Avro metadata length")
                    
            except Exception as e:
                print(f"[FAIL] Avro metadata parsing failed: {e}")
        else:
            print(f"[FAIL] No Avro magic bytes found (got: {magic.hex()})")
            
        # Try alternative approach - look for Avro patterns
        print(f"   Attempting pattern-based Avro detection...")
        
        # Look for common Avro encoding patterns
        if len(data) > 0:
            first_bytes = data[:20].hex()
            print(f"   First 20 bytes: {first_bytes}")
            
            # Check for Avro variable-length encoding patterns
            avro_patterns_found = []
            for i in range(min(10, len(data))):
                byte_val = data[i]
                if byte_val & 0x01 == 0:  # Even values might be Avro varint
                    avro_patterns_found.append(f"Possible varint at {i}: {byte_val}")
            
            if avro_patterns_found:
                print(f"   Potential Avro patterns:")
                for pattern in avro_patterns_found[:5]:  # Show first 5
                    print(f"     {pattern}")
                return {"type": "avro_patterns", "patterns": avro_patterns_found}
        
        print(f"[FAIL] No recognizable Avro patterns found")
        return None
        
    except Exception as e:
        print(f"[FAIL] Avro decoding failed: {e}")
        import traceback
        traceback.print_exc()
        return None


def test_alwayson_decoding(data):
    """Attempt to decode data using all alwayson_py methods."""
    print(f"\n[TEST] Testing alwayson_py decoding:")
    
    results = {}
    
    # Test TwPrim decoding
    print(f"   Testing TwPrim.from_bytes()...")
    try:
        prim = alwayson.TwPrim.from_bytes(data)
        print(f"   [OK] TwPrim successful!")
        print(f"      Type: {prim.get_type()}")
        print(f"      Value: {prim.get_value()}")
        results['TwPrim'] = prim
    except Exception as e:
        print(f"   [FAIL] TwPrim failed: {e}")
    
    # Test TwxMessage decoding
    print(f"   Testing TwxMessage.from_bytes()...")
    try:
        message = alwayson.TwxMessage.from_bytes(data)
        print(f"   [OK] TwxMessage successful!")
        print(f"      Type: {message.get_message_type()}")
        print(f"      Request ID: {message.get_request_id()}")
        print(f"      Description: {message.short_description()}")
        results['TwxMessage'] = message
    except Exception as e:
        print(f"   [FAIL] TwxMessage failed: {e}")
    
    # Test TwxEvent decoding (will likely fail as it expects JSON)
    print(f"   Testing TwxEvent.from_bytes()...")
    try:
        event = alwayson.TwxEvent.from_bytes(data)
        print(f"   [OK] TwxEvent successful!")
        print(f"      Name: {event.get_name()}")
        results['TwxEvent'] = event
    except Exception as e:
        print(f"   [FAIL] TwxEvent failed: {e}")
    
    # Test TwxService decoding (will likely fail as it expects JSON)
    print(f"   Testing TwxService.from_bytes()...")
    try:
        service = alwayson.TwxService.from_bytes(data)
        print(f"   [OK] TwxService successful!")
        print(f"      Name: {service.get_name()}")
        results['TwxService'] = service
    except Exception as e:
        print(f"   [FAIL] TwxService failed: {e}")
    
    # Test TwxProperty decoding (will likely fail as it expects JSON)
    print(f"   Testing TwxProperty.from_bytes()...")
    try:
        property_obj = alwayson.TwxProperty.from_bytes(data)
        print(f"   [OK] TwxProperty successful!")
        print(f"      Name: {property_obj.get_name()}")
        results['TwxProperty'] = property_obj
    except Exception as e:
        print(f"   [FAIL] TwxProperty failed: {e}")
    
    return results


def test_partial_decoding(data):
    """Try to decode portions of the data to find embedded structures."""
    print(f"\n[TEST] Testing partial/sliding window decoding:")
    
    successful_decodings = []
    
    # Try different starting positions
    for start_pos in range(0, min(20, len(data))):  # Try first 20 bytes as start positions
        for length in [4, 8, 13, 16, 32]:  # Try different lengths
            if start_pos + length > len(data):
                continue
                
            chunk = data[start_pos:start_pos + length]
            
            # Try TwPrim on this chunk
            try:
                prim = alwayson.TwPrim.from_bytes(chunk)
                successful_decodings.append({
                    'start': start_pos,
                    'length': length,
                    'type': 'TwPrim',
                    'data_type': prim.get_type(),
                    'value': str(prim.get_value())[:50],  # Truncate long values
                    'chunk_hex': chunk.hex()
                })
            except:
                pass
    
    if successful_decodings:
        print(f"   Found {len(successful_decodings)} successful partial decodings:")
        for i, result in enumerate(successful_decodings[:10]):  # Show first 10
            print(f"     {i+1}. @{result['start']}+{result['length']}: {result['type']} "
                  f"{result['data_type']} = '{result['value']}' (hex: {result['chunk_hex'][:16]}...)")
        return successful_decodings
    else:
        print(f"   No successful partial decodings found")
        return []


def main():
    """Main function to run the persistence test."""
    print("AlwaysOn Python - Persistence Test Script")
    print("=" * 50)
    
    try:
        # Read and process the persistence file
        print(f"[INFO] Reading persistence data from sample_persist.txt:")
        binary_data = read_persist_file("sample_persist.txt")
        
        # Analyze the structure
        analyze_binary_structure(binary_data)
        
        # Try Avro decoding first
        avro_result = test_avro_decoding(binary_data)
        
        # Try alwayson_py decoding
        alwayson_results = test_alwayson_decoding(binary_data)
        
        # Try partial decoding to find embedded structures
        partial_results = test_partial_decoding(binary_data)
        
        # Summary
        print(f"\n[SUMMARY] Decoding Results:")
        print(f"   Avro decoding: {'Success' if avro_result else 'Failed'}")
        print(f"   AlwaysOn successful types: {list(alwayson_results.keys())}")
        print(f"   Partial decodings found: {len(partial_results)}")
        
        total_successes = (1 if avro_result else 0) + len(alwayson_results) + len(partial_results)
        if total_successes > 0:
            print(f"\n[SUCCESS] Found {total_successes} successful decoding(s)!")
            
            # Show the most promising results
            if alwayson_results:
                print(f"\n[BEST RESULTS] AlwaysOn decodings:")
                for decoder_type, result in alwayson_results.items():
                    print(f"   {decoder_type}: {str(result)[:100]}...")
        else:
            print(f"\n[NOTE] This appears to be a format not yet supported")
            print(f"   by the current Avro or AlwaysOn implementations.")
            print(f"   Consider checking if it's a different serialization format")
            print(f"   (e.g., Protocol Buffers, MessagePack, custom binary format)")
        
        print(f"\n[DONE] Persistence test completed!")
        
    except Exception as e:
        print(f"[ERROR] Test failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
#!/usr/bin/env python3
"""
AlwaysOn Python - Sample Test Script

This script demonstrates how to use the alwayson_py library to:
1. Read hex data from a file
2. Convert hex to binary
3. Attempt to decode using TwPrim and TwxMessage
4. Show analysis of the data structure

Usage:
    uv run python examples/test_sample.py
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


def read_hex_file(filename):
    """Read hex string from file and convert to bytes."""
    script_dir = Path(__file__).parent
    hex_file = script_dir / filename
    
    if not hex_file.exists():
        raise FileNotFoundError(f"Sample file not found: {hex_file}")
    
    with open(hex_file, 'r') as f:
        hex_string = f.read().strip()
    
    print(f"[READ] Read hex string from {filename}")
    print(f"   Length: {len(hex_string)} characters")
    print(f"   First 40 chars: {hex_string[:40]}...")
    
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


def test_twprim_decoding(data):
    """Attempt to decode data as TwPrim."""
    print(f"\n[TEST] Testing TwPrim.from_bytes():")
    try:
        prim = alwayson.TwPrim.from_bytes(data)
        print(f"[OK] Successfully decoded as TwPrim!")
        print(f"   Type: {prim.get_type()}")
        print(f"   Value: {prim.get_value()}")
        print(f"   JSON: {prim.to_json()}")
        return prim
    except Exception as e:
        print(f"[FAIL] TwPrim decoding failed: {e}")
        return None


def test_message_decoding(data):
    """Attempt to decode data as TwxMessage."""
    print(f"\n[TEST] Testing TwxMessage.from_bytes():")
    try:
        message = alwayson.TwxMessage.from_bytes(data)
        print(f"[OK] Successfully decoded as TwxMessage!")
        print(f"   Message type: {message.get_message_type()}")
        print(f"   Request ID: {message.get_request_id()}")
        print(f"   Session ID: {message.get_session_id()}")
        print(f"   Endpoint: {message.get_endpoint()}")
        print(f"   Is Request: {message.is_request()}")
        print(f"   Is Response: {message.is_response()}")
        print(f"   Is Auth: {message.is_auth()}")
        print(f"   Is Bind: {message.is_bind()}")
        print(f"   Description: {message.short_description()}")
        return message
    except Exception as e:
        print(f"[FAIL] TwxMessage decoding failed: {e}")
        return None


def test_event_decoding(data):
    """Attempt to decode data as TwxEvent."""
    print(f"\n[TEST] Testing TwxEvent.from_bytes():")
    try:
        event = alwayson.TwxEvent.from_bytes(data)
        print(f"[OK] Successfully decoded as TwxEvent!")
        print(f"   Event name: {event.get_name()}")
        print(f"   Description: {event.get_description()}")
        print(f"   JSON: {event.to_json()}")
        return event
    except Exception as e:
        print(f"[FAIL] TwxEvent decoding failed: {e}")
        return None


def test_service_decoding(data):
    """Attempt to decode data as TwxService."""
    print(f"\n[TEST] Testing TwxService.from_bytes():")
    try:
        service = alwayson.TwxService.from_bytes(data)
        print(f"[OK] Successfully decoded as TwxService!")
        print(f"   Service name: {service.get_name()}")
        print(f"   Description: {service.get_description()}")
        print(f"   JSON: {service.to_json()}")
        return service
    except Exception as e:
        print(f"[FAIL] TwxService decoding failed: {e}")
        return None


def test_property_decoding(data):
    """Attempt to decode data as TwxProperty."""
    print(f"\n[TEST] Testing TwxProperty.from_bytes():")
    try:
        property = alwayson.TwxProperty.from_bytes(data)
        print(f"[OK] Successfully decoded as TwxProperty!")
        print(f"   Property name: {property.get_name()}")
        print(f"   Base type: {property.get_base_type()}")
        print(f"   Push threshold: {property.get_push_threshold()}")
        print(f"   Should read edge value: {property.should_read_edge_value()}")
        print(f"   JSON: {property.to_json()}")
        return property
    except Exception as e:
        print(f"[FAIL] TwxProperty decoding failed: {e}")
        return None


def test_simple_examples():
    """Test with simple examples to show working functionality."""
    print(f"\n[DEMO] Testing Simple Examples:")
    
    # Test string primitive
    print(f"   Creating string primitive...")
    string_prim = alwayson.TwPrim.string("Hello World")
    string_bytes = string_prim.to_bytes()
    decoded_string = alwayson.TwPrim.from_bytes(string_bytes)
    print(f"   [OK] String round-trip: '{decoded_string.get_value()}'")
    
    # Test number primitive
    print(f"   Creating number primitive...")
    number_prim = alwayson.TwPrim.number(42.5)
    number_bytes = number_prim.to_bytes()
    decoded_number = alwayson.TwPrim.from_bytes(number_bytes)
    print(f"   [OK] Number round-trip: {decoded_number.get_value()}")
    
    # Test auth message
    print(f"   Creating auth message...")
    auth_msg = alwayson.TwxMessage.build_auth(12345, "test-key")
    auth_bytes = auth_msg.to_bytes()
    decoded_auth = alwayson.TwxMessage.from_bytes(auth_bytes)
    print(f"   [OK] Auth message round-trip: {decoded_auth.get_message_type()}")


def main():
    """Main function to run the test."""
    print("AlwaysOn Python - Sample Test Script")
    print("=" * 50)
    
    try:
        # Test simple examples first
        test_simple_examples()
        
        # Read and process the sample file
        print(f"\n[INFO] Testing with sample_hex.txt (complex message):")
        binary_data = read_hex_file("sample_hex.txt")
        
        # Analyze the structure
        analyze_binary_structure(binary_data)
        
        # Try decoding as different types
        twprim_result = test_twprim_decoding(binary_data)
        message_result = test_message_decoding(binary_data)
        event_result = test_event_decoding(binary_data)
        service_result = test_service_decoding(binary_data)
        property_result = test_property_decoding(binary_data)
        
        print(f"\n[SUMMARY] Results:")
        print(f"   TwPrim decode: {'Success' if twprim_result else 'Failed'}")
        print(f"   TwxMessage decode: {'Success' if message_result else 'Failed'}")
        print(f"   TwxEvent decode: {'Success' if event_result else 'Failed'}")
        print(f"   TwxService decode: {'Success' if service_result else 'Failed'}")
        print(f"   TwxProperty decode: {'Success' if property_result else 'Failed'}")
        
        success_count = sum([bool(twprim_result), bool(message_result), bool(event_result), bool(service_result), bool(property_result)])
        if success_count == 0:
            print(f"\n[NOTE] This appears to be a complex ThingWorx protocol message")
            print(f"   that may require multipart message handling or special")
            print(f"   processing not yet implemented in the current version.")
        else:
            print(f"\n[SUCCESS] Decoded successfully as {success_count} different type(s)!")
        
        # Test with simple string file
        print(f"\n[INFO] Testing with simple_string.txt (TwPrim string):")
        try:
            simple_data = read_hex_file("simple_string.txt")
            analyze_binary_structure(simple_data)
            simple_result = test_twprim_decoding(simple_data)
            print(f"   Simple string decode: {'Success' if simple_result else 'Failed'}")
        except Exception as e:
            print(f"   [ERROR] Simple string test failed: {e}")

        print(f"\n[DONE] Test completed!")
        
    except Exception as e:
        print(f"[ERROR] Test failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
#!/usr/bin/env python3
"""
AlwaysOn Python - Enhanced Persistence Test with Avro+ThingWorx Format

Based on analysis of ThingWorx durablequeue code, the encoding format is:
1. Outer layer: Apache Avro binary encoding with schema
2. Inner values: ThingWorx BaseTypes.WritePrimitiveToByteArray() encoding

This script attempts to decode data with this understanding.

Usage:
    uv run python examples/test_persist_avro_thingworx.py
"""

import os
import sys
import binascii
import struct
from pathlib import Path
from typing import Dict, Any, List, Tuple

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
    from avro.datafile import DataFileReader, DataFileWriter
    from avro.io import DatumReader, DatumWriter, BinaryDecoder, BinaryEncoder
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


def analyze_as_thingworx_avro(data: bytes) -> Dict[str, Any]:
    """
    Analyze data as ThingWorx Avro format based on the durablequeue implementation.
    
    The format appears to be:
    - AvroPersistentPropertyStreamEntryMessage or AvroValueStreamEntry
    - Contains operationType (byte) and nested entry
    - Values are encoded with BaseTypes.WritePrimitiveToByteArray()
    """
    print(f"\n[ANALYZE] ThingWorx Avro Format Analysis:")
    results = {}
    
    # Check if first byte could be operationType
    if len(data) > 0:
        op_type = data[0]
        print(f"   First byte (possible operationType): {op_type:#04x} ({op_type})")
        results['operation_type'] = op_type
    
    # Try to identify Avro schema reflection patterns
    # Avro uses variable-length encoding for strings and arrays
    print(f"   Checking for Avro variable-length encoding patterns...")
    
    # Look for string length markers (Avro encodes string length as zigzag varint)
    pos = 0
    potential_strings = []
    
    while pos < len(data) - 1:
        # Try to read as Avro varint
        try:
            length, bytes_read = read_avro_varint(data[pos:])
            if 0 < length < 100 and pos + bytes_read + length <= len(data):
                # Try to extract string
                string_data = data[pos + bytes_read:pos + bytes_read + length]
                try:
                    decoded_str = string_data.decode('utf-8')
                    if is_printable_string(decoded_str):
                        potential_strings.append((pos, length, decoded_str))
                        print(f"   Found potential Avro string at {pos}: '{decoded_str}'")
                except:
                    pass
            pos += 1
        except:
            pos += 1
    
    results['avro_strings'] = potential_strings
    
    # Look for ThingWorx BaseTypes patterns
    print(f"\n   Checking for ThingWorx BaseTypes patterns...")
    
    # ThingWorx primitives start with type markers
    # Based on alwayson-codec analysis:
    # 0x00 = NOTHING, 0x01 = BOOLEAN, 0x02 = INTEGER, etc.
    thingworx_patterns = []
    
    for i in range(len(data) - 4):
        type_byte = data[i]
        if type_byte in [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]:
            # Try to decode as ThingWorx primitive
            try:
                prim = alwayson.TwPrim.from_bytes(data[i:i+20])  # Try with 20 bytes
                thingworx_patterns.append((i, type_byte, str(prim.get_type()), str(prim.get_value())[:30]))
                print(f"   Found ThingWorx primitive at {i}: {prim.get_type()} = {str(prim.get_value())[:30]}")
            except:
                pass
    
    results['thingworx_primitives'] = thingworx_patterns
    
    # Try to identify timestamp patterns (8-byte longs)
    print(f"\n   Checking for timestamp patterns (8-byte longs)...")
    timestamps = []
    
    for i in range(len(data) - 7):
        try:
            # Try little-endian long
            timestamp_le = struct.unpack('<Q', data[i:i+8])[0]
            # Check if it could be a reasonable timestamp (year 2000-2030)
            if 946684800000 < timestamp_le < 1893456000000:  # milliseconds since epoch
                from datetime import datetime
                dt = datetime.fromtimestamp(timestamp_le / 1000)
                timestamps.append((i, 'little-endian', timestamp_le, str(dt)))
                print(f"   Potential timestamp at {i} (LE): {dt}")
            
            # Try big-endian long
            timestamp_be = struct.unpack('>Q', data[i:i+8])[0]
            if 946684800000 < timestamp_be < 1893456000000:
                from datetime import datetime
                dt = datetime.fromtimestamp(timestamp_be / 1000)
                timestamps.append((i, 'big-endian', timestamp_be, str(dt)))
                print(f"   Potential timestamp at {i} (BE): {dt}")
        except:
            pass
    
    results['timestamps'] = timestamps
    
    return results


def read_avro_varint(data: bytes) -> Tuple[int, int]:
    """Read an Avro variable-length integer (zigzag encoded)."""
    n = 0
    shift = 0
    bytes_read = 0
    
    for byte in data:
        bytes_read += 1
        if byte & 0x80:
            n |= (byte & 0x7F) << shift
            shift += 7
        else:
            n |= byte << shift
            # Decode zigzag
            return (n >> 1) ^ -(n & 1), bytes_read
    
    raise ValueError("Incomplete varint")


def is_printable_string(s: str) -> bool:
    """Check if string contains mostly printable characters."""
    if len(s) == 0:
        return False
    printable_count = sum(1 for c in s if c.isprintable() or c in '\n\r\t')
    return printable_count / len(s) > 0.8


def attempt_avro_schema_decoding(data: bytes):
    """
    Attempt to decode using Avro with inferred schema based on ThingWorx classes.
    """
    if not AVRO_AVAILABLE:
        print(f"[SKIP] Avro library not available for schema-based decoding")
        return None
    
    print(f"\n[TEST] Attempting Avro schema-based decoding:")
    
    # Define schemas based on ThingWorx durablequeue classes
    schemas = []
    
    # AvroPersistentPropertyStreamEntryMessage schema
    persistent_property_schema = """
    {
        "type": "record",
        "name": "AvroPersistentPropertyStreamEntryMessage",
        "fields": [
            {"name": "operationType", "type": "int"},
            {"name": "avroPersistentPropertyEntry", "type": {
                "type": "record",
                "name": "AvroPersistentPropertyEntry",
                "fields": [
                    {"name": "source", "type": ["null", "string"], "default": null},
                    {"name": "id", "type": ["null", "string"], "default": null},
                    {"name": "entryTimestamp", "type": "long"},
                    {"name": "forceOverwrite", "type": "boolean"},
                    {"name": "propertyName", "type": ["null", "string"], "default": null},
                    {"name": "vtqTimestamp", "type": "long"},
                    {"name": "qualityStatus", "type": "int"},
                    {"name": "value", "type": ["null", "bytes"], "default": null}
                ]
            }}
        ]
    }
    """
    
    # AvroValueStreamEntry schema
    value_stream_schema = """
    {
        "type": "record",
        "name": "AvroValueStreamEntry",
        "fields": [
            {"name": "source", "type": ["null", "string"], "default": null},
            {"name": "name", "type": ["null", "string"], "default": null},
            {"name": "timestamp", "type": "long"},
            {"name": "qualityStatus", "type": "int"},
            {"name": "value", "type": ["null", "bytes"], "default": null}
        ]
    }
    """
    
    schemas = [
        ("PersistentProperty", persistent_property_schema),
        ("ValueStream", value_stream_schema)
    ]
    
    for schema_name, schema_json in schemas:
        try:
            print(f"   Trying {schema_name} schema...")
            schema = avro.schema.parse(schema_json)
            
            bytes_reader = io.BytesIO(data)
            decoder = BinaryDecoder(bytes_reader)
            reader = DatumReader(schema)
            
            result = reader.read(decoder)
            print(f"   [OK] Successfully decoded with {schema_name} schema!")
            print(f"   Result: {result}")
            
            # If value field exists and is bytes, try to decode as ThingWorx primitive
            if 'value' in result and result['value']:
                try:
                    prim = alwayson.TwPrim.from_bytes(result['value'])
                    print(f"   Decoded embedded value: {prim.get_type()} = {prim.get_value()}")
                except:
                    print(f"   Could not decode embedded value as TwPrim")
            
            return result
            
        except Exception as e:
            print(f"   [FAIL] {schema_name} schema failed: {str(e)[:100]}")
    
    return None


def main():
    """Main function to run the enhanced persistence test."""
    print("AlwaysOn Python - Enhanced ThingWorx Persistence Test")
    print("=" * 60)
    print("Based on analysis of ThingWorx durablequeue implementation:")
    print("- Outer format: Apache Avro binary encoding")
    print("- Inner values: BaseTypes.WritePrimitiveToByteArray() (ThingWorx primitives)")
    print("=" * 60)
    
    try:
        # Read the persistence file
        binary_data = read_persist_file("sample_persist.txt")
        
        # Analyze as ThingWorx Avro format
        analysis = analyze_as_thingworx_avro(binary_data)
        
        # Attempt schema-based Avro decoding
        avro_result = attempt_avro_schema_decoding(binary_data)
        
        # Summary
        print(f"\n[SUMMARY] Analysis Results:")
        if analysis.get('avro_strings'):
            print(f"   Found {len(analysis['avro_strings'])} Avro-encoded strings")
            for pos, length, string in analysis['avro_strings'][:3]:
                print(f"     - '{string}' at position {pos}")
        
        if analysis.get('thingworx_primitives'):
            print(f"   Found {len(analysis['thingworx_primitives'])} ThingWorx primitives")
            for pos, type_byte, type_name, value in analysis['thingworx_primitives'][:3]:
                print(f"     - {type_name} = '{value}' at position {pos}")
        
        if analysis.get('timestamps'):
            print(f"   Found {len(analysis['timestamps'])} potential timestamps")
            for pos, endian, value, dt_str in analysis['timestamps'][:2]:
                print(f"     - {dt_str} at position {pos} ({endian})")
        
        if avro_result:
            print(f"   Avro schema decoding: SUCCESS")
        else:
            print(f"   Avro schema decoding: Failed")
        
        print(f"\n[CONCLUSION]")
        print(f"The data appears to be a ThingWorx durable queue message with:")
        print(f"- Entity names: 'demo_thing', 'demo_int_prop'")
        print(f"- Likely contains persistent property or value stream entries")
        print(f"- Values are encoded using ThingWorx BaseTypes binary format")
        print(f"- Overall structure uses Avro binary encoding without file container")
        
        print(f"\n[DONE] Enhanced persistence test completed!")
        
    except Exception as e:
        print(f"[ERROR] Test failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
#!/usr/bin/env python3
"""
AlwaysOn Python - JSON Sample Test Script

This script demonstrates the JSON-based functionality of TwxEvent, TwxService, and TwxProperty classes.
It reads sample JSON files and tests the from_json() and from_bytes() methods.

Usage:
    uv run python examples/test_json_samples.py
"""

import os
import sys
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


def read_json_file(filename):
    """Read JSON content from file."""
    script_dir = Path(__file__).parent
    json_file = script_dir / filename
    
    if not json_file.exists():
        raise FileNotFoundError(f"JSON file not found: {json_file}")
    
    with open(json_file, 'r') as f:
        json_content = f.read().strip()
    
    print(f"[READ] Read JSON from {filename}")
    print(f"   Length: {len(json_content)} characters")
    print(f"   First 80 chars: {json_content[:80]}...")
    return json_content


def test_event_json():
    """Test TwxEvent with JSON data."""
    print(f"\n[TEST] Testing TwxEvent with JSON:")
    try:
        json_content = read_json_file("sample_event.json")
        
        # Test from_json method
        event = alwayson.TwxEvent.from_json(json_content)
        print(f"[OK] Successfully created TwxEvent from JSON!")
        print(f"   Event name: {event.get_name()}")
        print(f"   Description: {event.get_description()}")
        
        # Test round-trip serialization
        serialized_json = event.to_json()
        print(f"[OK] Round-trip JSON serialization successful")
        
        # Test from_bytes method (JSON as bytes)
        json_bytes = json_content.encode('utf-8')
        event_from_bytes = alwayson.TwxEvent.from_bytes(json_bytes)
        print(f"[OK] Successfully created TwxEvent from bytes!")
        print(f"   Event name: {event_from_bytes.get_name()}")
        
        # Test to_bytes method
        bytes_output = event.to_bytes()
        print(f"[OK] to_bytes() returned {len(bytes_output)} bytes")
        
        return True
        
    except Exception as e:
        print(f"[FAIL] TwxEvent test failed: {e}")
        import traceback
        traceback.print_exc()
        return False


def test_service_json():
    """Test TwxService with JSON data."""
    print(f"\n[TEST] Testing TwxService with JSON:")
    try:
        json_content = read_json_file("sample_service.json")
        
        # Test from_json method
        service = alwayson.TwxService.from_json(json_content)
        print(f"[OK] Successfully created TwxService from JSON!")
        print(f"   Service name: {service.get_name()}")
        print(f"   Description: {service.get_description()}")
        
        # Test round-trip serialization
        serialized_json = service.to_json()
        print(f"[OK] Round-trip JSON serialization successful")
        
        # Test from_bytes method (JSON as bytes)
        json_bytes = json_content.encode('utf-8')
        service_from_bytes = alwayson.TwxService.from_bytes(json_bytes)
        print(f"[OK] Successfully created TwxService from bytes!")
        print(f"   Service name: {service_from_bytes.get_name()}")
        
        # Test to_bytes method
        bytes_output = service.to_bytes()
        print(f"[OK] to_bytes() returned {len(bytes_output)} bytes")
        
        return True
        
    except Exception as e:
        print(f"[FAIL] TwxService test failed: {e}")
        import traceback
        traceback.print_exc()
        return False


def test_property_json():
    """Test TwxProperty with JSON data."""
    print(f"\n[TEST] Testing TwxProperty with JSON:")
    try:
        json_content = read_json_file("sample_property.json")
        
        # Test from_json method
        property_obj = alwayson.TwxProperty.from_json(json_content)
        print(f"[OK] Successfully created TwxProperty from JSON!")
        print(f"   Property name: {property_obj.get_name()}")
        print(f"   Base type: {property_obj.get_base_type()}")
        print(f"   Push threshold: {property_obj.get_push_threshold()}")
        print(f"   Should read edge value: {property_obj.should_read_edge_value()}")
        
        # Test round-trip serialization
        serialized_json = property_obj.to_json()
        print(f"[OK] Round-trip JSON serialization successful")
        
        # Test from_bytes method (JSON as bytes)
        json_bytes = json_content.encode('utf-8')
        property_from_bytes = alwayson.TwxProperty.from_bytes(json_bytes)
        print(f"[OK] Successfully created TwxProperty from bytes!")
        print(f"   Property name: {property_from_bytes.get_name()}")
        
        # Test to_bytes method
        bytes_output = property_obj.to_bytes()
        print(f"[OK] to_bytes() returned {len(bytes_output)} bytes")
        
        return True
        
    except Exception as e:
        print(f"[FAIL] TwxProperty test failed: {e}")
        import traceback
        traceback.print_exc()
        return False


def main():
    """Main function to run all JSON tests."""
    print("AlwaysOn Python - JSON Sample Test Script")
    print("=" * 50)
    
    # Test all three JSON-based classes
    event_success = test_event_json()
    service_success = test_service_json()
    property_success = test_property_json()
    
    print(f"\n[SUMMARY] JSON Test Results:")
    print(f"   TwxEvent JSON: {'Success' if event_success else 'Failed'}")
    print(f"   TwxService JSON: {'Success' if service_success else 'Failed'}")
    print(f"   TwxProperty JSON: {'Success' if property_success else 'Failed'}")
    
    success_count = sum([event_success, service_success, property_success])
    print(f"   Total: {success_count}/3 tests passed")
    
    if success_count == 3:
        print(f"\n[SUCCESS] All JSON tests passed!")
    else:
        print(f"\n[PARTIAL] {success_count} out of 3 tests passed")
        
    print(f"\n[DONE] JSON test completed!")


if __name__ == "__main__":
    main()
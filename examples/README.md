# AlwaysOn Python Examples

This folder contains example scripts and sample data for testing the `alwayson_py` library.

## Files

### Test Scripts

- **`test_sample.py`** - Comprehensive test script that demonstrates:
  - Reading hex data from files
  - Converting hex to binary
  - Decoding using `TwPrim.from_bytes()` and `TwxMessage.from_bytes()`
  - Binary data structure analysis
  - Round-trip serialization/deserialization testing

- **`test_simple_string.py`** - Simple test for decoding a TwPrim string

- **`test_json_samples.py`** - JSON-based test script that demonstrates:
  - Creating TwxEvent, TwxService, and TwxProperty from JSON
  - Round-trip JSON serialization/deserialization 
  - Converting JSON to bytes and back
  - Complete functionality testing for all three JSON-based classes

- **`test_persist.py`** - Persistence data decoding test script that demonstrates:
  - Reading hex data from sample_persist.txt with proper trimming
  - Attempting Avro format decoding first (requires avro-python3 package)
  - Attempting alwayson_py decoding with all available classes
  - Partial/sliding window decoding to find embedded structures
  - Comprehensive binary data analysis and pattern detection

- **`test_persist_avro_thingworx.py`** - Enhanced ThingWorx persistence decoder based on durablequeue analysis:
  - Understands ThingWorx Avro+BaseTypes encoding format
  - Attempts schema-based Avro decoding with ThingWorx-specific schemas
  - Identifies embedded ThingWorx primitives within Avro structures
  - Detects timestamps, entity names, and property names
  - Based on analysis of actual ThingWorx platform source code

### Sample Data

- **`sample_hex.txt`** - Complex ThingWorx protocol message (100 bytes)
  - Contains: Software Manager thing, request ID, timestamp field
  - Type: Likely a multipart message or complex event
  - Status: Partially decodable (first byte as boolean TwPrim)

- **`simple_string.txt`** - Simple TwPrim string encoded as hex (13 bytes)
  - Contains: "Hello World" string primitive
  - Type: STRING TwPrim
  - Status: Fully decodable ✅

- **`sample_event.json`** - ThingWorx Event definition in JSON format
  - Contains: "SteamSensorFault" event with EventData shape
  - Type: TwxEvent JSON
  - Status: Fully decodable ✅

- **`sample_service.json`** - ThingWorx Service definition in JSON format
  - Contains: "GetSteamSensorReadings" service with input/output definitions
  - Type: TwxService JSON  
  - Status: Fully decodable ✅

- **`sample_property.json`** - ThingWorx Property definition in JSON format
  - Contains: "TotalFlow" property with push settings and aspects
  - Type: TwxProperty JSON
  - Status: Fully decodable ✅

- **`sample_persist.txt`** - ThingWorx persistence data in hex format (68 bytes)
  - Contains: demo_thing, demo_int_prop, timestamps
  - Type: ThingWorx durablequeue format (Avro outer layer + BaseTypes inner values)
  - Status: Partially decodable (entity names extracted, TwPrim primitives found)
  - Source: Likely from Kafka/EventHubs persistent property or value stream queue

## Usage

### Run the comprehensive test:
```bash
uv run python examples/test_sample.py
```

### Run the simple string test:
```bash
uv run python examples/test_simple_string.py
```

### Run the JSON samples test:
```bash
uv run python examples/test_json_samples.py
```

### Run the persistence data test:
```bash
# Install Avro library first (optional but recommended)
uv add avro-python3

# Run the test
uv run python examples/test_persist.py
```

## Sample Output

The test scripts demonstrate:

1. **Working functionality**:
   - ✅ TwPrim creation and serialization
   - ✅ Binary round-trip for primitives (string, number, boolean)
   - ✅ TwxMessage creation for auth messages
   - ✅ Simple TwPrim decoding from hex files
   - ✅ TwxEvent JSON serialization/deserialization
   - ✅ TwxService JSON serialization/deserialization  
   - ✅ TwxProperty JSON serialization/deserialization
   - ✅ JSON-to-bytes conversion for all entity types

2. **Current limitations**:
   - ❌ Complex/multipart message decoding
   - ❌ Advanced ThingWorx protocol features
   - ❌ Binary format support for Event/Service/Property (JSON only)

## Creating Your Own Test Data

To create hex data for testing:

```python
import alwayson

# Create a primitive
prim = alwayson.TwPrim.string("Your text here")

# Get hex representation
hex_data = prim.to_bytes().hex()
print("Hex:", hex_data)

# Save to file
with open("your_test.txt", "w") as f:
    f.write(hex_data)
```

## Binary Data Analysis

The test scripts provide detailed analysis of binary data:
- Byte-level structure
- Readable ASCII strings
- Length and format information
- Decoding attempts with error handling

This helps understand the ThingWorx AlwaysOn protocol structure and debug decoding issues.
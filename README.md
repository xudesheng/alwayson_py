# AlwaysOn Python

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Python bindings for the ThingWorx AlwaysOn protocol codec, enabling efficient encoding and decoding of ThingWorx AlwaysOn protocol messages in Python.

> **Note**: The underlying Rust crates (`alwayson-codec`) are not yet open source. This Python library is currently delivered as pre-compiled binary wheels only.

## Features

- 🚀 **High Performance** - Rust-powered encoding/decoding
- 🐍 **Pythonic API** - Familiar Python interfaces
- 📦 **Zero Dependencies** - Self-contained binary wheels
- 🔧 **Complete Protocol Support** - All ThingWorx primitive types and messages
- 🛡️ **Type Safety** - Comprehensive type hints
- 📊 **Multipart Messages** - Handle large message splitting/merging

## Installation

```bash
pip install alwayson-py
```

## Quick Start

```python
import alwayson

# Create primitive values
string_value = alwayson.TwPrim.string("Hello World")
number_value = alwayson.TwPrim.number(42.5)
boolean_value = alwayson.TwPrim.boolean(True)

# Serialize to JSON or binary
json_data = string_value.to_json()
binary_data = string_value.to_bytes()

# Create an authentication message
auth_msg = alwayson.TwxMessage.build_auth(12345, "your-app-key")
binary_msg = auth_msg.to_bytes()

# Parse binary messages
parsed_msg = alwayson.TwxMessage.from_bytes(binary_msg)
print(f"Message type: {parsed_msg.get_message_type()}")

# Work with InfoTables - decode from binary
binary_infotable = bytes.fromhex("010974696d657374616d70...")  # Example binary data
infotable = alwayson.InfoTable.from_bytes(binary_infotable)
print(f"InfoTable has {infotable.get_row_count()} rows, {infotable.get_field_count()} fields")

# Convert InfoTable to JSON
json_representation = infotable.to_json()
print(json_representation)

# Create empty InfoTable
empty_table = alwayson.TwPrim.infotable_empty()
print(f"Type: {empty_table.get_type()}")  # Returns "INFOTABLE"
```

## Development

This project is built with [PyO3](https://pyo3.rs/) and [maturin](https://github.com/PyO3/maturin).

### Setup

```bash
# Clone the repository
git clone https://github.com/xudesheng/alwayson_py.git
cd alwayson_py

# Create virtual environment
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate

# Install maturin
pip install maturin[patchelf]

# Development build
maturin develop
```

### Testing

```bash
pip install pytest pytest-asyncio
pytest tests/
```

## Related Projects

- `alwayson-codec` - The underlying Rust codec library (not yet open source)
- `alwayson-base` - Full ThingWorx client implementation (not yet open source)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Desheng Xu** <xudesheng@gmail.com>
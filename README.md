# AlwaysOn Python

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Python bindings for the [alwayson-codec](https://github.com/xudesheng/alwayson-codec) Rust library, enabling efficient encoding and decoding of ThingWorx AlwaysOn protocol messages in Python.

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

# Create an authentication message
auth_msg = alwayson.TwxMessage.build_auth(12345, "your-app-key")
binary_data = auth_msg.to_bytes()

# Parse binary message
parsed_msg = alwayson.TwxMessage.from_bytes(binary_data)

# Work with InfoTables
shape = alwayson.DataShape()
shape.add_field("temperature", alwayson.BaseType.NUMBER)
shape.add_field("timestamp", alwayson.BaseType.DATETIME)

table = alwayson.InfoTable(shape)
table.add_row({
    "temperature": 23.5,
    "timestamp": alwayson.datetime_now()
})
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

- [alwayson-codec](https://github.com/xudesheng/alwayson-codec) - The underlying Rust library
- [alwayson-base](https://github.com/xudesheng/alwayson-base) - Full ThingWorx client implementation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Desheng Xu** <xudesheng@gmail.com>
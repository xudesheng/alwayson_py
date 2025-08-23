# AlwaysOn Python - Development Documentation

This document contains comprehensive information about the AlwaysOn Python project, including analysis of the upstream Rust crate and implementation plans.

## Project Overview

**Goal**: Create Python bindings for the `alwayson-codec` Rust library to enable efficient encoding/decoding of ThingWorx AlwaysOn protocol messages in Python.

**Upstream Repository**: https://github.com/xudesheng/alwayson-codec
**License**: MIT
**Approach**: PyO3 + maturin for Rust-Python bindings

## Upstream Crate Analysis

### Core Architecture

The `alwayson-codec` crate is structured around several key modules:

```
src/
├── lib.rs                 # Main library exports
├── base/                  # Base types and enums
│   ├── base_type.rs       # BaseType enum for primitive types
│   ├── characteristic.rs  # Entity characteristics
│   ├── entity_type.rs     # Thing/ThingTemplate/etc types
│   ├── log_level.rs      # Logging levels
│   ├── msg_code.rs       # Message response codes
│   ├── msg_type.rs       # Message type definitions
│   └── type_family.rs    # Type family classifications
├── primitive/             # Primitive data types
│   ├── mod.rs
│   ├── dese.rs           # Deserialization
│   ├── impls.rs          # TwPrim implementations
│   └── se.rs             # Serialization
├── message/               # Protocol messages
│   ├── tw_auth.rs        # Authentication messages
│   ├── tw_bind.rs        # Binding messages
│   ├── tw_header.rs      # Message headers
│   ├── tw_message.rs     # Main message enum
│   ├── tw_multipart.rs   # Multipart message handling
│   ├── tw_rawbinary.rs   # Binary data handling
│   ├── tw_request.rs     # Request messages
│   └── tw_response.rs    # Response messages
├── datashape/             # Data structure definitions
├── infotable/             # InfoTable implementation
├── aspect/                # Thing aspects
├── property/              # Property definitions
├── service/               # Service definitions
├── event/                 # Event definitions
└── util/                  # Utilities (time, etc.)
```

### Key Types to Expose

#### 1. Primitive Types (`TwPrim`)
```rust
// From src/primitive/mod.rs
pub enum TwPrim {
    BOOLEAN(BaseType, bool),
    INTEGER(BaseType, i32),
    LONG(BaseType, i64),
    NUMBER(BaseType, f64),
    STRING(BaseType, String),
    DATETIME(BaseType, i64),
    BLOB(BaseType, Vec<u8>),
    LOCATION(BaseType, Location),
    INFOTABLE(BaseType, InfoTable),
    JSON(BaseType, serde_json::Value),
    VARIANT(BaseType, Box<TwPrim>),
}
```

**Python API Implementation** ✅:
```python
from alwayson import TwPrim, BaseType

# Create primitives
value = TwPrim.string("Hello World")
number = TwPrim.number(42.5)
boolean = TwPrim.boolean(True)

# Serialize/deserialize (IMPLEMENTED)
json_str = value.to_json()
bytes_data = value.to_bytes()
decoded = TwPrim.from_bytes(bytes_data)
```

#### 2. Message Types (`TwxMsg`)
```rust
// From src/message/tw_message.rs
pub enum TwxMsg {
    Request(TwxMsgHeader, TwxReqBody),
    Response(TwxMsgHeader, TwxResBody),
    Auth(TwxMsgHeader, TwxAuthBody),
    Bind(TwxMsgHeader, TwxBindBody),
    Multipart(TwxMsgHeader, TwMsgMultipart),
    RawBinary(RawBinary),
}
```

**Python API Goal**:
```python
from alwayson import TwxMessage

# Create auth message
auth_msg = TwxMessage.build_auth(request_id=12345, app_key="your-key")
binary_data = auth_msg.to_bytes()

# Parse message
parsed_msg = TwxMessage.from_bytes(binary_data)
```

#### 3. InfoTable and DataShape
```rust
// From src/infotable/mod.rs and src/datashape/mod.rs
pub struct InfoTable {
    pub data_shape: DataShape,
    pub rows: Vec<InfoTableRow>,
}

pub struct DataShape {
    pub field_definitions: IndexMap<String, DataShapeEntry>,
}
```

**Python API Goal**:
```python
from alwayson import InfoTable, DataShape, BaseType

# Create data shape
shape = DataShape()
shape.add_field("id", BaseType.INTEGER)
shape.add_field("name", BaseType.STRING)
shape.add_field("value", BaseType.NUMBER)

# Create InfoTable
table = InfoTable(shape)
table.add_row({"id": 1, "name": "Sensor1", "value": 23.5})
```

#### 4. Multipart Message Handling
```rust
// From src/message/tw_multipart.rs
pub struct TwMsgMultipartDispatcherSync { /* ... */ }
pub struct TwMsgMultipartReceiverSync { /* ... */ }
```

**Python API Goal**:
```python
from alwayson import MultipartDispatcher, MultipartReceiver

# Split large message
dispatcher = MultipartDispatcher()
chunks = dispatcher.split(large_message)

# Reassemble
receiver = MultipartReceiver()
for chunk in chunks:
    complete_msg = receiver.add_chunk(chunk)
    if complete_msg:
        break
```

### Protocol Features Supported

1. **All ThingWorx Primitive Types**:
   - BOOLEAN, INTEGER, LONG, NUMBER (f64)
   - STRING, DATETIME, BLOB, LOCATION
   - INFOTABLE, JSON, VARIANT

2. **Message Types**:
   - Authentication (`TwxAuthBody`)
   - Bind requests (`TwxBindBody`)
   - Service requests/responses (`TwxReqBody`/`TwxResBody`)
   - Property updates and events
   - Multipart message handling

3. **Data Structures**:
   - `DataShape` - ThingWorx data shape definitions
   - `InfoTable` - Structured data tables
   - `TwxProperty`, `TwxService`, `TwxEvent` - Entity definitions

4. **Core Traits**:
   - `BytesStream` - Binary serialization/deserialization
   - Serde integration for JSON

### Dependencies
```toml
bytes = "1.5"
thiserror = "1.0"
byteorder = "1.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
indexmap = "2.1"
chrono = "0.4"
# ... and others
```

## Implementation Plan

### Phase 1: Basic Bindings Structure ✅
- [x] Project setup with PyO3/maturin
- [x] Cargo.toml with alwayson-codec dependency
- [x] Basic Python module structure
- [x] Build configuration

### Phase 2: Core Type Bindings
- [ ] Wrap `BaseType` enum
- [ ] Wrap `TwPrim` with all variants
- [ ] Basic serialization/deserialization methods
- [ ] Error handling integration

### Phase 3: Message System
- [ ] Wrap `TwxMsg` enum
- [ ] Authentication message builders
- [ ] Request/response message handling
- [ ] Binary format conversion

### Phase 4: Data Structures
- [ ] `DataShape` and `DataShapeEntry` wrappers
- [ ] `InfoTable` and `InfoTableRow` wrappers
- [ ] Complex data manipulation methods

### Phase 5: Advanced Features
- [ ] Multipart message handling
- [ ] Service/Property/Event definitions
- [ ] Aspect support
- [ ] Performance optimizations

### Phase 6: Python Conveniences
- [ ] Pythonic APIs and sugar methods
- [ ] Type hints and stub files
- [ ] Documentation and examples
- [ ] Async/await support considerations

## Technical Considerations

### PyO3 Integration Challenges

1. **Enum Handling**: Rust enums with data need careful mapping to Python
2. **Error Conversion**: `AlwaysOnError` needs Python exception mapping
3. **Memory Management**: Avoiding unnecessary copies for large data
4. **Complex Types**: IndexMap, chrono types need conversion

### Performance Goals

1. **Zero-copy where possible**: For large binary data and InfoTables
2. **Efficient serialization**: Leverage Rust performance for encoding/decoding
3. **Memory efficiency**: Minimize Python object overhead

### API Design Principles

1. **Pythonic**: Follow Python conventions and idioms
2. **Type Safety**: Provide comprehensive type hints
3. **Error Handling**: Clear Python exceptions with context
4. **Documentation**: Comprehensive docstrings and examples

## Build Instructions

### Development Setup
**Requirements**: This project uses `uv` package manager with Python 3.12.

```bash
# Install uv (if not already installed)
curl -LsSf https://astral.sh/uv/install.sh | sh  # Linux/macOS
# or: powershell -c "irm https://astral.sh/uv/install.ps1 | iex"  # Windows

# Development build
uv run maturin develop

# Production build  
uv run maturin build --release
```

### Testing
```bash
# Run tests with uv
uv run pytest tests/

# Test basic functionality
uv run python -c "import alwayson; print('Version:', alwayson.__version__)"
```

## Repository References

### Upstream Dependencies
- **alwayson-codec**: https://github.com/xudesheng/alwayson-codec
  - Branch: `main` 
  - Used via Git dependency in Cargo.toml
  - License: MIT

### Related Projects
- **alwayson-base**: https://github.com/xudesheng/alwayson-base (Full ThingWorx client)
- **ThingWorx Platform**: https://www.ptc.com/en/products/iot/thingworx-platform

## Error Handling Strategy

### Rust Error Types
```rust
// From src/error.rs
pub enum AlwaysOnError {
    InvalidBaseType(u8),
    SerdeJson(serde_json::Error),
    Utf8Error(std::str::Utf8Error),
    // ... others
}
```

### Python Exception Mapping
```python
class AlwaysOnError(Exception): ...
class InvalidBaseTypeError(AlwaysOnError): ...
class SerializationError(AlwaysOnError): ...
class DeserializationError(AlwaysOnError): ...
```

## Example Usage Scenarios

### 1. Basic Message Creation
```python
import alwayson

# Create and serialize an auth message
auth = alwayson.TwxMessage.build_auth(12345, "app-key")
binary_data = auth.to_bytes()
print(f"Message size: {len(binary_data)} bytes")

# Decode binary message
decoded = alwayson.TwxMessage.from_bytes(binary_data)
print(f"Message type: {decoded.get_message_type()}")
```

### 2. InfoTable Manipulation
```python
import alwayson

# Build InfoTable with sensor data
shape = alwayson.DataShape()
shape.add_field("timestamp", alwayson.BaseType.DATETIME)
shape.add_field("temperature", alwayson.BaseType.NUMBER)
shape.add_field("humidity", alwayson.BaseType.NUMBER)

table = alwayson.InfoTable(shape)
table.add_row({
    "timestamp": alwayson.TwPrim.datetime_now(),
    "temperature": alwayson.TwPrim.number(23.5),
    "humidity": alwayson.TwPrim.number(45.2)
})

# Serialize to ThingWorx format
json_data = table.to_json()
binary_data = table.to_bytes()
```

### 3. Large Message Handling
```python
import alwayson

# Handle large messages with multipart
large_data = create_large_infotable()
dispatcher = alwayson.MultipartDispatcher()

chunks = dispatcher.split_message(large_data)
print(f"Split into {len(chunks)} chunks")

# Reassemble on receiver side
receiver = alwayson.MultipartReceiver()
for chunk in chunks:
    result = receiver.add_chunk(chunk)
    if result:
        print("Message reassembled successfully")
        break
```

## Performance Benchmarks (Target)

Based on upstream Rust performance:
- **Serialization**: ~1-5μs for typical messages
- **Deserialization**: ~2-8μs for typical messages  
- **Memory usage**: Minimal allocations, zero-copy where possible
- **Multipart handling**: ~100MB/s throughput for large messages

Python bindings should maintain 80-90% of native Rust performance.

## Development Status

- [x] Project structure created
- [x] Build configuration setup  
- [x] Dependencies configured
- [x] Core type bindings (`TwPrim`, `BaseType`)
- [x] Binary serialization/deserialization support
- [x] Message system implementation (`TwxMessage`)
- [x] Authentication message support
- [x] PyPI package published (v0.1.1)
- [ ] Data structure wrappers (`InfoTable`, `DataShape`)
- [ ] Advanced features (multipart messages, events)
- [ ] Documentation and examples

## Contributing

### Code Style
- Rust: Follow `rustfmt` and `clippy` recommendations
- Python: Use `black`, `isort`, and `ruff` for formatting and linting
- Type hints required for all public APIs

### Pre-Commit Requirements
**IMPORTANT**: Before committing and pushing any code changes, ALWAYS run these checks IN ORDER:

1. **Rust Clippy Check**: 
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```
   - This ensures no clippy warnings or errors
   - Fix any issues found or add appropriate `#[allow()]` attributes for false positives

2. **Rust Format Check**:
   ```bash
   cargo fmt --all -- --check
   ```
   - This ensures proper Rust code formatting
   - If this fails, run `cargo fmt --all` to automatically fix formatting

3. **Python Format Check (Black)**:
   ```bash
   uv run black --check python/
   ```
   - This ensures proper Python code formatting
   - If this fails, run `uv run black python/` to automatically fix formatting
   - Make sure black is installed: `uv add --dev black`

4. **Python Import Sort Check (isort)**:
   ```bash
   uv run isort --check-only python/
   ```
   - This ensures Python imports are properly sorted and organized
   - If this fails, run `uv run isort python/` to automatically fix import order
   - Make sure isort is installed: `uv add --dev isort`

5. **Python Lint Check (ruff)**:
   ```bash
   uv run ruff check python/
   ```
   - This ensures Python code follows linting rules and best practices
   - If this fails, run `uv run ruff check --fix python/` to automatically fix issues
   - Make sure ruff is installed: `uv add --dev ruff`

**All five checks MUST pass before any code is committed.** This ensures code quality, consistency, and prevents CI/CD pipeline failures.

### Testing
- Unit tests for all public APIs
- Integration tests with real ThingWorx data
- Performance benchmarks
- Documentation examples that run as tests

---

**Last Updated**: 2025-08-22
**Author**: Desheng Xu <xudesheng@gmail.com>
**License**: MIT
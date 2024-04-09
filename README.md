# Hehners_math
![HEHm](https://github.com/HermiTech-LLC/Hehners_math/blob/main/HEHm.JPG)

## Overview
"Hehners_math" is a Rust-implemented math module dedicated to Hehner's unified algebra. This module offers practical tools for handling numbers with uncertainties, catering to arithmetic, vector, and matrix operations. It is designed with a focus on practicality, ease of use, and precision, making it a suitable tool for applications in science and engineering where accounting for uncertainty is critical.

## Key Features
- **UnifiedNumber**: Facilitates arithmetic operations with uncertainties, including basic, logarithmic, exponential, and trigonometric functions.
- **UnifiedVector**: Supports vector operations like addition and subtraction, emphasizing size compatibility.
- **UnifiedMatrix**: Enables matrix operations, focusing on size adherence and basic matrix functionalities.
- **Python Interoperability**: Uses the `cpython` crate for integration with Python, enhancing its utility in cross-language scenarios.
- **Symbolic Representation**: Offers symbolic representations for a clearer understanding of mathematical operations.
- **Efficient and Precise**: Aimed at delivering efficient performance in high-precision calculations, especially in complex scenarios.

## Future Expansion
The module aims to:
- Incorporate additional vector and matrix operations.
- Improve performance and computational efficiency.
- Extend cross-language interoperability, with a focus on Python integration.
- Continuously adapt and align with evolving scientific methodologies and Hehner's unified algebra principles.

## System Requirements
- Rust Programming Language (2021 Edition or later).
- `cpython` crate for Python connectivity.

## Installation Guide
To install "Hehners_math," follow these steps:
```bash
git clone http://github.com/HermiTech-LLC/Hehners_math.git
cd Hehners_math
cargo build --release
```

## Usage Example
Example of basic usage in Rust:
```rust
use Hehners_math::{UnifiedNumber, UnifiedVector, UnifiedMatrix};

fn main() {
    // Initializing UnifiedNumbers
    let num1 = UnifiedNumber::new(2.0, 0.1);
    let num2 = UnifiedNumber::new(3.0, 0.2);

    // Performing operations
    let result = num1.add(&num2);
    // Additional examples and operations...
}
```

## License
Licensed under the GNU Affero General Public License v3 (AGPL-3.0). For more details, refer to the [LICENSE](LICENSE) file.

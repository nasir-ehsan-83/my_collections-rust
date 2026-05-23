# mycollections-rust

![Rust](https://img.shields.io/badge/Rust-Language-orange)
![Status](https://img.shields.io/badge/status-in--progress-yellow)
![License](https://img.shields.io/badge/license-MIT-blue)

A collection of data structures and algorithms implemented in Rust with an emphasis on ownership-aware APIs, predictable performance, and low-level implementation details.

The project explores how classic algorithms and data structures can be designed using idiomatic Rust while maintaining explicit memory semantics and minimal abstraction overhead.

---

## Scope

Current and planned implementations include:

- Sorting algorithms
- Core data structures
- Searching algorithms
- Dynamic programming
- Graph algorithms
- String processing algorithms

---

## Project Structure

```text
src/
├── sort/
├── collections/
├── search/
├── dynamic_programming/
├── graph_algorithms/
└── string/
```

---

## Implemented

### Sorting Algorithms
- Bubble Sort
- Selection Sort
- Insertion Sort
- Shell Sort
- Heap Sort
- Merge Sort
- Quick Sort
- TimSort
- IntroSort
- Counting Sort
- Radix Sort
- Bucket Sort

*Additional variants and optimizations are added incrementally as the project evolves.*

---

## Design Focus

The implementations are written with attention to:

*   Ownership and borrowing semantics
*   Generic and reusable APIs
*   Allocation-aware design
*   Iterator-oriented patterns
*   Predictable complexity characteristics
*   Low-level implementation trade-offs
*   Zero-cost abstractions where applicable

**Some modules may later include:**
*   Benchmarking
*   Allocation profiling
*   Implementation comparisons
*   Unsafe vs safe implementation analysis

---

## Documentation

Each module is intended to document:

- Algorithmic complexity
- Implementation details
- API behavior
- Ownership trade-offs
- Rust-specific design decisions

*Examples and tests are written alongside implementations whenever possible.*

---

## Getting Started

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone the Repository
```bash
git clone https://github.com/nasir-ehsan-83/mycollections-rust
cd mycollections-rust
```

### Run Tests
```bash
cargo test
```

---

## Contributing

Implementation improvements, optimizations, and corrections are welcome.

---

## License

Licensed under the MIT License.

---

## Author

**Nasir Ahmad Ehsan**

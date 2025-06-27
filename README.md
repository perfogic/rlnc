# rlnc
`rlnc` is a Rust library that implements Random Linear Network Coding (RLNC) over $GF(2^8)$ with primitive polynomial $x^8 + x^4 + x^3 + x^2 + 1$. This library provides functionalities for encoding data, decoding coded pieces to recover the original data, and recoding existing coded pieces.

For a quick understanding of RLNC, have a look at my blog post @ https://itzmeanjan.in/pages/rlnc-in-depth.html.

Random Linear Network Coding (RLNC) excels in highly dynamic and lossy environments like multicast, peer-to-peer networks, and distributed storage, due to its "any K of N" property and inherent recoding capability. Unlike Reed-Solomon, which requires specific symbols for deterministic recovery, RLNC allows decoding from *any* set of linearly independent packets. Compared to Fountain Codes, RLNC offers robust algebraic linearity with coding vector overhead, whereas Fountain codes prioritize very low decoding complexity and indefinite symbol generation, often for large-scale broadcasts.

## Features
- **Encoder**: Splits original data into fixed-size pieces and generates new coded pieces by linearly combining these original pieces with random coefficients, sampled from $GF(2^8)$.
- **Decoder**: Receives coded pieces, applies Gaussian elimination to recover the original data, and handles linearly dependent pieces gracefully.
- **Recoder**: Takes already coded pieces and generates new coded pieces from them, facilitating multi-hop data distribution without requiring intermediate decoding.
- **Error Handling**: Defines a custom `RLNCError` enum to provide clear error messages for various operational failures.

## Prerequisites
Rust stable toolchain; see https://rustup.rs for installation guide. MSRV for this crate is 1.85.0.

 ```bash
# While developing this library, I was using
$ rustc --version
rustc 1.88.0 (6b00bc388 2025-06-23)
```

## Running Tests
For ensuring functional correctness of RLNC operations, the library includes a comprehensive test suite. Run all the tests.

```bash
# Testing on host.
make test

# Testing on web assembly target, using `wasmtime`.
rustup target add wasm32-wasip1
cargo install wasmtime-cli --locked
make test-wasm
```

```bash
running 4 tests
test common::gf256::test::prop_test_gf256_operations ... ok
test full::tests::prop_test_rlnc_encoder_decoder ... ok
test full::tests::prop_test_rlnc_encoder_recoder_decoder ... ok
test full::tests::prop_test_rlnc_decoding_with_useless_pieces ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 41.92s
```

> [!NOTE]
> There is a help menu, which introduces you to all available commands; just run `$ make` from the root directory of this project.

## Running Benchmarks

Performance benchmarks are included to evaluate the efficiency of the RLNC scheme. These benchmarks measure the time taken for various RLNC operations.

To run the benchmarks, execute the following command from the root of the project:

```bash
make bench
```

> [!WARNING]
> When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be misleading. I find https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

### On 12th Gen Intel(R) Core(TM) i7-1260P

Running benchmarks on `Linux 6.14.0-22-generic x86_64`, compiled with `rustc 1.88.0 (6b00bc388 2025-06-23)`.

- **Encoder Performance:** The `encode` operation consistently achieves high throughputs, generally ranging from **1.5 GiB/s to 1.8 GiB/s**. This performance remains robust and stable across varying data sizes (1MB to 32MB) and increasing piece counts (16 to 256). This indicates an efficient implementation that handles the computational demands of generating coded pieces very well, with increasing coding vector size having a minimal impact on throughput within these ranges.
- **Recoder Performance:** The `recode` operation stands out with exceptionally high throughputs, typically ranging between **1.7 GiB/s and 2.2 GiB/s**. This impressive speed highlights the recoder's efficiency in relaying coded data, enabling rapid generation of new linearly independent combinations from existing ones, which is crucial for decentralized and multi-hop network scenarios.
- **Decoder Performance:** The `decode` operation's throughput **decreases significantly as the number of pieces increases**, for a given data size.
  - For **1MB of data**, the throughput drops from approximately **60 MiB/s** (16 pieces) down to about **3.8 MiB/s** (256 pieces).
  - Similarly, for **16MB of data**, it goes from roughly **59 MiB/s** (16 pieces) to about **3.7 MiB/s** (256 pieces).
  - For **32MB of data**, it's around **59 MiB/s** (16 pieces) and approximately **29.6 MiB/s** (32 pieces).
  
  This trend is expected due to the higher computational complexity of Gaussian elimination when working with larger coefficient matrices (more pieces).

In summary, the `rlnc` Rust library crate provides very high-speed encoding and recoding capabilities. The decoding performance, while still functional, is inversely proportional to the number of pieces, reflecting the increased matrix operations required for reconstruction. This characteristic makes `rlnc` particularly valuable where data dissemination and intermediate re-encoding are frequent, and where the number of original pieces (`piece_count`) for decoding is kept at a reasonable level or where the computational cost of decoding is acceptable for the given `piece_count`.

#### Full RLNC Encoder

```bash
Timer precision: 14 ns
full_rlnc_encoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ encode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    610.4 µs      │ 724.2 µs      │ 638.3 µs      │ 643.1 µs      │ 100     │ 100
   │                                          1.699 GiB/s   │ 1.432 GiB/s   │ 1.625 GiB/s   │ 1.613 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            1.187 MiB   │ 1.187 MiB     │ 1.187 MiB     │ 1.187 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            1.125 MiB   │ 1.125 MiB     │ 1.125 MiB     │ 1.125 MiB     │         │
   ├─ 1.00 MB data splitted into 32 pieces    580.6 µs      │ 731 µs        │ 615.2 µs      │ 626.5 µs      │ 100     │ 100
   │                                          1.734 GiB/s   │ 1.377 GiB/s   │ 1.637 GiB/s   │ 1.607 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            64.06 KiB   │ 64.06 KiB     │ 64.06 KiB     │ 64.06 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            1.093 MiB   │ 1.093 MiB     │ 1.093 MiB     │ 1.093 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            1.062 MiB   │ 1.062 MiB     │ 1.062 MiB     │ 1.062 MiB     │         │
   ├─ 1.00 MB data splitted into 64 pieces    589.5 µs      │ 669.6 µs      │ 608.1 µs      │ 608.8 µs      │ 100     │ 100
   │                                          1.682 GiB/s   │ 1.481 GiB/s   │ 1.631 GiB/s   │ 1.629 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            32.12 KiB   │ 32.12 KiB     │ 32.12 KiB     │ 32.12 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            1.047 MiB   │ 1.047 MiB     │ 1.047 MiB     │ 1.047 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            1.031 MiB   │ 1.031 MiB     │ 1.031 MiB     │ 1.031 MiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces   595.6 µs      │ 642.4 µs      │ 608.9 µs      │ 608.8 µs      │ 100     │ 100
   │                                          1.652 GiB/s   │ 1.532 GiB/s   │ 1.616 GiB/s   │ 1.616 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            16.25 KiB   │ 16.25 KiB     │ 16.25 KiB     │ 16.25 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            1.023 MiB   │ 1.023 MiB     │ 1.023 MiB     │ 1.023 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            1.015 MiB   │ 1.015 MiB     │ 1.015 MiB     │ 1.015 MiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces   602.9 µs      │ 626.1 µs      │ 611.8 µs      │ 612.6 µs      │ 100     │ 100
   │                                          1.626 GiB/s   │ 1.566 GiB/s   │ 1.603 GiB/s   │ 1.601 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            8.501 KiB   │ 8.501 KiB     │ 8.501 KiB     │ 8.501 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            260         │ 260           │ 260           │ 260           │         │
   │                                            1.012 MiB   │ 1.012 MiB     │ 1.012 MiB     │ 1.012 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            259         │ 259           │ 259           │ 259           │         │
   │                                            1.008 MiB   │ 1.008 MiB     │ 1.008 MiB     │ 1.008 MiB     │         │
   ├─ 16.00 MB data splitted into 16 pieces   9.24 ms       │ 10.72 ms      │ 10.39 ms      │ 10.35 ms      │ 100     │ 100
   │                                          1.796 GiB/s   │ 1.547 GiB/s   │ 1.597 GiB/s   │ 1.603 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            19 MiB      │ 19 MiB        │ 19 MiB        │ 19 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            18 MiB      │ 18 MiB        │ 18 MiB        │ 18 MiB        │         │
   ├─ 16.00 MB data splitted into 32 pieces   9.871 ms      │ 10.46 ms      │ 10.18 ms      │ 10.18 ms      │ 100     │ 100
   │                                          1.632 GiB/s   │ 1.54 GiB/s    │ 1.581 GiB/s   │ 1.582 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            17.5 MiB    │ 17.5 MiB      │ 17.5 MiB      │ 17.5 MiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   ├─ 16.00 MB data splitted into 64 pieces   9.586 ms      │ 10.17 ms      │ 9.864 ms      │ 9.86 ms       │ 100     │ 100
   │                                          1.655 GiB/s   │ 1.56 GiB/s    │ 1.608 GiB/s   │ 1.609 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            16.75 MiB   │ 16.75 MiB     │ 16.75 MiB     │ 16.75 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            16.5 MiB    │ 16.5 MiB      │ 16.5 MiB      │ 16.5 MiB      │         │
   ├─ 16.00 MB data splitted into 128 pieces  9.613 ms      │ 9.985 ms      │ 9.814 ms      │ 9.802 ms      │ 100     │ 100
   │                                          1.638 GiB/s   │ 1.577 GiB/s   │ 1.604 GiB/s   │ 1.606 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            16.37 MiB   │ 16.37 MiB     │ 16.37 MiB     │ 16.37 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            16.25 MiB   │ 16.25 MiB     │ 16.25 MiB     │ 16.25 MiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces  9.593 ms      │ 9.953 ms      │ 9.782 ms      │ 9.784 ms      │ 100     │ 100
   │                                          1.635 GiB/s   │ 1.576 GiB/s   │ 1.603 GiB/s   │ 1.603 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128.5 KiB   │ 128.5 KiB     │ 128.5 KiB     │ 128.5 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            260         │ 260           │ 260           │ 260           │         │
   │                                            16.18 MiB   │ 16.18 MiB     │ 16.18 MiB     │ 16.18 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            259         │ 259           │ 259           │ 259           │         │
   │                                            16.12 MiB   │ 16.12 MiB     │ 16.12 MiB     │ 16.12 MiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces   21.07 ms      │ 22.54 ms      │ 22.17 ms      │ 22.14 ms      │ 100     │ 100
   │                                          1.575 GiB/s   │ 1.472 GiB/s   │ 1.497 GiB/s   │ 1.499 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            38 MiB      │ 38 MiB        │ 38 MiB        │ 38 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            36 MiB      │ 36 MiB        │ 36 MiB        │ 36 MiB        │         │
   ├─ 32.00 MB data splitted into 32 pieces   19.52 ms      │ 21.46 ms      │ 20.63 ms      │ 20.58 ms      │ 100     │ 100
   │                                          1.65 GiB/s    │ 1.501 GiB/s   │ 1.561 GiB/s   │ 1.565 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            35 MiB      │ 35 MiB        │ 35 MiB        │ 35 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            34 MiB      │ 34 MiB        │ 34 MiB        │ 34 MiB        │         │
   ├─ 32.00 MB data splitted into 64 pieces   19.72 ms      │ 20.52 ms      │ 20.28 ms      │ 20.23 ms      │ 100     │ 100
   │                                          1.609 GiB/s   │ 1.546 GiB/s   │ 1.564 GiB/s   │ 1.568 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            33.5 MiB    │ 33.5 MiB      │ 33.5 MiB      │ 33.5 MiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            33 MiB      │ 33 MiB        │ 33 MiB        │ 33 MiB        │         │
   ├─ 32.00 MB data splitted into 128 pieces  19.34 ms      │ 19.8 ms       │ 19.64 ms      │ 19.6 ms       │ 100     │ 100
   │                                          1.628 GiB/s   │ 1.589 GiB/s   │ 1.603 GiB/s   │ 1.606 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.2 KiB   │ 512.2 KiB     │ 512.2 KiB     │ 512.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            32.75 MiB   │ 32.75 MiB     │ 32.75 MiB     │ 32.75 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            32.5 MiB    │ 32.5 MiB      │ 32.5 MiB      │ 32.5 MiB      │         │
   ╰─ 32.00 MB data splitted into 256 pieces  19.35 ms      │ 19.73 ms      │ 19.58 ms      │ 19.56 ms      │ 100     │ 100
                                              1.62 GiB/s    │ 1.589 GiB/s   │ 1.602 GiB/s   │ 1.603 GiB/s   │         │
                                              max alloc:    │               │               │               │         │
                                                3           │ 3             │ 3             │ 3             │         │
                                                256.5 KiB   │ 256.5 KiB     │ 256.5 KiB     │ 256.5 KiB     │         │
                                              alloc:        │               │               │               │         │
                                                260         │ 260           │ 260           │ 260           │         │
                                                32.37 MiB   │ 32.37 MiB     │ 32.37 MiB     │ 32.37 MiB     │         │
                                              dealloc:      │               │               │               │         │
                                                259         │ 259           │ 259           │ 259           │         │
                                                32.25 MiB   │ 32.25 MiB     │ 32.25 MiB     │ 32.25 MiB     │         │
```

#### Full RLNC Recoder

```bash
Timer precision: 13 ns
full_rlnc_recoder                                                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ recode                                                                             │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces, recoding with 8 pieces      252.4 µs      │ 308.8 µs      │ 284.8 µs      │ 286.4 µs      │ 100     │ 100
   │                                                                    2.176 GiB/s   │ 1.779 GiB/s   │ 1.929 GiB/s   │ 1.918 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      768 KiB     │ 768 KiB       │ 768 KiB       │ 768 KiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      704 KiB     │ 704 KiB       │ 704 KiB       │ 704 KiB       │         │
   ├─ 1.00 MB data splitted into 32 pieces, recoding with 16 pieces     265.2 µs      │ 323 µs        │ 285.8 µs      │ 284.9 µs      │ 100     │ 100
   │                                                                    1.957 GiB/s   │ 1.607 GiB/s   │ 1.816 GiB/s   │ 1.822 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.09 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      640.1 KiB   │ 640.1 KiB     │ 640.1 KiB     │ 640.1 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      608 KiB     │ 608 KiB       │ 608 KiB       │ 608 KiB       │         │
   ├─ 1.00 MB data splitted into 64 pieces, recoding with 32 pieces     268.5 µs      │ 296.3 µs      │ 280.9 µs      │ 279.8 µs      │ 100     │ 100
   │                                                                    1.882 GiB/s   │ 1.706 GiB/s   │ 1.799 GiB/s   │ 1.806 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.18 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      576.2 KiB   │ 576.2 KiB     │ 576.2 KiB     │ 576.2 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      560.1 KiB   │ 560.1 KiB     │ 560.1 KiB     │ 560.1 KiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces, recoding with 64 pieces    274.5 µs      │ 307.8 µs      │ 287.4 µs      │ 286.8 µs      │ 100     │ 100
   │                                                                    1.834 GiB/s   │ 1.636 GiB/s   │ 1.752 GiB/s   │ 1.756 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.37 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      544.4 KiB   │ 544.4 KiB     │ 544.4 KiB     │ 544.4 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      536.3 KiB   │ 536.3 KiB     │ 536.3 KiB     │ 536.3 KiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces, recoding with 128 pieces   301.9 µs      │ 329.4 µs      │ 309.1 µs      │ 309.3 µs      │ 100     │ 100
   │                                                                    1.732 GiB/s   │ 1.587 GiB/s   │ 1.691 GiB/s   │ 1.69 GiB/s    │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.751 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      134         │ 134           │ 134           │ 134           │         │
   │                                                                      528.8 KiB   │ 528.8 KiB     │ 528.8 KiB     │ 528.8 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      133         │ 133           │ 133           │ 133           │         │
   │                                                                      524.6 KiB   │ 524.6 KiB     │ 524.6 KiB     │ 524.6 KiB     │         │
   ├─ 16.00 MB data splitted into 16 pieces, recoding with 8 pieces     4.382 ms      │ 5.275 ms      │ 4.925 ms      │ 4.969 ms      │ 100     │ 100
   │                                                                    2.005 GiB/s   │ 1.666 GiB/s   │ 1.784 GiB/s   │ 1.768 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      12 MiB      │ 12 MiB        │ 12 MiB        │ 12 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      11 MiB      │ 11 MiB        │ 11 MiB        │ 11 MiB        │         │
   ├─ 16.00 MB data splitted into 32 pieces, recoding with 16 pieces    4.187 ms      │ 4.775 ms      │ 4.696 ms      │ 4.677 ms      │ 100     │ 100
   │                                                                    1.982 GiB/s   │ 1.738 GiB/s   │ 1.767 GiB/s   │ 1.774 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      10 MiB      │ 10 MiB        │ 10 MiB        │ 10 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      9.5 MiB     │ 9.5 MiB       │ 9.5 MiB       │ 9.5 MiB       │         │
   ├─ 16.00 MB data splitted into 64 pieces, recoding with 32 pieces    4.367 ms      │ 4.575 ms      │ 4.52 ms       │ 4.508 ms      │ 100     │ 100
   │                                                                    1.845 GiB/s   │ 1.761 GiB/s   │ 1.782 GiB/s   │ 1.787 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      9 MiB       │ 9 MiB         │ 9 MiB         │ 9 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      8.75 MiB    │ 8.75 MiB      │ 8.75 MiB      │ 8.75 MiB      │         │
   ├─ 16.00 MB data splitted into 128 pieces, recoding with 64 pieces   4.332 ms      │ 4.536 ms      │ 4.467 ms      │ 4.463 ms      │ 100     │ 100
   │                                                                    1.833 GiB/s   │ 1.75 GiB/s    │ 1.777 GiB/s   │ 1.779 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      8.5 MiB     │ 8.5 MiB       │ 8.5 MiB       │ 8.5 MiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      8.375 MiB   │ 8.375 MiB     │ 8.375 MiB     │ 8.375 MiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces, recoding with 128 pieces  4.401 ms      │ 4.537 ms      │ 4.476 ms      │ 4.476 ms      │ 100     │ 100
   │                                                                    1.795 GiB/s   │ 1.741 GiB/s   │ 1.765 GiB/s   │ 1.765 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      134         │ 134           │ 134           │ 134           │         │
   │                                                                      8.25 MiB    │ 8.25 MiB      │ 8.25 MiB      │ 8.25 MiB      │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      133         │ 133           │ 133           │ 133           │         │
   │                                                                      8.188 MiB   │ 8.188 MiB     │ 8.188 MiB     │ 8.188 MiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces, recoding with 8 pieces     8.873 ms      │ 10.07 ms      │ 9.945 ms      │ 9.895 ms      │ 100     │ 100
   │                                                                    1.981 GiB/s   │ 1.744 GiB/s   │ 1.767 GiB/s   │ 1.776 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      24 MiB      │ 24 MiB        │ 24 MiB        │ 24 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      22 MiB      │ 22 MiB        │ 22 MiB        │ 22 MiB        │         │
   ├─ 32.00 MB data splitted into 32 pieces, recoding with 16 pieces    9.067 ms      │ 10.68 ms      │ 9.59 ms       │ 9.579 ms      │ 100     │ 100
   │                                                                    1.831 GiB/s   │ 1.553 GiB/s   │ 1.731 GiB/s   │ 1.733 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      20 MiB      │ 20 MiB        │ 20 MiB        │ 20 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      19 MiB      │ 19 MiB        │ 19 MiB        │ 19 MiB        │         │
   ├─ 32.00 MB data splitted into 64 pieces, recoding with 32 pieces    8.736 ms      │ 9.399 ms      │ 9.303 ms      │ 9.277 ms      │ 100     │ 100
   │                                                                    1.844 GiB/s   │ 1.714 GiB/s   │ 1.732 GiB/s   │ 1.737 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      18 MiB      │ 18 MiB        │ 18 MiB        │ 18 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      17.5 MiB    │ 17.5 MiB      │ 17.5 MiB      │ 17.5 MiB      │         │
   ├─ 32.00 MB data splitted into 128 pieces, recoding with 64 pieces   8.744 ms      │ 9.111 ms      │ 8.981 ms      │ 8.967 ms      │ 100     │ 100
   │                                                                    1.815 GiB/s   │ 1.742 GiB/s   │ 1.767 GiB/s   │ 1.77 GiB/s    │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      16.75 MiB   │ 16.75 MiB     │ 16.75 MiB     │ 16.75 MiB     │         │
   ╰─ 32.00 MB data splitted into 256 pieces, recoding with 128 pieces  8.745 ms      │ 9.063 ms      │ 8.935 ms      │ 8.936 ms      │ 100     │ 100
                                                                        1.804 GiB/s   │ 1.74 GiB/s    │ 1.765 GiB/s   │ 1.765 GiB/s   │         │
                                                                        max alloc:    │               │               │               │         │
                                                                          4           │ 4             │ 4             │ 4             │         │
                                                                          256.7 KiB   │ 256.7 KiB     │ 256.7 KiB     │ 256.7 KiB     │         │
                                                                        alloc:        │               │               │               │         │
                                                                          134         │ 134           │ 134           │ 134           │         │
                                                                          16.5 MiB    │ 16.5 MiB      │ 16.5 MiB      │ 16.5 MiB      │         │
                                                                        dealloc:      │               │               │               │         │
                                                                          133         │ 133           │ 133           │ 133           │         │
                                                                          16.37 MiB   │ 16.37 MiB     │ 16.37 MiB     │ 16.37 MiB     │         │
```

#### Full RLNC Decoder

```bash
Timer precision: 22 ns
full_rlnc_decoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ decode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    16.46 ms      │ 19.21 ms      │ 16.5 ms       │ 16.55 ms      │ 100     │ 100
   │                                          60.76 MiB/s   │ 52.05 MiB/s   │ 60.58 MiB/s   │ 60.43 MiB/s   │         │
   ├─ 1.00 MB data splitted into 32 pieces    33.09 ms      │ 33.56 ms      │ 33.13 ms      │ 33.16 ms      │ 100     │ 100
   │                                          30.24 MiB/s   │ 29.82 MiB/s   │ 30.2 MiB/s    │ 30.18 MiB/s   │         │
   ├─ 1.00 MB data splitted into 64 pieces    66.83 ms      │ 71.01 ms      │ 66.96 ms      │ 67.02 ms      │ 100     │ 100
   │                                          15.02 MiB/s   │ 14.13 MiB/s   │ 14.99 MiB/s   │ 14.97 MiB/s   │         │
   ├─ 1.00 MB data splitted into 128 pieces   135.2 ms      │ 135.5 ms      │ 135.3 ms      │ 135.3 ms      │ 100     │ 100
   │                                          7.51 MiB/s    │ 7.492 MiB/s   │ 7.505 MiB/s   │ 7.504 MiB/s   │         │
   ├─ 1.00 MB data splitted into 256 pieces   280 ms        │ 280.5 ms      │ 280.2 ms      │ 280.2 ms      │ 100     │ 100
   │                                          3.795 MiB/s   │ 3.787 MiB/s   │ 3.791 MiB/s   │ 3.791 MiB/s   │         │
   ├─ 16.00 MB data splitted into 16 pieces   267.7 ms      │ 273 ms        │ 269.8 ms      │ 269.7 ms      │ 100     │ 100
   │                                          59.75 MiB/s   │ 58.58 MiB/s   │ 59.29 MiB/s   │ 59.32 MiB/s   │         │
   ├─ 16.00 MB data splitted into 32 pieces   537.4 ms      │ 542.9 ms      │ 539.9 ms      │ 539.8 ms      │ 100     │ 100
   │                                          29.76 MiB/s   │ 29.47 MiB/s   │ 29.63 MiB/s   │ 29.63 MiB/s   │         │
   ├─ 16.00 MB data splitted into 64 pieces   1.081 s       │ 1.092 s       │ 1.084 s       │ 1.084 s       │ 93      │ 93
   │                                          14.79 MiB/s   │ 14.65 MiB/s   │ 14.76 MiB/s   │ 14.76 MiB/s   │         │
   ├─ 16.00 MB data splitted into 128 pieces  2.167 s       │ 2.171 s       │ 2.169 s       │ 2.169 s       │ 47      │ 47
   │                                          7.389 MiB/s   │ 7.374 MiB/s   │ 7.382 MiB/s   │ 7.381 MiB/s   │         │
   ├─ 16.00 MB data splitted into 256 pieces  4.352 s       │ 4.358 s       │ 4.356 s       │ 4.356 s       │ 23      │ 23
   │                                          3.69 MiB/s    │ 3.685 MiB/s   │ 3.687 MiB/s   │ 3.687 MiB/s   │         │
   ├─ 32.00 MB data splitted into 16 pieces   540.6 ms      │ 545.2 ms      │ 543.4 ms      │ 543.2 ms      │ 100     │ 100
   │                                          59.18 MiB/s   │ 58.68 MiB/s   │ 58.88 MiB/s   │ 58.9 MiB/s    │         │
   ├─ 32.00 MB data splitted into 32 pieces   1.077 s       │ 1.084 s       │ 1.08 s        │ 1.08 s        │ 93      │ 93
   │                                          29.7 MiB/s    │ 29.5 MiB/s    │ 29.6 MiB/s    │ 29.6 MiB/s    │         │
   ├─ 32.00 MB data splitted into 64 pieces   2.158 s       │ 2.164 s       │ 2.161 s       │ 2.161 s       │ 47      │ 47
   │                                          14.82 MiB/s   │ 14.78 MiB/s   │ 14.8 MiB/s    │ 14.8 MiB/s    │         │
   ├─ 32.00 MB data splitted into 128 pieces  4.319 s       │ 4.381 s       │ 4.322 s       │ 4.326 s       │ 24      │ 24
   │                                          7.412 MiB/s   │ 7.306 MiB/s   │ 7.407 MiB/s   │ 7.399 MiB/s   │         │
   ╰─ 32.00 MB data splitted into 256 pieces  8.649 s       │ 8.668 s       │ 8.654 s       │ 8.655 s       │ 12      │ 12
                                              3.706 MiB/s   │ 3.698 MiB/s   │ 3.704 MiB/s   │ 3.704 MiB/s   │         │
```

## Getting Started

To use `rlnc` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rlnc = "0.1.0" # Use the latest version available
rand = { version = "=0.9.1", features = ["small_rng"] } # Required for random number generation
```

### Full RLNC Workflow Example

I maintain an example demonstrating the Full RLNC workflow:

- Encoding original data
- Recoding to generate new pieces (simulating a relay node).
- Finally decoding all received pieces to recover the original data.

> [!NOTE]
> New recoded pieces could be either useful or not, based on Recoder input coded pieces from which they are recoded from and whether they have already been seen by Decoder or not.

See [full_rlnc.rs](./examples/full_rlnc.rs) example program. Run the program with `$ make example`.

# rlnc
Random Linear Network Coding

## Introduction
`rlnc` is a Rust library that implements Random Linear Network Coding (RLNC) over $GF(2^8)$ with primitive polynomial $x^8 + x^4 + x^3 + x^2 + 1$. This library provides functionalities for encoding data, decoding coded pieces to recover the original data, and recoding existing coded pieces.

For a quick understanding of RLNC, have a look at my blog post @ https://itzmeanjan.in/pages/rlnc-in-depth.html.

Random Linear Network Coding (RLNC) excels in highly dynamic and lossy environments like multicast, peer-to-peer networks, and distributed storage, due to its "any K of N" property and inherent recoding capability. Unlike Reed-Solomon, which requires specific symbols for deterministic recovery, RLNC allows decoding from *any* set of linearly independent packets. Compared to Fountain Codes, RLNC offers robust algebraic linearity with coding vector overhead, whereas Fountain codes prioritize very low decoding complexity and indefinite symbol generation, often for large-scale broadcasts.

## Features
For now this crate implements only **Full RLNC** scheme.

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

## Testing
For ensuring functional correctness of RLNC operations, the library includes a comprehensive test suite. Run all the tests.

```bash
# Testing on host.
make test

# Testing on web assembly target, using `wasmtime`.
rustup target add wasm32-wasip1
cargo install wasmtime-cli --locked
make test-wasm
```

### Code Coverage
To generate a detailed code coverage report in HTML format, use [cargo-tarpaulin](https://github.com/xd009642/tarpaulin):

```bash
# Install cargo-tarpaulin if not already installed
cargo install cargo-tarpaulin
make coverage
```

```bash
Coverage Results:
|| Tested/Total Lines:
|| src/common/errors.rs: 0/1
|| src/common/gf256.rs: 11/13
|| src/full/decoder.rs: 67/73
|| src/full/encoder.rs: 31/33
|| src/full/recoder.rs: 30/36
||
89.10% coverage, 139/156 lines covered
```

This will create an HTML coverage report at `tarpaulin-report.html` that you can open in your web browser to view detailed line-by-line coverage information for all source files.

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

## Benchmarking
Performance benchmarks are included to evaluate the efficiency of the RLNC scheme. These benchmarks measure the time taken for various RLNC operations.

To run the benchmarks, execute the following command from the root of the project:

```bash
make bench
```

> [!WARNING]
> When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be misleading. I find https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

### On 12th Gen Intel(R) Core(TM) i7-1260P

Running benchmarks on `Linux 6.14.0-22-generic x86_64`, compiled with `rustc 1.88.0 (6b00bc388 2025-06-23)`.

- **Full RLNC Encoder Performance**
The encoder demonstrates strong performance, generally achieving throughputs between **1.6 GiB/s and 1.8 GiB/s**.
  - **Speed:** Encoding 1MB of data typically takes around 500-600 microseconds. For larger files, such as 32MB, the encoding time is in the range of 17-21 milliseconds.
  - **Impact of Number of Pieces:** The number of pieces the data is split into (e.g., 16, 32, 64, 128, 256) has a minimal impact on the encoding speed, with throughputs remaining consistently high across these variations.

- **Full RLNC Recoder Performance**
The recoder is notably faster than the encoder, often reaching throughputs between **1.9 GiB/s and 2.6 GiB/s**.
  - **Speed:** Recoding 1MB of data is very quick, taking approximately 200-270 microseconds. For 32MB of data, recoding completes in roughly 7-9 milliseconds.
  - **Impact of Number of Pieces:** Similar to the encoder, the recoder's performance remains largely consistent regardless of how many pieces the data is split into, demonstrating robust speed across different configurations.

- **Full RLNC Decoder Performance**
The decoder's performance is considerably slower compared to both the encoder and recoder, with throughputs ranging from **4 MiB/s to 70 MiB/s**.
  - **Speed:** Decoding 1MB of data can take a significant amount of time, from about 14 milliseconds (when split into 16 pieces) up to over 250 milliseconds (when split into 256 pieces). For larger files like 32MB, decoding can extend to several seconds (e.g., over 7 seconds for 256 pieces).
  - **Impact of Number of Pieces:** The most significant parameter impact is observed in the decoding phase. As the number of pieces increases, the decoding time increases substantially, leading to a considerable drop in throughput. This indicates that decoding is the most computationally intensive part of the full RLNC process, and its performance is inversely proportional to the number of pieces.

In summary, the full RLNC implementation excels in encoding and recoding speeds, maintaining GiB/s throughputs with minimal sensitivity to the number of data pieces. However, decoding is a much slower operation, with its performance significantly diminishing as the data is split into a greater number of pieces.

#### Full RLNC Encoder

```bash
Timer precision: 18 ns
full_rlnc_encoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ encode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    499.9 µs      │ 674 µs        │ 564.5 µs      │ 565.1 µs      │ 100     │ 100
   │                                          2.075 GiB/s   │ 1.539 GiB/s   │ 1.837 GiB/s   │ 1.836 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            1.187 MiB   │ 1.187 MiB     │ 1.187 MiB     │ 1.187 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            1.125 MiB   │ 1.125 MiB     │ 1.125 MiB     │ 1.125 MiB     │         │
   ├─ 1.00 MB data splitted into 32 pieces    514.6 µs      │ 655 µs        │ 567.9 µs      │ 576.6 µs      │ 100     │ 100
   │                                          1.957 GiB/s   │ 1.537 GiB/s   │ 1.773 GiB/s   │ 1.746 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            64.06 KiB   │ 64.06 KiB     │ 64.06 KiB     │ 64.06 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            1.093 MiB   │ 1.093 MiB     │ 1.093 MiB     │ 1.093 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            1.062 MiB   │ 1.062 MiB     │ 1.062 MiB     │ 1.062 MiB     │         │
   ├─ 1.00 MB data splitted into 64 pieces    527 µs        │ 635.9 µs      │ 554.9 µs      │ 562.7 µs      │ 100     │ 100
   │                                          1.882 GiB/s   │ 1.559 GiB/s   │ 1.787 GiB/s   │ 1.762 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            32.12 KiB   │ 32.12 KiB     │ 32.12 KiB     │ 32.12 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            1.047 MiB   │ 1.047 MiB     │ 1.047 MiB     │ 1.047 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            1.031 MiB   │ 1.031 MiB     │ 1.031 MiB     │ 1.031 MiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces   519.7 µs      │ 632.7 µs      │ 576.9 µs      │ 576.7 µs      │ 100     │ 100
   │                                          1.894 GiB/s   │ 1.555 GiB/s   │ 1.706 GiB/s   │ 1.706 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            16.25 KiB   │ 16.25 KiB     │ 16.25 KiB     │ 16.25 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            1.023 MiB   │ 1.023 MiB     │ 1.023 MiB     │ 1.023 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            1.015 MiB   │ 1.015 MiB     │ 1.015 MiB     │ 1.015 MiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces   525.8 µs      │ 604.2 µs      │ 555.3 µs      │ 556.6 µs      │ 100     │ 100
   │                                          1.865 GiB/s   │ 1.623 GiB/s   │ 1.766 GiB/s   │ 1.761 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            8.501 KiB   │ 8.501 KiB     │ 8.501 KiB     │ 8.501 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            260         │ 260           │ 260           │ 260           │         │
   │                                            1.012 MiB   │ 1.012 MiB     │ 1.012 MiB     │ 1.012 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            259         │ 259           │ 259           │ 259           │         │
   │                                            1.008 MiB   │ 1.008 MiB     │ 1.008 MiB     │ 1.008 MiB     │         │
   ├─ 16.00 MB data splitted into 16 pieces   8.843 ms      │ 11.91 ms      │ 9.638 ms      │ 9.672 ms      │ 100     │ 100
   │                                          1.877 GiB/s   │ 1.393 GiB/s   │ 1.722 GiB/s   │ 1.716 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            19 MiB      │ 19 MiB        │ 19 MiB        │ 19 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            18 MiB      │ 18 MiB        │ 18 MiB        │ 18 MiB        │         │
   ├─ 16.00 MB data splitted into 32 pieces   8.83 ms       │ 11.82 ms      │ 9.331 ms      │ 9.386 ms      │ 100     │ 100
   │                                          1.824 GiB/s   │ 1.362 GiB/s   │ 1.726 GiB/s   │ 1.716 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            17.5 MiB    │ 17.5 MiB      │ 17.5 MiB      │ 17.5 MiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   ├─ 16.00 MB data splitted into 64 pieces   8.709 ms      │ 11.46 ms      │ 9.112 ms      │ 9.17 ms       │ 100     │ 100
   │                                          1.822 GiB/s   │ 1.384 GiB/s   │ 1.741 GiB/s   │ 1.73 GiB/s    │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            16.75 MiB   │ 16.75 MiB     │ 16.75 MiB     │ 16.75 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            16.5 MiB    │ 16.5 MiB      │ 16.5 MiB      │ 16.5 MiB      │         │
   ├─ 16.00 MB data splitted into 128 pieces  8.676 ms      │ 11.83 ms      │ 9.034 ms      │ 9.13 ms       │ 100     │ 100
   │                                          1.814 GiB/s   │ 1.33 GiB/s    │ 1.742 GiB/s   │ 1.724 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            16.37 MiB   │ 16.37 MiB     │ 16.37 MiB     │ 16.37 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            16.25 MiB   │ 16.25 MiB     │ 16.25 MiB     │ 16.25 MiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces  8.624 ms      │ 11.62 ms      │ 9.051 ms      │ 9.135 ms      │ 100     │ 100
   │                                          1.818 GiB/s   │ 1.349 GiB/s   │ 1.733 GiB/s   │ 1.717 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128.5 KiB   │ 128.5 KiB     │ 128.5 KiB     │ 128.5 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            260         │ 260           │ 260           │ 260           │         │
   │                                            16.18 MiB   │ 16.18 MiB     │ 16.18 MiB     │ 16.18 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            259         │ 259           │ 259           │ 259           │         │
   │                                            16.12 MiB   │ 16.12 MiB     │ 16.12 MiB     │ 16.12 MiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces   18.33 ms      │ 23.09 ms      │ 20.41 ms      │ 20.5 ms       │ 100     │ 100
   │                                          1.811 GiB/s   │ 1.437 GiB/s   │ 1.626 GiB/s   │ 1.619 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            38 MiB      │ 38 MiB        │ 38 MiB        │ 38 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            36 MiB      │ 36 MiB        │ 36 MiB        │ 36 MiB        │         │
   ├─ 32.00 MB data splitted into 32 pieces   17.91 ms      │ 21.94 ms      │ 19.13 ms      │ 19.2 ms       │ 100     │ 100
   │                                          1.799 GiB/s   │ 1.468 GiB/s   │ 1.683 GiB/s   │ 1.677 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            35 MiB      │ 35 MiB        │ 35 MiB        │ 35 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            34 MiB      │ 34 MiB        │ 34 MiB        │ 34 MiB        │         │
   ├─ 32.00 MB data splitted into 64 pieces   17.73 ms      │ 21.62 ms      │ 18.72 ms      │ 18.73 ms      │ 100     │ 100
   │                                          1.789 GiB/s   │ 1.467 GiB/s   │ 1.695 GiB/s   │ 1.694 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            33.5 MiB    │ 33.5 MiB      │ 33.5 MiB      │ 33.5 MiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            33 MiB      │ 33 MiB        │ 33 MiB        │ 33 MiB        │         │
   ├─ 32.00 MB data splitted into 128 pieces  17.34 ms      │ 21.49 ms      │ 18.14 ms      │ 18.23 ms      │ 100     │ 100
   │                                          1.815 GiB/s   │ 1.465 GiB/s   │ 1.736 GiB/s   │ 1.727 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.2 KiB   │ 512.2 KiB     │ 512.2 KiB     │ 512.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            32.75 MiB   │ 32.75 MiB     │ 32.75 MiB     │ 32.75 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            32.5 MiB    │ 32.5 MiB      │ 32.5 MiB      │ 32.5 MiB      │         │
   ╰─ 32.00 MB data splitted into 256 pieces  17.43 ms      │ 21.25 ms      │ 18.03 ms      │ 18.14 ms      │ 100     │ 100
                                              1.799 GiB/s   │ 1.476 GiB/s   │ 1.739 GiB/s   │ 1.729 GiB/s   │         │
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
Timer precision: 11 ns
full_rlnc_recoder                                                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ recode                                                                             │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces, recoding with 8 pieces      204.9 µs      │ 294.2 µs      │ 232.2 µs      │ 237.2 µs      │ 100     │ 100
   │                                                                    2.68 GiB/s    │ 1.867 GiB/s   │ 2.365 GiB/s   │ 2.316 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      768 KiB     │ 768 KiB       │ 768 KiB       │ 768 KiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      704 KiB     │ 704 KiB       │ 704 KiB       │ 704 KiB       │         │
   ├─ 1.00 MB data splitted into 32 pieces, recoding with 16 pieces     220.5 µs      │ 283 µs        │ 246.4 µs      │ 248.8 µs      │ 100     │ 100
   │                                                                    2.354 GiB/s   │ 1.834 GiB/s   │ 2.107 GiB/s   │ 2.087 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.09 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      640.1 KiB   │ 640.1 KiB     │ 640.1 KiB     │ 640.1 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      608 KiB     │ 608 KiB       │ 608 KiB       │ 608 KiB       │         │
   ├─ 1.00 MB data splitted into 64 pieces, recoding with 32 pieces     230.4 µs      │ 271.2 µs      │ 248.2 µs      │ 248.7 µs      │ 100     │ 100
   │                                                                    2.193 GiB/s   │ 1.863 GiB/s   │ 2.036 GiB/s   │ 2.032 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.18 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      576.2 KiB   │ 576.2 KiB     │ 576.2 KiB     │ 576.2 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      560.1 KiB   │ 560.1 KiB     │ 560.1 KiB     │ 560.1 KiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces, recoding with 64 pieces    236.4 µs      │ 364.2 µs      │ 248.9 µs      │ 251.9 µs      │ 100     │ 100
   │                                                                    2.13 GiB/s    │ 1.382 GiB/s   │ 2.023 GiB/s   │ 1.999 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.37 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      544.4 KiB   │ 544.4 KiB     │ 544.4 KiB     │ 544.4 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      536.3 KiB   │ 536.3 KiB     │ 536.3 KiB     │ 536.3 KiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces, recoding with 128 pieces   260.9 µs      │ 289.9 µs      │ 268.8 µs      │ 269.5 µs      │ 100     │ 100
   │                                                                    2.003 GiB/s   │ 1.803 GiB/s   │ 1.945 GiB/s   │ 1.94 GiB/s    │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.751 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      134         │ 134           │ 134           │ 134           │         │
   │                                                                      528.8 KiB   │ 528.8 KiB     │ 528.8 KiB     │ 528.8 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      133         │ 133           │ 133           │ 133           │         │
   │                                                                      524.6 KiB   │ 524.6 KiB     │ 524.6 KiB     │ 524.6 KiB     │         │
   ├─ 16.00 MB data splitted into 16 pieces, recoding with 8 pieces     3.626 ms      │ 4.455 ms      │ 4.105 ms      │ 4.096 ms      │ 100     │ 100
   │                                                                    2.423 GiB/s   │ 1.972 GiB/s   │ 2.14 GiB/s    │ 2.145 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      12 MiB      │ 12 MiB        │ 12 MiB        │ 12 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      11 MiB      │ 11 MiB        │ 11 MiB        │ 11 MiB        │         │
   ├─ 16.00 MB data splitted into 32 pieces, recoding with 16 pieces    3.486 ms      │ 4.691 ms      │ 3.96 ms       │ 3.996 ms      │ 100     │ 100
   │                                                                    2.381 GiB/s   │ 1.769 GiB/s   │ 2.096 GiB/s   │ 2.076 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      10 MiB      │ 10 MiB        │ 10 MiB        │ 10 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      9.5 MiB     │ 9.5 MiB       │ 9.5 MiB       │ 9.5 MiB       │         │
   ├─ 16.00 MB data splitted into 64 pieces, recoding with 32 pieces    3.666 ms      │ 4.094 ms      │ 3.835 ms      │ 3.839 ms      │ 100     │ 100
   │                                                                    2.198 GiB/s   │ 1.968 GiB/s   │ 2.1 GiB/s     │ 2.099 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      9 MiB       │ 9 MiB         │ 9 MiB         │ 9 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      8.75 MiB    │ 8.75 MiB      │ 8.75 MiB      │ 8.75 MiB      │         │
   ├─ 16.00 MB data splitted into 128 pieces, recoding with 64 pieces   3.624 ms      │ 4.021 ms      │ 3.781 ms      │ 3.784 ms      │ 100     │ 100
   │                                                                    2.191 GiB/s   │ 1.975 GiB/s   │ 2.1 GiB/s     │ 2.098 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      8.5 MiB     │ 8.5 MiB       │ 8.5 MiB       │ 8.5 MiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      8.375 MiB   │ 8.375 MiB     │ 8.375 MiB     │ 8.375 MiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces, recoding with 128 pieces  3.618 ms      │ 4.173 ms      │ 3.804 ms      │ 3.803 ms      │ 100     │ 100
   │                                                                    2.184 GiB/s   │ 1.894 GiB/s   │ 2.077 GiB/s   │ 2.077 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      134         │ 134           │ 134           │ 134           │         │
   │                                                                      8.25 MiB    │ 8.25 MiB      │ 8.25 MiB      │ 8.25 MiB      │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      133         │ 133           │ 133           │ 133           │         │
   │                                                                      8.188 MiB   │ 8.188 MiB     │ 8.188 MiB     │ 8.188 MiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces, recoding with 8 pieces     7.551 ms      │ 9.252 ms      │ 8.57 ms       │ 8.563 ms      │ 100     │ 100
   │                                                                    2.327 GiB/s   │ 1.899 GiB/s   │ 2.051 GiB/s   │ 2.052 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      24 MiB      │ 24 MiB        │ 24 MiB        │ 24 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      22 MiB      │ 22 MiB        │ 22 MiB        │ 22 MiB        │         │
   ├─ 32.00 MB data splitted into 32 pieces, recoding with 16 pieces    7.717 ms      │ 8.69 ms       │ 8.263 ms      │ 8.252 ms      │ 100     │ 100
   │                                                                    2.151 GiB/s   │ 1.91 GiB/s    │ 2.009 GiB/s   │ 2.011 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      20 MiB      │ 20 MiB        │ 20 MiB        │ 20 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      19 MiB      │ 19 MiB        │ 19 MiB        │ 19 MiB        │         │
   ├─ 32.00 MB data splitted into 64 pieces, recoding with 32 pieces    7.351 ms      │ 8.51 ms       │ 8.024 ms      │ 8.023 ms      │ 100     │ 100
   │                                                                    2.192 GiB/s   │ 1.893 GiB/s   │ 2.008 GiB/s   │ 2.008 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      18 MiB      │ 18 MiB        │ 18 MiB        │ 18 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      17.5 MiB    │ 17.5 MiB      │ 17.5 MiB      │ 17.5 MiB      │         │
   ├─ 32.00 MB data splitted into 128 pieces, recoding with 64 pieces   7.298 ms      │ 7.958 ms      │ 7.743 ms      │ 7.736 ms      │ 100     │ 100
   │                                                                    2.175 GiB/s   │ 1.994 GiB/s   │ 2.05 GiB/s    │ 2.052 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      16.75 MiB   │ 16.75 MiB     │ 16.75 MiB     │ 16.75 MiB     │         │
   ╰─ 32.00 MB data splitted into 256 pieces, recoding with 128 pieces  7.84 ms       │ 8.806 ms      │ 8.196 ms      │ 8.201 ms      │ 100     │ 100
                                                                        2.012 GiB/s   │ 1.791 GiB/s   │ 1.925 GiB/s   │ 1.923 GiB/s   │         │
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
Timer precision: 19 ns
full_rlnc_decoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ decode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    14.32 ms      │ 16.78 ms      │ 15.09 ms      │ 15.14 ms      │ 100     │ 100
   │                                          69.84 MiB/s   │ 59.58 MiB/s   │ 66.25 MiB/s   │ 66.03 MiB/s   │         │
   ├─ 1.00 MB data splitted into 32 pieces    29.54 ms      │ 31.87 ms      │ 30.17 ms      │ 30.29 ms      │ 100     │ 100
   │                                          33.88 MiB/s   │ 31.4 MiB/s    │ 33.17 MiB/s   │ 33.04 MiB/s   │         │
   ├─ 1.00 MB data splitted into 64 pieces    59.35 ms      │ 61.93 ms      │ 60.08 ms      │ 60.18 ms      │ 100     │ 100
   │                                          16.91 MiB/s   │ 16.2 MiB/s    │ 16.7 MiB/s    │ 16.68 MiB/s   │         │
   ├─ 1.00 MB data splitted into 128 pieces   119.6 ms      │ 122.4 ms      │ 120.7 ms      │ 120.7 ms      │ 100     │ 100
   │                                          8.49 MiB/s    │ 8.294 MiB/s   │ 8.412 MiB/s   │ 8.408 MiB/s   │         │
   ├─ 1.00 MB data splitted into 256 pieces   242.5 ms      │ 258.6 ms      │ 252 ms        │ 251.9 ms      │ 100     │ 100
   │                                          4.382 MiB/s   │ 4.108 MiB/s   │ 4.216 MiB/s   │ 4.218 MiB/s   │         │
   ├─ 16.00 MB data splitted into 16 pieces   240.5 ms      │ 258.7 ms      │ 243.3 ms      │ 244.7 ms      │ 100     │ 100
   │                                          66.52 MiB/s   │ 61.83 MiB/s   │ 65.73 MiB/s   │ 65.37 MiB/s   │         │
   ├─ 16.00 MB data splitted into 32 pieces   475.2 ms      │ 520.6 ms      │ 495.4 ms      │ 496.2 ms      │ 100     │ 100
   │                                          33.66 MiB/s   │ 30.73 MiB/s   │ 32.29 MiB/s   │ 32.24 MiB/s   │         │
   ├─ 16.00 MB data splitted into 64 pieces   963.2 ms      │ 1.014 s       │ 992.5 ms      │ 991.3 ms      │ 100     │ 100
   │                                          16.61 MiB/s   │ 15.77 MiB/s   │ 16.12 MiB/s   │ 16.14 MiB/s   │         │
   ├─ 16.00 MB data splitted into 128 pieces  1.943 s       │ 2.019 s       │ 1.994 s       │ 1.99 s        │ 51      │ 51
   │                                          8.24 MiB/s    │ 7.929 MiB/s   │ 8.031 MiB/s   │ 8.044 MiB/s   │         │
   ├─ 16.00 MB data splitted into 256 pieces  3.864 s       │ 4.027 s       │ 3.893 s       │ 3.911 s       │ 26      │ 26
   │                                          4.156 MiB/s   │ 3.988 MiB/s   │ 4.125 MiB/s   │ 4.106 MiB/s   │         │
   ├─ 32.00 MB data splitted into 16 pieces   489.8 ms      │ 516.6 ms      │ 497.6 ms      │ 499.3 ms      │ 100     │ 100
   │                                          65.32 MiB/s   │ 61.93 MiB/s   │ 64.3 MiB/s    │ 64.08 MiB/s   │         │
   ├─ 32.00 MB data splitted into 32 pieces   968.8 ms      │ 1.059 s       │ 1 s           │ 1.003 s       │ 100     │ 100
   │                                          33.03 MiB/s   │ 30.21 MiB/s   │ 31.99 MiB/s   │ 31.87 MiB/s   │         │
   ├─ 32.00 MB data splitted into 64 pieces   1.913 s       │ 2.075 s       │ 1.965 s       │ 1.97 s        │ 51      │ 51
   │                                          16.72 MiB/s   │ 15.42 MiB/s   │ 16.27 MiB/s   │ 16.23 MiB/s   │         │
   ├─ 32.00 MB data splitted into 128 pieces  3.87 s        │ 3.938 s       │ 3.896 s       │ 3.899 s       │ 26      │ 26
   │                                          8.272 MiB/s   │ 8.128 MiB/s   │ 8.216 MiB/s   │ 8.21 MiB/s    │         │
   ╰─ 32.00 MB data splitted into 256 pieces  7.733 s       │ 7.897 s       │ 7.767 s       │ 7.783 s       │ 13      │ 13
                                              4.145 MiB/s   │ 4.06 MiB/s    │ 4.127 MiB/s   │ 4.119 MiB/s   │         │
```

## Usage

To use `rlnc` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rlnc = "=0.3.0" # Use the latest version available
rand = { version = "=0.9.1" } # Required for random number generation
```

### Full RLNC Workflow Example

I maintain an example demonstrating the Full RLNC workflow:

- Encoding original data into coded pieces.
- Recoding to generate new pieces from received coded pieces. It simulates a relay node.
- Finally decoding all received pieces to recover the original data.

> [!NOTE]
> New recoded pieces could be either useful or not for the Decoder, based on Recoder input coded pieces i.e. from which they are recoded and whether they have already been seen by Decoder or not.

See [full_rlnc.rs](./examples/full_rlnc.rs) example program. Run the program with `$ make example`.

```bash
Initialized Encoder with 10240 bytes of data, split into 32 pieces, each of 321 bytes. Each coded piece will be of 353 bytes.
Initializing Decoder, expecting 32 original pieces of 321 bytes each.

Sender generating 16 initial coded pieces...
  Decoded direct piece 1: Useful.
  Decoded direct piece 2: Useful.
  Decoded direct piece 3: Useful.
  Decoded direct piece 4: Useful.
  Decoded direct piece 5: Useful.
  Decoded direct piece 6: Useful.
  Decoded direct piece 7: Useful.
  Decoded direct piece 8: Useful.
  Decoded direct piece 9: Useful.
  Decoded direct piece 10: Useful.
  Decoded direct piece 11: Useful.
  Decoded direct piece 12: Useful.
  Decoded direct piece 13: Useful.
  Decoded direct piece 14: Useful.
  Decoded direct piece 15: Useful.
  Decoded direct piece 16: Useful.

Initializing Recoder with 5648 bytes of received coded pieces.

Recoder active. Generating recoded pieces...
  Decoded recoded piece 1: Not useful.
  Decoded recoded piece 2: Not useful.
  Decoded recoded piece 3: Not useful.
  Decoded recoded piece 4: Not useful.
  Decoded recoded piece 5: Not useful.
  Decoded recoded piece 6: Not useful.
  Decoded recoded piece 7: Not useful.
  Decoded recoded piece 8: Not useful.
  Decoded recoded piece 9: Not useful.
  Decoded recoded piece 10: Not useful.
  Decoded recoded piece 11: Not useful.
  Decoded recoded piece 12: Not useful.
  Decoded recoded piece 13: Not useful.
  Decoded recoded piece 14: Not useful.
  Decoded recoded piece 15: Not useful.
  Decoded recoded piece 16: Not useful.
  Decoded recoded piece 17: Not useful.
  Decoded recoded piece 18: Not useful.
  Decoded recoded piece 19: Not useful.
  Decoded recoded piece 20: Not useful.
  Decoded recoded piece 21: Not useful.
  Decoded recoded piece 22: Not useful.
  Decoded recoded piece 23: Not useful.
  Decoded recoded piece 24: Not useful.
  Decoded recoded piece 25: Not useful.
  Decoded recoded piece 26: Not useful.
  Decoded recoded piece 27: Not useful.
  Decoded recoded piece 28: Not useful.
  Decoded recoded piece 29: Not useful.
  Decoded recoded piece 30: Not useful.
  Decoded recoded piece 31: Not useful.
  Decoded recoded piece 32: Not useful.
  Decoded recoded piece 33: Not useful.
  Decoded recoded piece 34: Not useful.
  Decoded recoded piece 35: Not useful.
  Decoded recoded piece 36: Not useful.
  Decoded recoded piece 37: Not useful.
  Decoded recoded piece 38: Not useful.
  Decoded recoded piece 39: Not useful.
  Decoded recoded piece 40: Not useful.
  Decoded recoded piece 41: Not useful.
  Decoded recoded piece 42: Not useful.
  Decoded recoded piece 43: Not useful.
  Decoded recoded piece 44: Not useful.
  Decoded recoded piece 45: Not useful.
  Decoded recoded piece 46: Not useful.
  Decoded recoded piece 47: Not useful.
  Decoded recoded piece 48: Not useful.
  Decoded recoded piece 49: Not useful.
  Decoded recoded piece 50: Not useful.
  Decoded recoded piece 51: Not useful.
  Decoded recoded piece 52: Not useful.
  Decoded recoded piece 53: Not useful.
  Decoded recoded piece 54: Not useful.
  Decoded recoded piece 55: Not useful.
  Decoded recoded piece 56: Not useful.
  Decoded recoded piece 57: Not useful.
  Decoded recoded piece 58: Not useful.
  Decoded recoded piece 59: Not useful.
  Decoded recoded piece 60: Not useful.
  Decoded recoded piece 61: Not useful.
  Decoded recoded piece 62: Not useful.
  Decoded recoded piece 63: Not useful.
  Decoded recoded piece 64: Not useful.

Initializing a new Recoder with 5648 bytes of received coded pieces.
  Decoded recoded piece 1: Useful.
  Decoded recoded piece 2: Useful.
  Decoded recoded piece 3: Useful.
  Decoded recoded piece 4: Useful.
  Decoded recoded piece 5: Useful.
  Decoded recoded piece 6: Useful.
  Decoded recoded piece 7: Useful.
  Decoded recoded piece 8: Useful.

Still need more pieces. Generating direct piece 17 from encoder...
  Decoded direct piece 17: Useful.

Still need more pieces. Generating direct piece 18 from encoder...
  Decoded direct piece 18: Useful.

Still need more pieces. Generating direct piece 19 from encoder...
  Decoded direct piece 19: Useful.

Still need more pieces. Generating direct piece 20 from encoder...
  Decoded direct piece 20: Useful.

Still need more pieces. Generating direct piece 21 from encoder...
  Decoded direct piece 21: Useful.

Still need more pieces. Generating direct piece 22 from encoder...
  Decoded direct piece 22: Useful.

Still need more pieces. Generating direct piece 23 from encoder...
  Decoded direct piece 23: Useful.

Still need more pieces. Generating direct piece 24 from encoder...
  Decoded direct piece 24: Useful.

Retrieving decoded data...

RLNC workflow completed successfully! Original data matches decoded data.
```

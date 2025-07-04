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
# Testing on host, first with `default` feature, then with `parallel` feature enabled.
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
running 13 tests
test full::decoder::tests::test_decoder_decode_invalid_piece_length ... ok
test full::decoder::tests::test_decoder_new_invalid_inputs ... ok
test full::encoder::tests::test_encoder_getters ... ok
test full::encoder::tests::test_encoder_code_with_coding_vector_invalid_inputs ... ok
test full::encoder::tests::test_encoder_without_padding_invalid_data ... ok
test full::recoder::tests::test_recoder_getters ... ok
test full::recoder::tests::test_recoder_new_invalid_inputs ... ok
test full::encoder::tests::test_encoder_new_invalid_inputs ... ok
test full::decoder::tests::test_decoder_getters ... ok
test common::gf256::test::prop_test_gf256_operations ... ok
test full::tests::prop_test_rlnc_encoder_decoder ... ok
test full::tests::prop_test_rlnc_encoder_recoder_decoder ... ok
test full::tests::prop_test_rlnc_decoding_with_useless_pieces has been running for over 60 seconds
test full::tests::prop_test_rlnc_decoding_with_useless_pieces ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 69.01s

   Doc-tests rlnc

running 1 test
test src/lib.rs - (line 50) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

> [!NOTE]
> There is a help menu, which introduces you to all available commands; just run `$ make` from the root directory of this project.

## Benchmarking
Performance benchmarks are included to evaluate the efficiency of the RLNC scheme. These benchmarks measure the time taken for various RLNC operations.

To run the benchmarks, execute the following command from the root of the project:

```bash
make bench # First with `default` feature, then with `parallel` feature enabled.
```

> [!WARNING]
> When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be misleading. I find https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

### On 12th Gen Intel(R) Core(TM) i7-1260P

Running benchmarks on `Linux 6.14.0-23-generic x86_64`, compiled with `rustc 1.88.0 (6b00bc388 2025-06-23)`.

- **Full RLNC Encoder Performance**
  The encoder demonstrates strong performance.
  - **`default` Feature:** Achieves throughputs consistently between **1.6 GiB/s and 2.3 GiB/s**. Encoding 1MB of data typically takes around 500-600 microseconds, and for 32MB, it takes 15-19 milliseconds.
  - **`parallel` Feature (Rayon):** Significantly boosts performance, reaching throughputs between **3.4 GiB/s and 9.7 GiB/s**. Encoding 1MB of data is much faster, typically taking 100-280 microseconds, and for 32MB, it takes 3.2-8.6 milliseconds.
  - **Impact of Number of Pieces:** For both default and parallel implementations, the number of pieces the data is split into has a minimal impact on the encoding speed, with throughputs remaining consistently high across various piece counts.

- **Full RLNC Recoder Performance**
  The recoder is also very fast.
  - **`default` Feature:** Achieves throughputs between **1.6 GiB/s and 2.6 GiB/s**. Recoding 1MB of data typically takes 200-320 microseconds, and for 32MB, it takes 7.8-10.1 milliseconds.
  - **`parallel` Feature (Rayon):** Shows substantial improvements, reaching throughputs between **2.9 GiB/s and 7.9 GiB/s**. Recoding 1MB of data is very quick, taking approximately 85-190 microseconds, and for 32MB, it takes 2.0-5.6 milliseconds.
  - **Impact of Number of Pieces:** Similar to the encoder, the recoder's performance remains largely consistent regardless of how many pieces the data is split into, demonstrating robust speed across different configurations.

- **Full RLNC Decoder Performance**
  The decoder's performance is considerably slower compared to both the encoder and recoder, as expected due to the computational complexity of Gaussian elimination.
  - **Default Feature (No Parallelism):** Throughputs range from **4 MiB/s to 74 MiB/s**. Decoding 1MB of data can take a significant amount of time, from about 13-18 milliseconds (when split into 16 pieces) up to over 230-250 milliseconds (when split into 256 pieces). For larger files like 32MB, decoding can extend to several seconds (e.g., over 7 seconds for 256 pieces).
  - **Impact of Number of Pieces:** The most significant parameter impact is observed in the decoding phase. As the number of pieces increases, the decoding time increases substantially, leading to a considerable drop in throughput. This indicates that decoding is the most computationally intensive part of the full RLNC process, and its performance is inversely proportional to the number of pieces. Parallelism benefits from this operation are minimal due to the sequential nature of Gaussian elimination.

In summary, the full RLNC implementation excels in encoding and recoding speeds, maintaining GiB/s throughputs with minimal sensitivity to the number of data pieces. The `parallel` feature, leveraging Rust `rayon` data-parallelism framework, provides significant speedups for both encoding and recoding. However, decoding remains a much slower operation, with its performance significantly diminishing as the data is split into a greater number of pieces, and currently does **not** implement a parallel decoding algorithm.

#### Full RLNC Encoder

```bash
# Encoding without `rayon` data-parallelism

Timer precision: 19 ns
full_rlnc_encoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ encode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    453.3 µs      │ 652.4 µs      │ 518.5 µs      │ 516.8 µs      │ 100     │ 100
   │                                          2.288 GiB/s   │ 1.59 GiB/s    │ 2.001 GiB/s   │ 2.007 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            64.03 KiB   │ 64.03 KiB     │ 64.03 KiB     │ 64.03 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            64.03 KiB   │ 64.03 KiB     │ 64.03 KiB     │ 64.03 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            16 B        │ 16 B          │ 16 B          │ 16 B          │         │
   ├─ 1.00 MB data splitted into 32 pieces    490.6 µs      │ 609.1 µs      │ 523.4 µs      │ 523.4 µs      │ 100     │ 100
   │                                          2.052 GiB/s   │ 1.653 GiB/s   │ 1.923 GiB/s   │ 1.924 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            32.06 KiB   │ 32.06 KiB     │ 32.06 KiB     │ 32.06 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            32.06 KiB   │ 32.06 KiB     │ 32.06 KiB     │ 32.06 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 32 B          │         │
   ├─ 1.00 MB data splitted into 64 pieces    496 µs        │ 575.8 µs      │ 520 µs        │ 522 µs        │ 100     │ 100
   │                                          1.999 GiB/s   │ 1.722 GiB/s   │ 1.907 GiB/s   │ 1.899 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            16.12 KiB   │ 16.12 KiB     │ 16.12 KiB     │ 16.12 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            16.12 KiB   │ 16.12 KiB     │ 16.12 KiB     │ 16.12 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 64 B          │         │
   ├─ 1.00 MB data splitted into 128 pieces   504.1 µs      │ 562.1 µs      │ 523.1 µs      │ 524.7 µs      │ 100     │ 100
   │                                          1.952 GiB/s   │ 1.751 GiB/s   │ 1.881 GiB/s   │ 1.875 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            8.25 KiB    │ 8.25 KiB      │ 8.25 KiB      │ 8.25 KiB      │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            8.25 KiB    │ 8.25 KiB      │ 8.25 KiB      │ 8.25 KiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 128 B         │         │
   ├─ 1.00 MB data splitted into 256 pieces   506 µs        │ 590.4 µs      │ 523.5 µs      │ 529 µs        │ 100     │ 100
   │                                          1.938 GiB/s   │ 1.661 GiB/s   │ 1.873 GiB/s   │ 1.853 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            4.5 KiB     │ 4.5 KiB       │ 4.5 KiB       │ 4.5 KiB       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            4.5 KiB     │ 4.5 KiB       │ 4.5 KiB       │ 4.5 KiB       │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            256 B       │ 256 B         │ 256 B         │ 256 B         │         │
   ├─ 16.00 MB data splitted into 16 pieces   8.122 ms      │ 9.107 ms      │ 8.794 ms      │ 8.752 ms      │ 100     │ 100
   │                                          2.043 GiB/s   │ 1.822 GiB/s   │ 1.887 GiB/s   │ 1.896 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            16 B        │ 16 B          │ 16 B          │ 16 B          │         │
   ├─ 16.00 MB data splitted into 32 pieces   7.865 ms      │ 9.532 ms      │ 8.513 ms      │ 8.565 ms      │ 100     │ 100
   │                                          2.048 GiB/s   │ 1.69 GiB/s    │ 1.892 GiB/s   │ 1.881 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            512 KiB     │ 512 KiB       │ 512 KiB       │ 512 KiB       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            512 KiB     │ 512 KiB       │ 512 KiB       │ 512 KiB       │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 32 B          │         │
   ├─ 16.00 MB data splitted into 64 pieces   8.101 ms      │ 9.894 ms      │ 8.408 ms      │ 8.443 ms      │ 100     │ 100
   │                                          1.958 GiB/s   │ 1.603 GiB/s   │ 1.887 GiB/s   │ 1.879 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            256.1 KiB   │ 256.1 KiB     │ 256.1 KiB     │ 256.1 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            256.1 KiB   │ 256.1 KiB     │ 256.1 KiB     │ 256.1 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 64 B          │         │
   ├─ 16.00 MB data splitted into 128 pieces  8.099 ms      │ 8.868 ms      │ 8.384 ms      │ 8.409 ms      │ 100     │ 100
   │                                          1.944 GiB/s   │ 1.775 GiB/s   │ 1.878 GiB/s   │ 1.872 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            128.2 KiB   │ 128.2 KiB     │ 128.2 KiB     │ 128.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            128.2 KiB   │ 128.2 KiB     │ 128.2 KiB     │ 128.2 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 128 B         │         │
   ├─ 16.00 MB data splitted into 256 pieces  7.948 ms      │ 8.973 ms      │ 8.399 ms      │ 8.421 ms      │ 100     │ 100
   │                                          1.973 GiB/s   │ 1.748 GiB/s   │ 1.867 GiB/s   │ 1.862 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            64.5 KiB    │ 64.5 KiB      │ 64.5 KiB      │ 64.5 KiB      │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            64.5 KiB    │ 64.5 KiB      │ 64.5 KiB      │ 64.5 KiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            256 B       │ 256 B         │ 256 B         │ 256 B         │         │
   ├─ 32.00 MB data splitted into 16 pieces   14.82 ms      │ 18.81 ms      │ 16.92 ms      │ 16.9 ms       │ 100     │ 100
   │                                          2.239 GiB/s   │ 1.764 GiB/s   │ 1.961 GiB/s   │ 1.964 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            16 B        │ 16 B          │ 16 B          │ 16 B          │         │
   ├─ 32.00 MB data splitted into 32 pieces   16.12 ms      │ 18.7 ms       │ 16.88 ms      │ 16.89 ms      │ 100     │ 100
   │                                          1.998 GiB/s   │ 1.722 GiB/s   │ 1.909 GiB/s   │ 1.907 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 32 B          │         │
   ├─ 32.00 MB data splitted into 64 pieces   16.16 ms      │ 17.39 ms      │ 16.83 ms      │ 16.82 ms      │ 100     │ 100
   │                                          1.963 GiB/s   │ 1.824 GiB/s   │ 1.885 GiB/s   │ 1.886 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 64 B          │         │
   ├─ 32.00 MB data splitted into 128 pieces  16.44 ms      │ 18.57 ms      │ 16.82 ms      │ 16.88 ms      │ 100     │ 100
   │                                          1.914 GiB/s   │ 1.695 GiB/s   │ 1.871 GiB/s   │ 1.864 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 128 B         │         │
   ╰─ 32.00 MB data splitted into 256 pieces  16.53 ms      │ 18.82 ms      │ 16.77 ms      │ 16.86 ms      │ 100     │ 100
                                              1.897 GiB/s   │ 1.666 GiB/s   │ 1.869 GiB/s   │ 1.86 GiB/s    │         │
                                              max alloc:    │               │               │               │         │
                                                2           │ 2             │ 2             │ 2             │         │
                                                128.5 KiB   │ 128.5 KiB     │ 128.5 KiB     │ 128.5 KiB     │         │
                                              alloc:        │               │               │               │         │
                                                2           │ 2             │ 2             │ 2             │         │
                                                128.5 KiB   │ 128.5 KiB     │ 128.5 KiB     │ 128.5 KiB     │         │
                                              dealloc:      │               │               │               │         │
                                                1           │ 1             │ 1             │ 1             │         │
                                                256 B       │ 256 B         │ 256 B         │ 256 B         │         │

# ---------------------------------------------------------------------------------------------------------------------------
# Encoding with `rayon` data-parallelism

Timer precision: 18 ns
full_rlnc_encoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ encode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    145.8 µs      │ 3.104 ms      │ 231.3 µs      │ 280.8 µs      │ 100     │ 100
   │                                          7.112 GiB/s   │ 342.2 MiB/s   │ 4.484 GiB/s   │ 3.693 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 2.68          │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 607.5 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 3.73          │         │
   │                                            64.03 KiB   │ 64.03 KiB     │ 64.03 KiB     │ 64.6 KiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3.07          │         │
   │                                            128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                          grow:         │               │               │               │         │
   │                                            0           │ 0             │ 0             │ 0.02          │         │
   │                                            0 B         │ 0 B           │ 0 B           │ 2.56 B        │         │
   ├─ 1.00 MB data splitted into 32 pieces    160.2 µs      │ 1.091 ms      │ 202.6 µs      │ 216.7 µs      │ 100     │ 100
   │                                          6.284 GiB/s   │ 944.9 MiB/s   │ 4.97 GiB/s    │ 4.646 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.02          │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 94.4 B        │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.02          │         │
   │                                            32.06 KiB   │ 32.06 KiB     │ 32.06 KiB     │ 32.09 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            64.03 KiB   │ 64.03 KiB     │ 64.03 KiB     │ 64.03 KiB     │         │
   ├─ 1.00 MB data splitted into 64 pieces    115.7 µs      │ 313.1 µs      │ 162.4 µs      │ 173.2 µs      │ 100     │ 100
   │                                          8.567 GiB/s   │ 3.167 GiB/s   │ 6.106 GiB/s   │ 5.726 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.01          │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 143.2 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.01          │         │
   │                                            16.12 KiB   │ 16.12 KiB     │ 16.12 KiB     │ 16.14 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            32.06 KiB   │ 32.06 KiB     │ 32.06 KiB     │ 32.06 KiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces   106 µs        │ 320.6 µs      │ 157.7 µs      │ 162.8 µs      │ 100     │ 100
   │                                          9.282 GiB/s   │ 3.069 GiB/s   │ 6.24 GiB/s    │ 6.045 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.02          │         │
   │                                            256 B       │ 256 B         │ 256 B         │ 286.4 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.02          │         │
   │                                            8.25 KiB    │ 8.25 KiB      │ 8.25 KiB      │ 8.28 KiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            16.12 KiB   │ 16.12 KiB     │ 16.12 KiB     │ 16.12 KiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces   103.5 µs      │ 414 µs        │ 113.5 µs      │ 154.2 µs      │ 100     │ 100
   │                                          9.47 GiB/s    │ 2.369 GiB/s   │ 8.639 GiB/s   │ 6.359 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.01          │         │
   │                                            512 B       │ 512 B         │ 512 B         │ 527.2 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.01          │         │
   │                                            4.5 KiB     │ 4.5 KiB       │ 4.5 KiB       │ 4.515 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            8.251 KiB   │ 8.251 KiB     │ 8.251 KiB     │ 8.251 KiB     │         │
   ├─ 16.00 MB data splitted into 16 pieces   3.207 ms      │ 6.052 ms      │ 3.665 ms      │ 3.682 ms      │ 100     │ 100
   │                                          5.175 GiB/s   │ 2.742 GiB/s   │ 4.529 GiB/s   │ 4.507 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.02          │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 62.4 B        │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.02          │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   ├─ 16.00 MB data splitted into 32 pieces   2.413 ms      │ 3.345 ms      │ 2.77 ms       │ 2.783 ms      │ 100     │ 100
   │                                          6.675 GiB/s   │ 4.815 GiB/s   │ 5.816 GiB/s   │ 5.788 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.02          │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 94.4 B        │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.02          │         │
   │                                            512 KiB     │ 512 KiB       │ 512 KiB       │ 512 KiB       │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 16.00 MB data splitted into 64 pieces   2.046 ms      │ 2.72 ms       │ 2.446 ms      │ 2.419 ms      │ 100     │ 100
   │                                          7.754 GiB/s   │ 5.833 GiB/s   │ 6.485 GiB/s   │ 6.559 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.01          │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 143.2 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.01          │         │
   │                                            256.1 KiB   │ 256.1 KiB     │ 256.1 KiB     │ 256.1 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512 KiB     │ 512 KiB       │ 512 KiB       │ 512 KiB       │         │
   ├─ 16.00 MB data splitted into 128 pieces  1.748 ms      │ 2.497 ms      │ 2.167 ms      │ 2.144 ms      │ 100     │ 100
   │                                          9.005 GiB/s   │ 6.304 GiB/s   │ 7.263 GiB/s   │ 7.343 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.02          │         │
   │                                            256 B       │ 256 B         │ 256 B         │ 286.4 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.02          │         │
   │                                            128.2 KiB   │ 128.2 KiB     │ 128.2 KiB     │ 128.2 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            256.1 KiB   │ 256.1 KiB     │ 256.1 KiB     │ 256.1 KiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces  1.571 ms      │ 2.203 ms      │ 2.013 ms      │ 1.989 ms      │ 100     │ 100
   │                                          9.982 GiB/s   │ 7.119 GiB/s   │ 7.789 GiB/s   │ 7.886 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.01          │         │
   │                                            512 B       │ 512 B         │ 512 B         │ 527.2 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.01          │         │
   │                                            64.5 KiB    │ 64.5 KiB      │ 64.5 KiB      │ 64.51 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128.2 KiB   │ 128.2 KiB     │ 128.2 KiB     │ 128.2 KiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces   7.871 ms      │ 9.819 ms      │ 8.457 ms      │ 8.544 ms      │ 100     │ 100
   │                                          4.218 GiB/s   │ 3.381 GiB/s   │ 3.925 GiB/s   │ 3.885 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.02          │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 62.4 B        │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.02          │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   ├─ 32.00 MB data splitted into 32 pieces   6.513 ms      │ 8.753 ms      │ 7.068 ms      │ 7.118 ms      │ 100     │ 100
   │                                          4.947 GiB/s   │ 3.681 GiB/s   │ 4.558 GiB/s   │ 4.526 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.02          │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 94.4 B        │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.02          │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   ├─ 32.00 MB data splitted into 64 pieces   4.633 ms      │ 6.025 ms      │ 5.533 ms      │ 5.5 ms        │ 100     │ 100
   │                                          6.849 GiB/s   │ 5.267 GiB/s   │ 5.735 GiB/s   │ 5.77 GiB/s    │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.01          │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 143.2 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.01          │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 32.00 MB data splitted into 128 pieces  3.873 ms      │ 5.375 ms      │ 4.667 ms      │ 4.626 ms      │ 100     │ 100
   │                                          8.131 GiB/s   │ 5.859 GiB/s   │ 6.747 GiB/s   │ 6.807 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1.02          │         │
   │                                            256 B       │ 256 B         │ 256 B         │ 286.4 B       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2.02          │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   ╰─ 32.00 MB data splitted into 256 pieces  3.241 ms      │ 4.404 ms      │ 4.084 ms      │ 3.999 ms      │ 100     │ 100
                                              9.678 GiB/s   │ 7.122 GiB/s   │ 7.68 GiB/s    │ 7.843 GiB/s   │         │
                                              max alloc:    │               │               │               │         │
                                                1           │ 1             │ 1             │ 1.01          │         │
                                                512 B       │ 512 B         │ 512 B         │ 527.2 B       │         │
                                              alloc:        │               │               │               │         │
                                                2           │ 2             │ 2             │ 2.01          │         │
                                                128.5 KiB   │ 128.5 KiB     │ 128.5 KiB     │ 128.5 KiB     │         │
                                              dealloc:      │               │               │               │         │
                                                3           │ 3             │ 3             │ 3             │         │
                                                256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
```

#### Full RLNC Recoder

```bash
# Recoding without `rayon` data-parallelism

Timer precision: 19 ns
full_rlnc_recoder                                                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ recode                                                                             │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces, recoding with 8 pieces      210.5 µs      │ 282.8 µs      │ 245.4 µs      │ 250.5 µs      │ 100     │ 100
   │                                                                    2.609 GiB/s   │ 1.942 GiB/s   │ 2.238 GiB/s   │ 2.192 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      64.03 KiB   │ 64.03 KiB     │ 64.03 KiB     │ 64.03 KiB     │         │
   ├─ 1.00 MB data splitted into 32 pieces, recoding with 16 pieces     239.4 µs      │ 302.1 µs      │ 273.9 µs      │ 270.1 µs      │ 100     │ 100
   │                                                                    2.168 GiB/s   │ 1.719 GiB/s   │ 1.895 GiB/s   │ 1.922 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.09 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.09 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      32.06 KiB   │ 32.06 KiB     │ 32.06 KiB     │ 32.06 KiB     │         │
   ├─ 1.00 MB data splitted into 64 pieces, recoding with 32 pieces     254.2 µs      │ 309.5 µs      │ 268.2 µs      │ 268.4 µs      │ 100     │ 100
   │                                                                    1.988 GiB/s   │ 1.633 GiB/s   │ 1.884 GiB/s   │ 1.883 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.18 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.18 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      16.12 KiB   │ 16.12 KiB     │ 16.12 KiB     │ 16.12 KiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces, recoding with 64 pieces    258 µs        │ 302.1 µs      │ 267.9 µs      │ 267.7 µs      │ 100     │ 100
   │                                                                    1.952 GiB/s   │ 1.667 GiB/s   │ 1.879 GiB/s   │ 1.881 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.37 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.37 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      8.25 KiB    │ 8.25 KiB      │ 8.25 KiB      │ 8.25 KiB      │         │
   ├─ 1.00 MB data splitted into 256 pieces, recoding with 128 pieces   274.7 µs      │ 315 µs        │ 286.7 µs      │ 288 µs        │ 100     │ 100
   │                                                                    1.903 GiB/s   │ 1.66 GiB/s    │ 1.824 GiB/s   │ 1.815 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.751 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.751 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      4.5 KiB     │ 4.5 KiB       │ 4.5 KiB       │ 4.5 KiB       │         │
   ├─ 16.00 MB data splitted into 16 pieces, recoding with 8 pieces     4.059 ms      │ 4.446 ms      │ 4.194 ms      │ 4.198 ms      │ 100     │ 100
   │                                                                    2.165 GiB/s   │ 1.976 GiB/s   │ 2.095 GiB/s   │ 2.093 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 16.00 MB data splitted into 32 pieces, recoding with 16 pieces    3.711 ms      │ 4.561 ms      │ 4.191 ms      │ 4.184 ms      │ 100     │ 100
   │                                                                    2.236 GiB/s   │ 1.819 GiB/s   │ 1.98 GiB/s    │ 1.983 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      512 KiB     │ 512 KiB       │ 512 KiB       │ 512 KiB       │         │
   ├─ 16.00 MB data splitted into 64 pieces, recoding with 32 pieces    4.02 ms       │ 4.577 ms      │ 4.244 ms      │ 4.25 ms       │ 100     │ 100
   │                                                                    2.004 GiB/s   │ 1.76 GiB/s    │ 1.898 GiB/s   │ 1.895 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      256.1 KiB   │ 256.1 KiB     │ 256.1 KiB     │ 256.1 KiB     │         │
   ├─ 16.00 MB data splitted into 128 pieces, recoding with 64 pieces   4.06 ms       │ 4.496 ms      │ 4.234 ms      │ 4.253 ms      │ 100     │ 100
   │                                                                    1.955 GiB/s   │ 1.766 GiB/s   │ 1.875 GiB/s   │ 1.867 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.3 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      128.2 KiB   │ 128.2 KiB     │ 128.2 KiB     │ 128.2 KiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces, recoding with 128 pieces  3.981 ms      │ 4.527 ms      │ 4.226 ms      │ 4.233 ms      │ 100     │ 100
   │                                                                    1.985 GiB/s   │ 1.745 GiB/s   │ 1.87 GiB/s    │ 1.866 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      64.5 KiB    │ 64.5 KiB      │ 64.5 KiB      │ 64.5 KiB      │         │
   ├─ 32.00 MB data splitted into 16 pieces, recoding with 8 pieces     7.983 ms      │ 9.973 ms      │ 9.176 ms      │ 9.181 ms      │ 100     │ 100
   │                                                                    2.201 GiB/s   │ 1.762 GiB/s   │ 1.915 GiB/s   │ 1.914 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   ├─ 32.00 MB data splitted into 32 pieces, recoding with 16 pieces    7.814 ms      │ 10.06 ms      │ 9 ms          │ 9.002 ms      │ 100     │ 100
   │                                                                    2.124 GiB/s   │ 1.65 GiB/s    │ 1.844 GiB/s   │ 1.844 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 32.00 MB data splitted into 64 pieces, recoding with 32 pieces    8.264 ms      │ 10 ms         │ 8.944 ms      │ 8.977 ms      │ 100     │ 100
   │                                                                    1.949 GiB/s   │ 1.611 GiB/s   │ 1.801 GiB/s   │ 1.795 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   ├─ 32.00 MB data splitted into 128 pieces, recoding with 64 pieces   8.308 ms      │ 9.759 ms      │ 8.78 ms       │ 8.854 ms      │ 100     │ 100
   │                                                                    1.91 GiB/s    │ 1.626 GiB/s   │ 1.808 GiB/s   │ 1.793 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.3 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   ╰─ 32.00 MB data splitted into 256 pieces, recoding with 128 pieces  8.382 ms      │ 9.788 ms      │ 8.801 ms      │ 8.901 ms      │ 100     │ 100
                                                                        1.882 GiB/s   │ 1.611 GiB/s   │ 1.792 GiB/s   │ 1.772 GiB/s   │         │
                                                                        max alloc:    │               │               │               │         │
                                                                          4           │ 4             │ 4             │ 4             │         │
                                                                          256.7 KiB   │ 256.7 KiB     │ 256.7 KiB     │ 256.7 KiB     │         │
                                                                        alloc:        │               │               │               │         │
                                                                          4           │ 4             │ 4             │ 4             │         │
                                                                          256.7 KiB   │ 256.7 KiB     │ 256.7 KiB     │ 256.7 KiB     │         │
                                                                        dealloc:      │               │               │               │         │
                                                                          3           │ 3             │ 3             │ 3             │         │
                                                                          128.5 KiB   │ 128.5 KiB     │ 128.5 KiB     │ 128.5 KiB     │         │

# ---------------------------------------------------------------------------------------------------------------------------
# Recoding with `rayon` data-parallelism

Timer precision: 18 ns
full_rlnc_recoder                                                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ recode                                                                             │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces, recoding with 8 pieces      130.6 µs      │ 446.1 µs      │ 177 µs        │ 189.4 µs      │ 100     │ 100
   │                                                                    4.204 GiB/s   │ 1.231 GiB/s   │ 3.102 GiB/s   │ 2.9 GiB/s     │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.01          │         │
   │                                                                      48 B        │ 48 B          │ 48 B          │ 63.2 B        │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.01          │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      192 KiB     │ 192 KiB       │ 192 KiB       │ 192 KiB       │         │
   ├─ 1.00 MB data splitted into 32 pieces, recoding with 16 pieces     84.81 µs      │ 515.4 µs      │ 112.6 µs      │ 124.1 µs      │ 100     │ 100
   │                                                                    6.122 GiB/s   │ 1.007 GiB/s   │ 4.61 GiB/s    │ 4.184 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      96 B        │ 96 B          │ 96 B          │ 126.4 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.12 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      96.06 KiB   │ 96.06 KiB     │ 96.06 KiB     │ 96.06 KiB     │         │
   ├─ 1.00 MB data splitted into 64 pieces, recoding with 32 pieces     82.57 µs      │ 178.7 µs      │ 104.3 µs      │ 108.5 µs      │ 100     │ 100
   │                                                                    6.122 GiB/s   │ 2.828 GiB/s   │ 4.842 GiB/s   │ 4.658 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.01          │         │
   │                                                                      192 B       │ 192 B         │ 192 B         │ 207.2 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.01          │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.2 KiB      │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      48.12 KiB   │ 48.12 KiB     │ 48.12 KiB     │ 48.12 KiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces, recoding with 64 pieces    87.09 µs      │ 617.3 µs      │ 100.7 µs      │ 115.5 µs      │ 100     │ 100
   │                                                                    5.783 GiB/s   │ 835.5 MiB/s   │ 4.999 GiB/s   │ 4.358 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      384 B       │ 384 B         │ 384 B         │ 414.4 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.4 KiB      │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      24.25 KiB   │ 24.25 KiB     │ 24.25 KiB     │ 24.25 KiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces, recoding with 128 pieces   113.5 µs      │ 225 µs        │ 132.1 µs      │ 143.6 µs      │ 100     │ 100
   │                                                                    4.605 GiB/s   │ 2.323 GiB/s   │ 3.958 GiB/s   │ 3.64 GiB/s    │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.01          │         │
   │                                                                      768 B       │ 768 B         │ 768 B         │ 783.2 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.01          │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.766 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      12.5 KiB    │ 12.5 KiB      │ 12.5 KiB      │ 12.5 KiB      │         │
   ├─ 16.00 MB data splitted into 16 pieces, recoding with 8 pieces     2.281 ms      │ 2.962 ms      │ 2.522 ms      │ 2.547 ms      │ 100     │ 100
   │                                                                    3.852 GiB/s   │ 2.966 GiB/s   │ 3.483 GiB/s   │ 3.45 GiB/s    │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.01          │         │
   │                                                                      48 B        │ 48 B          │ 48 B          │ 63.2 B        │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.01          │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      3 MiB       │ 3 MiB         │ 3 MiB         │ 3 MiB         │         │
   ├─ 16.00 MB data splitted into 32 pieces, recoding with 16 pieces    1.46 ms       │ 2.492 ms      │ 1.679 ms      │ 1.686 ms      │ 100     │ 100
   │                                                                    5.683 GiB/s   │ 3.33 GiB/s    │ 4.941 GiB/s   │ 4.922 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      96 B        │ 96 B          │ 96 B          │ 126.4 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      1.5 MiB     │ 1.5 MiB       │ 1.5 MiB       │ 1.5 MiB       │         │
   ├─ 16.00 MB data splitted into 64 pieces, recoding with 32 pieces    1.245 ms      │ 1.824 ms      │ 1.516 ms      │ 1.521 ms      │ 100     │ 100
   │                                                                    6.472 GiB/s   │ 4.416 GiB/s   │ 5.313 GiB/s   │ 5.297 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      192 B       │ 192 B         │ 192 B         │ 222.4 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.2 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      768.1 KiB   │ 768.1 KiB     │ 768.1 KiB     │ 768.1 KiB     │         │
   ├─ 16.00 MB data splitted into 128 pieces, recoding with 64 pieces   1.174 ms      │ 1.719 ms      │ 1.377 ms      │ 1.394 ms      │ 100     │ 100
   │                                                                    6.764 GiB/s   │ 4.619 GiB/s   │ 5.765 GiB/s   │ 5.693 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      384 B       │ 384 B         │ 384 B         │ 414.4 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.4 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      384.2 KiB   │ 384.2 KiB     │ 384.2 KiB     │ 384.2 KiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces, recoding with 128 pieces  1.071 ms      │ 1.942 ms      │ 1.3 ms        │ 1.309 ms      │ 100     │ 100
   │                                                                    7.378 GiB/s   │ 4.07 GiB/s    │ 6.079 GiB/s   │ 6.036 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.01          │         │
   │                                                                      768 B       │ 768 B         │ 768 B         │ 783.2 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.01          │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      192.5 KiB   │ 192.5 KiB     │ 192.5 KiB     │ 192.5 KiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces, recoding with 8 pieces     4.493 ms      │ 5.58 ms       │ 4.981 ms      │ 5.002 ms      │ 100     │ 100
   │                                                                    3.912 GiB/s   │ 3.15 GiB/s    │ 3.528 GiB/s   │ 3.513 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      48 B        │ 48 B          │ 48 B          │ 78.4 B        │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      6 MiB       │ 6 MiB         │ 6 MiB         │ 6 MiB         │         │
   ├─ 32.00 MB data splitted into 32 pieces, recoding with 16 pieces    3.302 ms      │ 4.485 ms      │ 3.559 ms      │ 3.595 ms      │ 100     │ 100
   │                                                                    5.026 GiB/s   │ 3.701 GiB/s   │ 4.663 GiB/s   │ 4.617 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      96 B        │ 96 B          │ 96 B          │ 126.4 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      3 MiB       │ 3 MiB         │ 3 MiB         │ 3 MiB         │         │
   ├─ 32.00 MB data splitted into 64 pieces, recoding with 32 pieces    2.557 ms      │ 3.5 ms        │ 2.796 ms      │ 2.808 ms      │ 100     │ 100
   │                                                                    6.301 GiB/s   │ 4.604 GiB/s   │ 5.762 GiB/s   │ 5.737 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      192 B       │ 192 B         │ 192 B         │ 222.4 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      1.5 MiB     │ 1.5 MiB       │ 1.5 MiB       │ 1.5 MiB       │         │
   ├─ 32.00 MB data splitted into 128 pieces, recoding with 64 pieces   2.219 ms      │ 3.033 ms      │ 2.457 ms      │ 2.493 ms      │ 100     │ 100
   │                                                                    7.154 GiB/s   │ 5.234 GiB/s   │ 6.461 GiB/s   │ 6.367 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      2           │ 2             │ 2             │ 2.02          │         │
   │                                                                      384 B       │ 384 B         │ 384 B         │ 414.4 B       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4.02          │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.4 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      5           │ 5             │ 5             │ 5             │         │
   │                                                                      768.2 KiB   │ 768.2 KiB     │ 768.2 KiB     │ 768.2 KiB     │         │
   ╰─ 32.00 MB data splitted into 256 pieces, recoding with 128 pieces  2.004 ms      │ 3.006 ms      │ 2.274 ms      │ 2.286 ms      │ 100     │ 100
                                                                        7.873 GiB/s   │ 5.247 GiB/s   │ 6.937 GiB/s   │ 6.899 GiB/s   │         │
                                                                        max alloc:    │               │               │               │         │
                                                                          2           │ 2             │ 2             │ 2.01          │         │
                                                                          768 B       │ 768 B         │ 768 B         │ 783.2 B       │         │
                                                                        alloc:        │               │               │               │         │
                                                                          4           │ 4             │ 4             │ 4.01          │         │
                                                                          256.7 KiB   │ 256.7 KiB     │ 256.7 KiB     │ 256.7 KiB     │         │
                                                                        dealloc:      │               │               │               │         │
                                                                          5           │ 5             │ 5             │ 5             │         │
                                                                          384.5 KiB   │ 384.5 KiB     │ 384.5 KiB     │ 384.5 KiB     │         │
```

#### Full RLNC Decoder

```bash
Timer precision: 18 ns
full_rlnc_decoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ decode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    13.47 ms      │ 17.34 ms      │ 14.13 ms      │ 14.22 ms      │ 100     │ 100
   │                                          74.24 MiB/s   │ 57.67 MiB/s   │ 70.74 MiB/s   │ 70.31 MiB/s   │         │
   ├─ 1.00 MB data splitted into 32 pieces    27.63 ms      │ 31.7 ms       │ 28.76 ms      │ 28.87 ms      │ 100     │ 100
   │                                          36.21 MiB/s   │ 31.57 MiB/s   │ 34.79 MiB/s   │ 34.66 MiB/s   │         │
   ├─ 1.00 MB data splitted into 64 pieces    56.35 ms      │ 62.67 ms      │ 57.83 ms      │ 58.2 ms       │ 100     │ 100
   │                                          17.81 MiB/s   │ 16.01 MiB/s   │ 17.35 MiB/s   │ 17.24 MiB/s   │         │
   ├─ 1.00 MB data splitted into 128 pieces   113.6 ms      │ 123 ms        │ 116.9 ms      │ 117.3 ms      │ 100     │ 100
   │                                          8.937 MiB/s   │ 8.257 MiB/s   │ 8.683 MiB/s   │ 8.653 MiB/s   │         │
   ├─ 1.00 MB data splitted into 256 pieces   237.1 ms      │ 250.5 ms      │ 241.9 ms      │ 241.9 ms      │ 100     │ 100
   │                                          4.482 MiB/s   │ 4.241 MiB/s   │ 4.392 MiB/s   │ 4.392 MiB/s   │         │
   ├─ 16.00 MB data splitted into 16 pieces   227 ms        │ 235.6 ms      │ 230.5 ms      │ 230.7 ms      │ 100     │ 100
   │                                          70.45 MiB/s   │ 67.88 MiB/s   │ 69.39 MiB/s   │ 69.34 MiB/s   │         │
   ├─ 16.00 MB data splitted into 32 pieces   424.6 ms      │ 470.8 ms      │ 449.8 ms      │ 453.7 ms      │ 100     │ 100
   │                                          37.67 MiB/s   │ 33.98 MiB/s   │ 35.57 MiB/s   │ 35.26 MiB/s   │         │
   ├─ 16.00 MB data splitted into 64 pieces   884.5 ms      │ 899 ms        │ 894.1 ms      │ 893.1 ms      │ 100     │ 100
   │                                          18.09 MiB/s   │ 17.79 MiB/s   │ 17.89 MiB/s   │ 17.91 MiB/s   │         │
   ├─ 16.00 MB data splitted into 128 pieces  1.762 s       │ 1.812 s       │ 1.78 s        │ 1.78 s        │ 57      │ 57
   │                                          9.086 MiB/s   │ 8.834 MiB/s   │ 8.992 MiB/s   │ 8.993 MiB/s   │         │
   ├─ 16.00 MB data splitted into 256 pieces  3.553 s       │ 3.673 s       │ 3.611 s       │ 3.602 s       │ 28      │ 28
   │                                          4.519 MiB/s   │ 4.372 MiB/s   │ 4.447 MiB/s   │ 4.458 MiB/s   │         │
   ├─ 32.00 MB data splitted into 16 pieces   446.2 ms      │ 458.8 ms      │ 452.9 ms      │ 452.7 ms      │ 100     │ 100
   │                                          71.7 MiB/s    │ 69.74 MiB/s   │ 70.64 MiB/s   │ 70.67 MiB/s   │         │
   ├─ 32.00 MB data splitted into 32 pieces   892.4 ms      │ 975.2 ms      │ 906.6 ms      │ 907.7 ms      │ 100     │ 100
   │                                          35.85 MiB/s   │ 32.81 MiB/s   │ 35.29 MiB/s   │ 35.25 MiB/s   │         │
   ├─ 32.00 MB data splitted into 64 pieces   1.765 s       │ 1.912 s       │ 1.881 s       │ 1.859 s       │ 54      │ 54
   │                                          18.12 MiB/s   │ 16.73 MiB/s   │ 17.01 MiB/s   │ 17.21 MiB/s   │         │
   ├─ 32.00 MB data splitted into 128 pieces  3.563 s       │ 3.755 s       │ 3.727 s       │ 3.698 s       │ 28      │ 28
   │                                          8.983 MiB/s   │ 8.524 MiB/s   │ 8.589 MiB/s   │ 8.657 MiB/s   │         │
   ╰─ 32.00 MB data splitted into 256 pieces  7.021 s       │ 7.531 s       │ 7.117 s       │ 7.229 s       │ 14      │ 14
                                              4.566 MiB/s   │ 4.257 MiB/s   │ 4.504 MiB/s   │ 4.434 MiB/s   │         │
```

## Usage

To use `rlnc` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rlnc = "=0.4.0"                                      # Use the latest version available on crates.io.
# or
rlnc = { version = "=0.4.0", features = "parallel" } # Uses `rayon`-based data-parallelism for much faster encoding/ recoding.

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

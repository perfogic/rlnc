# rlnc
Random Linear Network Coding

## Introduction
`rlnc` is a Rust library crate that implements Random Linear Network Coding (RLNC) over $GF(2^8)$ with primitive polynomial $x^8 + x^4 + x^3 + x^2 + 1$. This library provides functionalities for erasure-coding data, reconstructing original data from coded pieces, and recoding existing coded pieces to new erasure-coded pieces, without ever decoding it back to original data.

For a quick understanding of RLNC, have a look at my blog post @ https://itzmeanjan.in/pages/rlnc-in-depth.html.

Random Linear Network Coding (RLNC) excels in highly dynamic and lossy environments like multicast, peer-to-peer networks, and distributed storage, due to interesting properties such as encoding with random-sampled coefficients, any `k` out of `n` coded-pieces are sufficient to recover and recoding new pieces with existing erasure-coded pieces. Unlike Reed-Solomon, which requires specific symbols for deterministic recovery, RLNC allows decoding from *any* set of linearly independent packets. Compared to Fountain Codes, RLNC offers robust algebraic linearity with coding vector overhead, whereas Fountain codes prioritize very low decoding complexity and indefinite symbol generation, often for large-scale broadcasts.

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
For ensuring functional correctness of RLNC operations, the library includes a comprehensive test suite. Run all the tests by running following commands.

```bash
# Testing on host, first with `default` feature, then with `parallel` feature enabled.
make test

# Testing on web assembly target, using `wasmtime`.
rustup target add wasm32-wasip1
cargo install wasmtime-cli --locked
make test-wasm
```

```bash
running 14 tests
test full::decoder::tests::test_decoder_decode_invalid_piece_length ... ok
test full::decoder::tests::test_decoder_new_invalid_inputs ... ok
test full::encoder::tests::test_encoder_code_with_coding_vector_invalid_inputs ... ok
test full::decoder::tests::test_decoder_getters ... ok
test full::encoder::tests::test_encoder_getters ... ok
test full::encoder::tests::test_encoder_without_padding_invalid_data ... ok
test full::encoder::tests::test_encoder_new_invalid_inputs ... ok
test full::recoder::tests::test_recoder_getters ... ok
test full::recoder::tests::test_recoder_new_invalid_inputs ... ok
test common::gf256::test::prop_test_gf256_operations ... ok
test full::tests::prop_test_rlnc_encoder_decoder ... ok
test full::decoder_matrix::test::prop_test_rref_is_idempotent ... ok
test full::tests::prop_test_rlnc_encoder_recoder_decoder ... ok
test full::tests::prop_test_rlnc_decoding_with_useless_pieces has been running for over 60 seconds
test full::tests::prop_test_rlnc_decoding_with_useless_pieces ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.59s

   Doc-tests rlnc

running 3 tests
test src/common/simd_mul_table.rs - common::simd_mul_table (line 22) ... ignored
test src/common/simd_mul_table.rs - common::simd_mul_table (line 8) ... ignored
test src/lib.rs - (line 49) ... ok

test result: ok. 1 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s
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
|| src/common/gf256.rs: 9/11
|| src/common/simd.rs: 42/64
|| src/full/decoder.rs: 29/34
|| src/full/decoder_matrix.rs: 50/55
|| src/full/encoder.rs: 28/28
|| src/full/recoder.rs: 30/36
||
82.10% coverage, 188/229 lines covered
```

This will create an HTML coverage report at `tarpaulin-report.html` that you can open in your web browser to view detailed line-by-line coverage information for all source files.

> [!NOTE]
> There is a help menu, which introduces you to all available commands; just run `$ make` from the root directory of this project.

## Benchmarking
Performance benchmarks for several input configurations are included to evaluate the efficiency of this RLNC implementation.

To run the benchmarks, execute the following command from the root of the project:

```bash
make bench # First with `default` feature, then with `parallel` feature enabled.
```

> [!WARNING]
> When benchmarking make sure you've disabled CPU frequency scaling, otherwise numbers you see can be misleading. I find https://github.com/google/benchmark/blob/b40db869/docs/reducing_variance.md helpful.

### On 12th Gen Intel(R) Core(TM) i7-1260P

Running benchmarks on `Linux 6.14.0-24-generic x86_64`, compiled with `rustc 1.88.0 (6b00bc388 2025-06-23)`.

Component | With `default` feature | With `parallel` feature, using rayon-based data-parallelism | Impact of number of pieces on performance
--- | --- | --- | ---
Full RLNC Encoder | Throughput of 6.5 GiB/s to 31.1 GiB/s | Throughput of 3.4 GiB/s to 9.7 GiB/s | The number of pieces original data got split into has a **minimal** impact on the encoding speed.
Full RLNC Recoder | Throughput of 9.0 GiB/s to 32.1 GiB/s | Throughput of 2.9 GiB/s to 7.9 GiB/s | Similar to the encoder, the recoder's performance remains largely consistent regardless of how many pieces the original data is split into.
Full RLNC Decoder | Throughput of 67 MiB/s to 1.67 GiB/s | **Doesn't yet implement a parallel decoding mode** | As the number of pieces increases, the decoding time increases substantially, leading to a considerable drop in throughput. This indicates that decoding is the most computationally intensive part of the full RLNC scheme, and its performance is inversely proportional to the number of pieces.

In summary, the full RLNC implementation demonstrates excellent encoding and recoding speeds, consistently achieving GiB/s throughputs with minimal sensitivity to the number of data pieces. The `parallel` feature, leveraging Rust `rayon` data-parallelism framework, also provides good performance for both encoding and recoding. Whether you want to use that feature, completely depends on your usecase. However, decoding remains a much slower operation, with its performance significantly diminishing as the data is split into a greater number of pieces, and currently does **not** implement a parallel decoding algorithm.

#### Full RLNC Encoder

```bash
# Encoding without `rayon` data-parallelism, instead using AVX2-powered SIMD vector x scalar multiplication

Timer precision: 22 ns
full_rlnc_encoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ encode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    55.4 µs       │ 122.2 µs      │ 59.28 µs      │ 64.83 µs      │ 100     │ 100
   │                                          18.72 GiB/s   │ 8.486 GiB/s   │ 17.5 GiB/s    │ 16 GiB/s      │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            64.03 KiB   │ 64.03 KiB     │ 64.03 KiB     │ 64.03 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            64.03 KiB   │ 64.03 KiB     │ 64.03 KiB     │ 64.03 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            16 B        │ 16 B          │ 16 B          │ 16 B          │         │
   ├─ 1.00 MB data splitted into 32 pieces    38.97 µs      │ 155.3 µs      │ 42.76 µs      │ 45.32 µs      │ 100     │ 100
   │                                          25.84 GiB/s   │ 6.482 GiB/s   │ 23.54 GiB/s   │ 22.22 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            32.06 KiB   │ 32.06 KiB     │ 32.06 KiB     │ 32.06 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            32.06 KiB   │ 32.06 KiB     │ 32.06 KiB     │ 32.06 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 32 B          │         │
   ├─ 1.00 MB data splitted into 64 pieces    31.87 µs      │ 82.83 µs      │ 33.38 µs      │ 34.86 µs      │ 100     │ 100
   │                                          31.11 GiB/s   │ 11.97 GiB/s   │ 29.71 GiB/s   │ 28.45 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            16.12 KiB   │ 16.12 KiB     │ 16.12 KiB     │ 16.12 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            16.12 KiB   │ 16.12 KiB     │ 16.12 KiB     │ 16.12 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 64 B          │         │
   ├─ 1.00 MB data splitted into 128 pieces   35.18 µs      │ 50.14 µs      │ 36.21 µs      │ 36.45 µs      │ 100     │ 100
   │                                          27.98 GiB/s   │ 19.63 GiB/s   │ 27.18 GiB/s   │ 27 GiB/s      │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            8.25 KiB    │ 8.25 KiB      │ 8.25 KiB      │ 8.25 KiB      │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            8.25 KiB    │ 8.25 KiB      │ 8.25 KiB      │ 8.25 KiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 128 B         │         │
   ├─ 1.00 MB data splitted into 256 pieces   33.36 µs      │ 51.54 µs      │ 39.89 µs      │ 39.63 µs      │ 100     │ 100
   │                                          29.39 GiB/s   │ 19.02 GiB/s   │ 24.58 GiB/s   │ 24.74 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            4.5 KiB     │ 4.5 KiB       │ 4.5 KiB       │ 4.5 KiB       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            4.5 KiB     │ 4.5 KiB       │ 4.5 KiB       │ 4.5 KiB       │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            256 B       │ 256 B         │ 256 B         │ 256 B         │         │
   ├─ 16.00 MB data splitted into 16 pieces   1.06 ms       │ 1.569 ms      │ 1.184 ms      │ 1.196 ms      │ 100     │ 100
   │                                          15.65 GiB/s   │ 10.57 GiB/s   │ 14.01 GiB/s   │ 13.88 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            16 B        │ 16 B          │ 16 B          │ 16 B          │         │
   ├─ 16.00 MB data splitted into 32 pieces   939.4 µs      │ 1.688 ms      │ 1.017 ms      │ 1.039 ms      │ 100     │ 100
   │                                          17.15 GiB/s   │ 9.542 GiB/s   │ 15.83 GiB/s   │ 15.49 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            512 KiB     │ 512 KiB       │ 512 KiB       │ 512 KiB       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            512 KiB     │ 512 KiB       │ 512 KiB       │ 512 KiB       │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 32 B          │         │
   ├─ 16.00 MB data splitted into 64 pieces   802.9 µs      │ 1.63 ms       │ 1.004 ms      │ 1.032 ms      │ 100     │ 100
   │                                          19.76 GiB/s   │ 9.733 GiB/s   │ 15.79 GiB/s   │ 15.37 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            256.1 KiB   │ 256.1 KiB     │ 256.1 KiB     │ 256.1 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            256.1 KiB   │ 256.1 KiB     │ 256.1 KiB     │ 256.1 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 64 B          │         │
   ├─ 16.00 MB data splitted into 128 pieces  688.3 µs      │ 1.235 ms      │ 725.3 µs      │ 743.4 µs      │ 100     │ 100
   │                                          22.87 GiB/s   │ 12.74 GiB/s   │ 21.7 GiB/s    │ 21.18 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            128.2 KiB   │ 128.2 KiB     │ 128.2 KiB     │ 128.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            128.2 KiB   │ 128.2 KiB     │ 128.2 KiB     │ 128.2 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 128 B         │         │
   ├─ 16.00 MB data splitted into 256 pieces  692.2 µs      │ 1.118 ms      │ 739.4 µs      │ 774.9 µs      │ 100     │ 100
   │                                          22.65 GiB/s   │ 14.03 GiB/s   │ 21.21 GiB/s   │ 20.24 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            64.5 KiB    │ 64.5 KiB      │ 64.5 KiB      │ 64.5 KiB      │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            64.5 KiB    │ 64.5 KiB      │ 64.5 KiB      │ 64.5 KiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            256 B       │ 256 B         │ 256 B         │ 256 B         │         │
   ├─ 32.00 MB data splitted into 16 pieces   1.995 ms      │ 3.003 ms      │ 2.143 ms      │ 2.188 ms      │ 100     │ 100
   │                                          16.63 GiB/s   │ 11.05 GiB/s   │ 15.48 GiB/s   │ 15.17 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            16 B        │ 16 B          │ 16 B          │ 16 B          │         │
   ├─ 32.00 MB data splitted into 32 pieces   1.961 ms      │ 2.821 ms      │ 2.081 ms      │ 2.118 ms      │ 100     │ 100
   │                                          16.42 GiB/s   │ 11.42 GiB/s   │ 15.48 GiB/s   │ 15.21 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            32 B        │ 32 B          │ 32 B          │ 32 B          │         │
   ├─ 32.00 MB data splitted into 64 pieces   1.87 ms       │ 2.043 ms      │ 1.926 ms      │ 1.934 ms      │ 100     │ 100
   │                                          16.97 GiB/s   │ 15.53 GiB/s   │ 16.47 GiB/s   │ 16.4 GiB/s    │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            64 B        │ 64 B          │ 64 B          │ 64 B          │         │
   ├─ 32.00 MB data splitted into 128 pieces  1.86 ms       │ 2.315 ms      │ 1.914 ms      │ 1.937 ms      │ 100     │ 100
   │                                          16.92 GiB/s   │ 13.6 GiB/s    │ 16.44 GiB/s   │ 16.25 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            2           │ 2             │ 2             │ 2             │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            1           │ 1             │ 1             │ 1             │         │
   │                                            128 B       │ 128 B         │ 128 B         │ 128 B         │         │
   ╰─ 32.00 MB data splitted into 256 pieces  1.77 ms       │ 2.671 ms      │ 1.827 ms      │ 1.861 ms      │ 100     │ 100
                                              17.71 GiB/s   │ 11.74 GiB/s   │ 17.16 GiB/s   │ 16.85 GiB/s   │         │
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
# Recoding without `rayon` data-parallelism, instead using AVX2-powered SIMD vector x scalar multiplication

Timer precision: 13 ns
full_rlnc_recoder                                                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ recode                                                                             │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces, recoding with 8 pieces      18.77 µs      │ 32.79 µs      │ 21.37 µs      │ 22 µs         │ 100     │ 100
   │                                                                    29.26 GiB/s   │ 16.75 GiB/s   │ 25.7 GiB/s    │ 24.97 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      64.03 KiB   │ 64.03 KiB     │ 64.03 KiB     │ 64.03 KiB     │         │
   ├─ 1.00 MB data splitted into 32 pieces, recoding with 16 pieces     16.42 µs      │ 20.77 µs      │ 18.81 µs      │ 18.6 µs       │ 100     │ 100
   │                                                                    31.62 GiB/s   │ 24.99 GiB/s   │ 27.6 GiB/s    │ 27.91 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.09 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.09 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      32.06 KiB   │ 32.06 KiB     │ 32.06 KiB     │ 32.06 KiB     │         │
   ├─ 1.00 MB data splitted into 64 pieces, recoding with 32 pieces     15.73 µs      │ 23.95 µs      │ 16.55 µs      │ 16.77 µs      │ 100     │ 100
   │                                                                    32.13 GiB/s   │ 21.1 GiB/s    │ 30.53 GiB/s   │ 30.13 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.18 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.18 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      16.12 KiB   │ 16.12 KiB     │ 16.12 KiB     │ 16.12 KiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces, recoding with 64 pieces    22.8 µs       │ 30.59 µs      │ 23.55 µs      │ 23.76 µs      │ 100     │ 100
   │                                                                    22.09 GiB/s   │ 16.46 GiB/s   │ 21.38 GiB/s   │ 21.19 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.37 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.37 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      8.25 KiB    │ 8.25 KiB      │ 8.25 KiB      │ 8.25 KiB      │         │
   ├─ 1.00 MB data splitted into 256 pieces, recoding with 128 pieces   43.34 µs      │ 57.91 µs      │ 44.87 µs      │ 45.49 µs      │ 100     │ 100
   │                                                                    12.06 GiB/s   │ 9.03 GiB/s    │ 11.65 GiB/s   │ 11.49 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.751 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.751 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      4.5 KiB     │ 4.5 KiB       │ 4.5 KiB       │ 4.5 KiB       │         │
   ├─ 16.00 MB data splitted into 16 pieces, recoding with 8 pieces     563.5 µs      │ 706.9 µs      │ 623.9 µs      │ 626.2 µs      │ 100     │ 100
   │                                                                    15.59 GiB/s   │ 12.43 GiB/s   │ 14.08 GiB/s   │ 14.03 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 16.00 MB data splitted into 32 pieces, recoding with 16 pieces    480.5 µs      │ 644.8 µs      │ 535.3 µs      │ 536.7 µs      │ 100     │ 100
   │                                                                    17.27 GiB/s   │ 12.87 GiB/s   │ 15.5 GiB/s    │ 15.46 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      512 KiB     │ 512 KiB       │ 512 KiB       │ 512 KiB       │         │
   ├─ 16.00 MB data splitted into 64 pieces, recoding with 32 pieces    461.6 µs      │ 692.9 µs      │ 484.9 µs      │ 491.1 µs      │ 100     │ 100
   │                                                                    17.45 GiB/s   │ 11.63 GiB/s   │ 16.61 GiB/s   │ 16.4 GiB/s    │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      256.1 KiB   │ 256.1 KiB     │ 256.1 KiB     │ 256.1 KiB     │         │
   ├─ 16.00 MB data splitted into 128 pieces, recoding with 64 pieces   561.5 µs      │ 770.5 µs      │ 621.5 µs      │ 627.4 µs      │ 100     │ 100
   │                                                                    14.14 GiB/s   │ 10.3 GiB/s    │ 12.77 GiB/s   │ 12.65 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.3 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      128.2 KiB   │ 128.2 KiB     │ 128.2 KiB     │ 128.2 KiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces, recoding with 128 pieces  479.3 µs      │ 826.3 µs      │ 628.6 µs      │ 590.3 µs      │ 100     │ 100
   │                                                                    16.48 GiB/s   │ 9.565 GiB/s   │ 12.57 GiB/s   │ 13.38 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      64.5 KiB    │ 64.5 KiB      │ 64.5 KiB      │ 64.5 KiB      │         │
   ├─ 32.00 MB data splitted into 16 pieces, recoding with 8 pieces     1.253 ms      │ 1.685 ms      │ 1.42 ms       │ 1.426 ms      │ 100     │ 100
   │                                                                    14.02 GiB/s   │ 10.43 GiB/s   │ 12.37 GiB/s   │ 12.32 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   ├─ 32.00 MB data splitted into 32 pieces, recoding with 16 pieces    1.102 ms      │ 1.581 ms      │ 1.249 ms      │ 1.258 ms      │ 100     │ 100
   │                                                                    15.06 GiB/s   │ 10.49 GiB/s   │ 13.28 GiB/s   │ 13.19 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 32.00 MB data splitted into 64 pieces, recoding with 32 pieces    1.014 ms      │ 1.287 ms      │ 1.077 ms      │ 1.085 ms      │ 100     │ 100
   │                                                                    15.89 GiB/s   │ 12.51 GiB/s   │ 14.95 GiB/s   │ 14.85 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   ├─ 32.00 MB data splitted into 128 pieces, recoding with 64 pieces   1 ms          │ 1.248 ms      │ 1.045 ms      │ 1.049 ms      │ 100     │ 100
   │                                                                    15.86 GiB/s   │ 12.72 GiB/s   │ 15.18 GiB/s   │ 15.13 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.3 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      3           │ 3             │ 3             │ 3             │         │
   │                                                                      256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   ╰─ 32.00 MB data splitted into 256 pieces, recoding with 128 pieces  1.037 ms      │ 1.272 ms      │ 1.06 ms       │ 1.069 ms      │ 100     │ 100
                                                                        15.21 GiB/s   │ 12.39 GiB/s   │ 14.87 GiB/s   │ 14.75 GiB/s   │         │
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
# Decoding with AVX2-powered SIMD vector x scalar multiplication

Timer precision: 18 ns
full_rlnc_decoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ decode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    600 µs        │ 1.041 ms      │ 615.2 µs      │ 622.9 µs      │ 100     │ 100
   │                                          1.627 GiB/s   │ 960.8 MiB/s   │ 1.587 GiB/s   │ 1.568 GiB/s   │         │
   ├─ 1.00 MB data splitted into 32 pieces    1.18 ms       │ 1.629 ms      │ 1.201 ms      │ 1.207 ms      │ 100     │ 100
   │                                          847.6 MiB/s   │ 614.1 MiB/s   │ 833 MiB/s     │ 829.2 MiB/s   │         │
   ├─ 1.00 MB data splitted into 64 pieces    2.267 ms      │ 2.404 ms      │ 2.299 ms      │ 2.302 ms      │ 100     │ 100
   │                                          442.7 MiB/s   │ 417.5 MiB/s   │ 436.6 MiB/s   │ 436 MiB/s     │         │
   ├─ 1.00 MB data splitted into 128 pieces   5.296 ms      │ 5.583 ms      │ 5.333 ms      │ 5.338 ms      │ 100     │ 100
   │                                          191.7 MiB/s   │ 181.9 MiB/s   │ 190.4 MiB/s   │ 190.2 MiB/s   │         │
   ├─ 1.00 MB data splitted into 256 pieces   15.27 ms      │ 16.21 ms      │ 15.55 ms      │ 15.55 ms      │ 100     │ 100
   │                                          69.59 MiB/s   │ 65.54 MiB/s   │ 68.33 MiB/s   │ 68.32 MiB/s   │         │
   ├─ 16.00 MB data splitted into 16 pieces   16.4 ms       │ 20.56 ms      │ 16.97 ms      │ 17.08 ms      │ 100     │ 100
   │                                          975.5 MiB/s   │ 778.1 MiB/s   │ 942.4 MiB/s   │ 936.5 MiB/s   │         │
   ├─ 16.00 MB data splitted into 32 pieces   27.24 ms      │ 30.95 ms      │ 28.12 ms      │ 28.26 ms      │ 100     │ 100
   │                                          587.3 MiB/s   │ 516.8 MiB/s   │ 568.9 MiB/s   │ 566 MiB/s     │         │
   ├─ 16.00 MB data splitted into 64 pieces   49.54 ms      │ 62.7 ms       │ 49.74 ms      │ 50.23 ms      │ 100     │ 100
   │                                          323 MiB/s     │ 255.2 MiB/s   │ 321.6 MiB/s   │ 318.5 MiB/s   │         │
   ├─ 16.00 MB data splitted into 128 pieces  98.43 ms      │ 102.1 ms      │ 98.98 ms      │ 99.32 ms      │ 100     │ 100
   │                                          162.6 MiB/s   │ 156.7 MiB/s   │ 161.8 MiB/s   │ 161.2 MiB/s   │         │
   ├─ 16.00 MB data splitted into 256 pieces  201.8 ms      │ 209.3 ms      │ 202.7 ms      │ 203 ms        │ 100     │ 100
   │                                          79.58 MiB/s   │ 76.73 MiB/s   │ 79.21 MiB/s   │ 79.11 MiB/s   │         │
   ├─ 32.00 MB data splitted into 16 pieces   46.33 ms      │ 49.04 ms      │ 46.51 ms      │ 46.7 ms       │ 100     │ 100
   │                                          690.5 MiB/s   │ 652.4 MiB/s   │ 687.8 MiB/s   │ 685.1 MiB/s   │         │
   ├─ 32.00 MB data splitted into 32 pieces   78.74 ms      │ 81.78 ms      │ 79.06 ms      │ 79.29 ms      │ 100     │ 100
   │                                          406.4 MiB/s   │ 391.2 MiB/s   │ 404.7 MiB/s   │ 403.5 MiB/s   │         │
   ├─ 32.00 MB data splitted into 64 pieces   132.4 ms      │ 137.6 ms      │ 132.9 ms      │ 133.3 ms      │ 100     │ 100
   │                                          241.5 MiB/s   │ 232.4 MiB/s   │ 240.7 MiB/s   │ 239.9 MiB/s   │         │
   ├─ 32.00 MB data splitted into 128 pieces  241.9 ms      │ 249.3 ms      │ 243.1 ms      │ 243.6 ms      │ 100     │ 100
   │                                          132.3 MiB/s   │ 128.4 MiB/s   │ 131.6 MiB/s   │ 131.4 MiB/s   │         │
   ╰─ 32.00 MB data splitted into 256 pieces  476 ms        │ 485.5 ms      │ 479.1 ms      │ 479.4 ms      │ 100     │ 100
                                              67.35 MiB/s   │ 66.03 MiB/s   │ 66.9 MiB/s    │ 66.87 MiB/s   │         │
```

## Usage

To use `rlnc` library crate in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rlnc = "=0.8.1"                                      # On x86 target, it offers AVX2 and SSSE3 optimization for fast encoding, recoding and decoding.
# or
rlnc = { version = "=0.8.1", features = "parallel" } # Uses `rayon`-based data-parallelism for fast encoding/ recoding.

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

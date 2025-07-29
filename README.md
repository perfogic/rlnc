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
|| src/common/simd.rs: 45/67
|| src/full/decoder.rs: 28/33
|| src/full/decoder_matrix.rs: 49/55
|| src/full/encoder.rs: 29/29
|| src/full/recoder.rs: 30/36
|| 
81.90% coverage, 190/232 lines covered
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
Full RLNC Encoder | Throughput of 4.3 GiB/s to 18.6 GiB/s | Throughput of 3.4 GiB/s to 9.7 GiB/s | The number of pieces original data got split into has a **minimal** impact on the encoding speed.
Full RLNC Recoder | Throughput of 5.2 GiB/s to 17.0 GiB/s | Throughput of 2.9 GiB/s to 7.9 GiB/s | Similar to the encoder, the recoder's performance remains largely consistent regardless of how many pieces the original data is split into.
Full RLNC Decoder | Throughput of 67 MiB/s to 1.67 GiB/s | **Doesn't yet implement a parallel decoding mode** | As the number of pieces increases, the decoding time increases substantially, leading to a considerable drop in throughput. This indicates that decoding is the most computationally intensive part of the full RLNC scheme, and its performance is inversely proportional to the number of pieces.

In summary, the full RLNC implementation demonstrates excellent encoding and recoding speeds, consistently achieving GiB/s throughputs with minimal sensitivity to the number of data pieces. The `parallel` feature, leveraging Rust `rayon` data-parallelism framework, also provides good performance for both encoding and recoding. Whether you want to use that feature, completely depends on your usecase. However, decoding remains a much slower operation, with its performance significantly diminishing as the data is split into a greater number of pieces, and currently does **not** implement a parallel decoding algorithm.

#### Full RLNC Encoder

```bash
# Encoding without `rayon` data-parallelism, instead using AVX2-powered SIMD vector x scalar multiplication

Timer precision: 15 ns
full_rlnc_encoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ encode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    70.85 µs      │ 159.8 µs      │ 77.42 µs      │ 78.97 µs      │ 100     │ 100
   │                                          14.64 GiB/s   │ 6.492 GiB/s   │ 13.4 GiB/s    │ 13.13 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            18          │ 18            │ 18            │ 18            │         │
   │                                            1.062 MiB   │ 1.062 MiB     │ 1.062 MiB     │ 1.062 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            17          │ 17            │ 17            │ 17            │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 1.00 MB data splitted into 32 pieces    69.25 µs      │ 88.99 µs      │ 72.13 µs      │ 72.97 µs      │ 100     │ 100
   │                                          14.54 GiB/s   │ 11.31 GiB/s   │ 13.96 GiB/s   │ 13.8 GiB/s    │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            64.06 KiB   │ 64.06 KiB     │ 64.06 KiB     │ 64.06 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            34          │ 34            │ 34            │ 34            │         │
   │                                            1.031 MiB   │ 1.031 MiB     │ 1.031 MiB     │ 1.031 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            33          │ 33            │ 33            │ 33            │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 1.00 MB data splitted into 64 pieces    53.26 µs      │ 67.4 µs       │ 55.43 µs      │ 55.62 µs      │ 100     │ 100
   │                                          18.62 GiB/s   │ 14.71 GiB/s   │ 17.89 GiB/s   │ 17.83 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            32.12 KiB   │ 32.12 KiB     │ 32.12 KiB     │ 32.12 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            66          │ 66            │ 66            │ 66            │         │
   │                                            1.015 MiB   │ 1.015 MiB     │ 1.015 MiB     │ 1.015 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            65          │ 65            │ 65            │ 65            │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 1.00 MB data splitted into 128 pieces   52.41 µs      │ 78.46 µs      │ 55.28 µs      │ 58.43 µs      │ 100     │ 100
   │                                          18.77 GiB/s   │ 12.54 GiB/s   │ 17.8 GiB/s    │ 16.84 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            16.25 KiB   │ 16.25 KiB     │ 16.25 KiB     │ 16.25 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            130         │ 130           │ 130           │ 130           │         │
   │                                            1.008 MiB   │ 1.008 MiB     │ 1.008 MiB     │ 1.008 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            129         │ 129           │ 129           │ 129           │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 1.00 MB data splitted into 256 pieces   56.25 µs      │ 66.85 µs      │ 58.82 µs      │ 59.37 µs      │ 100     │ 100
   │                                          17.43 GiB/s   │ 14.67 GiB/s   │ 16.67 GiB/s   │ 16.51 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            8.501 KiB   │ 8.501 KiB     │ 8.501 KiB     │ 8.501 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            258         │ 258           │ 258           │ 258           │         │
   │                                            1.004 MiB   │ 1.004 MiB     │ 1.004 MiB     │ 1.004 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            257         │ 257           │ 257           │ 257           │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   ├─ 16.00 MB data splitted into 16 pieces   2.475 ms      │ 3.137 ms      │ 2.572 ms      │ 2.602 ms      │ 100     │ 100
   │                                          6.706 GiB/s   │ 5.291 GiB/s   │ 6.453 GiB/s   │ 6.379 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            18          │ 18            │ 18            │ 18            │         │
   │                                            17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            17          │ 17            │ 17            │ 17            │         │
   │                                            16 MiB      │ 16 MiB        │ 16 MiB        │ 16 MiB        │         │
   ├─ 16.00 MB data splitted into 32 pieces   1.584 ms      │ 2.423 ms      │ 1.673 ms      │ 1.745 ms      │ 100     │ 100
   │                                          10.16 GiB/s   │ 6.647 GiB/s   │ 9.627 GiB/s   │ 9.229 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            34          │ 34            │ 34            │ 34            │         │
   │                                            16.5 MiB    │ 16.5 MiB      │ 16.5 MiB      │ 16.5 MiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            33          │ 33            │ 33            │ 33            │         │
   │                                            16 MiB      │ 16 MiB        │ 16 MiB        │ 16 MiB        │         │
   ├─ 16.00 MB data splitted into 64 pieces   1.304 ms      │ 2.149 ms      │ 1.357 ms      │ 1.434 ms      │ 100     │ 100
   │                                          12.16 GiB/s   │ 7.381 GiB/s   │ 11.68 GiB/s   │ 11.06 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            66          │ 66            │ 66            │ 66            │         │
   │                                            16.25 MiB   │ 16.25 MiB     │ 16.25 MiB     │ 16.25 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            65          │ 65            │ 65            │ 65            │         │
   │                                            16 MiB      │ 16 MiB        │ 16 MiB        │ 16 MiB        │         │
   ├─ 16.00 MB data splitted into 128 pieces  1.286 ms      │ 1.9 ms        │ 1.318 ms      │ 1.36 ms       │ 100     │ 100
   │                                          12.24 GiB/s   │ 8.283 GiB/s   │ 11.94 GiB/s   │ 11.57 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            130         │ 130           │ 130           │ 130           │         │
   │                                            16.12 MiB   │ 16.12 MiB     │ 16.12 MiB     │ 16.12 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            129         │ 129           │ 129           │ 129           │         │
   │                                            16 MiB      │ 16 MiB        │ 16 MiB        │ 16 MiB        │         │
   ├─ 16.00 MB data splitted into 256 pieces  1.324 ms      │ 1.887 ms      │ 1.351 ms      │ 1.39 ms       │ 100     │ 100
   │                                          11.84 GiB/s   │ 8.31 GiB/s    │ 11.6 GiB/s    │ 11.28 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128.5 KiB   │ 128.5 KiB     │ 128.5 KiB     │ 128.5 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            258         │ 258           │ 258           │ 258           │         │
   │                                            16.06 MiB   │ 16.06 MiB     │ 16.06 MiB     │ 16.06 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            257         │ 257           │ 257           │ 257           │         │
   │                                            16 MiB      │ 16 MiB        │ 16 MiB        │ 16 MiB        │         │
   ├─ 32.00 MB data splitted into 16 pieces   6.252 ms      │ 7.625 ms      │ 6.42 ms       │ 6.481 ms      │ 100     │ 100
   │                                          5.31 GiB/s    │ 4.354 GiB/s   │ 5.171 GiB/s   │ 5.122 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            18          │ 18            │ 18            │ 18            │         │
   │                                            34 MiB      │ 34 MiB        │ 34 MiB        │ 34 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            17          │ 17            │ 17            │ 17            │         │
   │                                            32 MiB      │ 32 MiB        │ 32 MiB        │ 32 MiB        │         │
   ├─ 32.00 MB data splitted into 32 pieces   4.829 ms      │ 5.558 ms      │ 4.954 ms      │ 5.026 ms      │ 100     │ 100
   │                                          6.672 GiB/s   │ 5.797 GiB/s   │ 6.504 GiB/s   │ 6.411 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            34          │ 34            │ 34            │ 34            │         │
   │                                            33 MiB      │ 33 MiB        │ 33 MiB        │ 33 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            33          │ 33            │ 33            │ 33            │         │
   │                                            32 MiB      │ 32 MiB        │ 32 MiB        │ 32 MiB        │         │
   ├─ 32.00 MB data splitted into 64 pieces   3.786 ms      │ 4.662 ms      │ 3.961 ms      │ 4.031 ms      │ 100     │ 100
   │                                          8.381 GiB/s   │ 6.806 GiB/s   │ 8.01 GiB/s    │ 7.871 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            66          │ 66            │ 66            │ 66            │         │
   │                                            32.5 MiB    │ 32.5 MiB      │ 32.5 MiB      │ 32.5 MiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            65          │ 65            │ 65            │ 65            │         │
   │                                            32 MiB      │ 32 MiB        │ 32 MiB        │ 32 MiB        │         │
   ├─ 32.00 MB data splitted into 128 pieces  3.382 ms      │ 4.069 ms      │ 3.453 ms      │ 3.507 ms      │ 100     │ 100
   │                                          9.312 GiB/s   │ 7.739 GiB/s   │ 9.119 GiB/s   │ 8.979 GiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.2 KiB   │ 512.2 KiB     │ 512.2 KiB     │ 512.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            130         │ 130           │ 130           │ 130           │         │
   │                                            32.25 MiB   │ 32.25 MiB     │ 32.25 MiB     │ 32.25 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            129         │ 129           │ 129           │ 129           │         │
   │                                            32 MiB      │ 32 MiB        │ 32 MiB        │ 32 MiB        │         │
   ╰─ 32.00 MB data splitted into 256 pieces  3.279 ms      │ 4.002 ms      │ 3.472 ms      │ 3.519 ms      │ 100     │ 100
                                              9.565 GiB/s   │ 7.837 GiB/s   │ 9.034 GiB/s   │ 8.913 GiB/s   │         │
                                              max alloc:    │               │               │               │         │
                                                3           │ 3             │ 3             │ 3             │         │
                                                256.5 KiB   │ 256.5 KiB     │ 256.5 KiB     │ 256.5 KiB     │         │
                                              alloc:        │               │               │               │         │
                                                258         │ 258           │ 258           │ 258           │         │
                                                32.12 MiB   │ 32.12 MiB     │ 32.12 MiB     │ 32.12 MiB     │         │
                                              dealloc:      │               │               │               │         │
                                                257         │ 257           │ 257           │ 257           │         │
                                                32 MiB      │ 32 MiB        │ 32 MiB        │ 32 MiB        │         │

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

Timer precision: 14 ns
full_rlnc_recoder                                                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ recode                                                                             │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces, recoding with 8 pieces      39.28 µs      │ 61.43 µs      │ 41.19 µs      │ 42.17 µs      │ 100     │ 100
   │                                                                    13.98 GiB/s   │ 8.943 GiB/s   │ 13.33 GiB/s   │ 13.02 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      12          │ 12            │ 12            │ 12            │         │
   │                                                                      640 KiB     │ 640 KiB       │ 640 KiB       │ 640 KiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      11          │ 11            │ 11            │ 11            │         │
   │                                                                      576 KiB     │ 576 KiB       │ 576 KiB       │ 576 KiB       │         │
   ├─ 1.00 MB data splitted into 32 pieces, recoding with 16 pieces     34.5 µs       │ 39.46 µs      │ 36.99 µs      │ 36.87 µs      │ 100     │ 100
   │                                                                    15.04 GiB/s   │ 13.15 GiB/s   │ 14.03 GiB/s   │ 14.08 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.09 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      20          │ 20            │ 20            │ 20            │         │
   │                                                                      576.1 KiB   │ 576.1 KiB     │ 576.1 KiB     │ 576.1 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      19          │ 19            │ 19            │ 19            │         │
   │                                                                      544 KiB     │ 544 KiB       │ 544 KiB       │ 544 KiB       │         │
   ├─ 1.00 MB data splitted into 64 pieces, recoding with 32 pieces     29.68 µs      │ 35.03 µs      │ 30.13 µs      │ 30.41 µs      │ 100     │ 100
   │                                                                    17.03 GiB/s   │ 14.43 GiB/s   │ 16.77 GiB/s   │ 16.62 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.18 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      36          │ 36            │ 36            │ 36            │         │
   │                                                                      544.2 KiB   │ 544.2 KiB     │ 544.2 KiB     │ 544.2 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      35          │ 35            │ 35            │ 35            │         │
   │                                                                      528.1 KiB   │ 528.1 KiB     │ 528.1 KiB     │ 528.1 KiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces, recoding with 64 pieces    34.71 µs      │ 43.89 µs      │ 36.72 µs      │ 36.88 µs      │ 100     │ 100
   │                                                                    14.51 GiB/s   │ 11.47 GiB/s   │ 13.71 GiB/s   │ 13.65 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.37 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      68          │ 68            │ 68            │ 68            │         │
   │                                                                      528.4 KiB   │ 528.4 KiB     │ 528.4 KiB     │ 528.4 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      67          │ 67            │ 67            │ 67            │         │
   │                                                                      520.3 KiB   │ 520.3 KiB     │ 520.3 KiB     │ 520.3 KiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces, recoding with 128 pieces   60.61 µs      │ 69.06 µs      │ 62.96 µs      │ 63.54 µs      │ 100     │ 100
   │                                                                    8.627 GiB/s   │ 7.572 GiB/s   │ 8.305 GiB/s   │ 8.229 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.751 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      132         │ 132           │ 132           │ 132           │         │
   │                                                                      520.8 KiB   │ 520.8 KiB     │ 520.8 KiB     │ 520.8 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      131         │ 131           │ 131           │ 131           │         │
   │                                                                      516.6 KiB   │ 516.6 KiB     │ 516.6 KiB     │ 516.6 KiB     │         │
   ├─ 16.00 MB data splitted into 16 pieces, recoding with 8 pieces     1.26 ms       │ 1.503 ms      │ 1.294 ms      │ 1.308 ms      │ 100     │ 100
   │                                                                    6.973 GiB/s   │ 5.847 GiB/s   │ 6.788 GiB/s   │ 6.718 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      12          │ 12            │ 12            │ 12            │         │
   │                                                                      10 MiB      │ 10 MiB        │ 10 MiB        │ 10 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      11          │ 11            │ 11            │ 11            │         │
   │                                                                      9 MiB       │ 9 MiB         │ 9 MiB         │ 9 MiB         │         │
   ├─ 16.00 MB data splitted into 32 pieces, recoding with 16 pieces    899.1 µs      │ 1.637 ms      │ 964.6 µs      │ 1.005 ms      │ 100     │ 100
   │                                                                    9.232 GiB/s   │ 5.069 GiB/s   │ 8.605 GiB/s   │ 8.255 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      20          │ 20            │ 20            │ 20            │         │
   │                                                                      9 MiB       │ 9 MiB         │ 9 MiB         │ 9 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      19          │ 19            │ 19            │ 19            │         │
   │                                                                      8.5 MiB     │ 8.5 MiB       │ 8.5 MiB       │ 8.5 MiB       │         │
   ├─ 16.00 MB data splitted into 64 pieces, recoding with 32 pieces    700 µs        │ 932.3 µs      │ 781.4 µs      │ 790.4 µs      │ 100     │ 100
   │                                                                    11.51 GiB/s   │ 8.643 GiB/s   │ 10.31 GiB/s   │ 10.19 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      36          │ 36            │ 36            │ 36            │         │
   │                                                                      8.5 MiB     │ 8.5 MiB       │ 8.5 MiB       │ 8.5 MiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      35          │ 35            │ 35            │ 35            │         │
   │                                                                      8.25 MiB    │ 8.25 MiB      │ 8.25 MiB      │ 8.25 MiB      │         │
   ├─ 16.00 MB data splitted into 128 pieces, recoding with 64 pieces   689.1 µs      │ 1.035 ms      │ 775.3 µs      │ 793.4 µs      │ 100     │ 100
   │                                                                    11.52 GiB/s   │ 7.673 GiB/s   │ 10.24 GiB/s   │ 10.01 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      68          │ 68            │ 68            │ 68            │         │
   │                                                                      8.25 MiB    │ 8.25 MiB      │ 8.25 MiB      │ 8.25 MiB      │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      67          │ 67            │ 67            │ 67            │         │
   │                                                                      8.125 MiB   │ 8.125 MiB     │ 8.125 MiB     │ 8.125 MiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces, recoding with 128 pieces  737.7 µs      │ 1.143 ms      │ 801.3 µs      │ 818.4 µs      │ 100     │ 100
   │                                                                    10.71 GiB/s   │ 6.913 GiB/s   │ 9.864 GiB/s   │ 9.657 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      132         │ 132           │ 132           │ 132           │         │
   │                                                                      8.125 MiB   │ 8.125 MiB     │ 8.125 MiB     │ 8.125 MiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      131         │ 131           │ 131           │ 131           │         │
   │                                                                      8.063 MiB   │ 8.063 MiB     │ 8.063 MiB     │ 8.063 MiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces, recoding with 8 pieces     2.929 ms      │ 3.376 ms      │ 3.034 ms      │ 3.061 ms      │ 100     │ 100
   │                                                                    5.999 GiB/s   │ 5.205 GiB/s   │ 5.792 GiB/s   │ 5.741 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      12          │ 12            │ 12            │ 12            │         │
   │                                                                      20 MiB      │ 20 MiB        │ 20 MiB        │ 20 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      11          │ 11            │ 11            │ 11            │         │
   │                                                                      18 MiB      │ 18 MiB        │ 18 MiB        │ 18 MiB        │         │
   ├─ 32.00 MB data splitted into 32 pieces, recoding with 16 pieces    2.525 ms      │ 2.856 ms      │ 2.635 ms      │ 2.643 ms      │ 100     │ 100
   │                                                                    6.574 GiB/s   │ 5.812 GiB/s   │ 6.299 GiB/s   │ 6.28 GiB/s    │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      20          │ 20            │ 20            │ 20            │         │
   │                                                                      18 MiB      │ 18 MiB        │ 18 MiB        │ 18 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      19          │ 19            │ 19            │ 19            │         │
   │                                                                      17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   ├─ 32.00 MB data splitted into 64 pieces, recoding with 32 pieces    1.945 ms      │ 2.249 ms      │ 1.975 ms      │ 2.005 ms      │ 100     │ 100
   │                                                                    8.281 GiB/s   │ 7.162 GiB/s   │ 8.155 GiB/s   │ 8.035 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      36          │ 36            │ 36            │ 36            │         │
   │                                                                      17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      35          │ 35            │ 35            │ 35            │         │
   │                                                                      16.5 MiB    │ 16.5 MiB      │ 16.5 MiB      │ 16.5 MiB      │         │
   ├─ 32.00 MB data splitted into 128 pieces, recoding with 64 pieces   1.685 ms      │ 1.941 ms      │ 1.731 ms      │ 1.747 ms      │ 100     │ 100
   │                                                                    9.42 GiB/s    │ 8.176 GiB/s   │ 9.168 GiB/s   │ 9.083 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      68          │ 68            │ 68            │ 68            │         │
   │                                                                      16.5 MiB    │ 16.5 MiB      │ 16.5 MiB      │ 16.5 MiB      │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      67          │ 67            │ 67            │ 67            │         │
   │                                                                      16.25 MiB   │ 16.25 MiB     │ 16.25 MiB     │ 16.25 MiB     │         │
   ╰─ 32.00 MB data splitted into 256 pieces, recoding with 128 pieces  1.714 ms      │ 1.918 ms      │ 1.739 ms      │ 1.746 ms      │ 100     │ 100
                                                                        9.201 GiB/s   │ 8.223 GiB/s   │ 9.07 GiB/s    │ 9.032 GiB/s   │         │
                                                                        max alloc:    │               │               │               │         │
                                                                          4           │ 4             │ 4             │ 4             │         │
                                                                          256.7 KiB   │ 256.7 KiB     │ 256.7 KiB     │ 256.7 KiB     │         │
                                                                        alloc:        │               │               │               │         │
                                                                          132         │ 132           │ 132           │ 132           │         │
                                                                          16.25 MiB   │ 16.25 MiB     │ 16.25 MiB     │ 16.25 MiB     │         │
                                                                        dealloc:      │               │               │               │         │
                                                                          131         │ 131           │ 131           │ 131           │         │
                                                                          16.12 MiB   │ 16.12 MiB     │ 16.12 MiB     │ 16.12 MiB     │         │

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
rlnc = "=0.7.0"                                      # On x86 target, it offers AVX2 and SSSE3 optimization for fast encoding, recoding and decoding.
# or
rlnc = { version = "=0.7.0", features = "parallel" } # Uses `rayon`-based data-parallelism for fast encoding/ recoding.

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

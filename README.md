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

#### Full RLNC Encoder

```bash
Timer precision: 19 ns
full_rlnc_encoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ encode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    479.9 µs      │ 723 µs        │ 603.5 µs      │ 598.2 µs      │ 100     │ 100
   │                                          130.2 MiB/s   │ 86.46 MiB/s   │ 103.5 MiB/s   │ 104.4 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            1.187 MiB   │ 1.187 MiB     │ 1.187 MiB     │ 1.187 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            1.125 MiB   │ 1.125 MiB     │ 1.125 MiB     │ 1.125 MiB     │         │
   ├─ 1.00 MB data splitted into 32 pieces    475.8 µs      │ 577.3 µs      │ 508.3 µs      │ 511.8 µs      │ 100     │ 100
   │                                          65.73 MiB/s   │ 54.17 MiB/s   │ 61.53 MiB/s   │ 61.11 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            64.06 KiB   │ 64.06 KiB     │ 64.06 KiB     │ 64.06 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            1.093 MiB   │ 1.093 MiB     │ 1.093 MiB     │ 1.093 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            1.062 MiB   │ 1.062 MiB     │ 1.062 MiB     │ 1.062 MiB     │         │
   ├─ 1.00 MB data splitted into 64 pieces    496.1 µs      │ 559.7 µs      │ 515.9 µs      │ 519.9 µs      │ 100     │ 100
   │                                          31.61 MiB/s   │ 28.02 MiB/s   │ 30.4 MiB/s    │ 30.17 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            32.12 KiB   │ 32.12 KiB     │ 32.12 KiB     │ 32.12 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            1.047 MiB   │ 1.047 MiB     │ 1.047 MiB     │ 1.047 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            1.031 MiB   │ 1.031 MiB     │ 1.031 MiB     │ 1.031 MiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces   509.4 µs      │ 601.7 µs      │ 531.7 µs      │ 535.3 µs      │ 100     │ 100
   │                                          15.57 MiB/s   │ 13.18 MiB/s   │ 14.92 MiB/s   │ 14.82 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            16.25 KiB   │ 16.25 KiB     │ 16.25 KiB     │ 16.25 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            1.023 MiB   │ 1.023 MiB     │ 1.023 MiB     │ 1.023 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            1.015 MiB   │ 1.015 MiB     │ 1.015 MiB     │ 1.015 MiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces   513.9 µs      │ 604.2 µs      │ 537.6 µs      │ 540.3 µs      │ 100     │ 100
   │                                          8.077 MiB/s   │ 6.87 MiB/s    │ 7.721 MiB/s   │ 7.682 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            8.501 KiB   │ 8.501 KiB     │ 8.501 KiB     │ 8.501 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            260         │ 260           │ 260           │ 260           │         │
   │                                            1.012 MiB   │ 1.012 MiB     │ 1.012 MiB     │ 1.012 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            259         │ 259           │ 259           │ 259           │         │
   │                                            1.008 MiB   │ 1.008 MiB     │ 1.008 MiB     │ 1.008 MiB     │         │
   ├─ 16.00 MB data splitted into 16 pieces   8.447 ms      │ 9.493 ms      │ 8.992 ms      │ 8.989 ms      │ 100     │ 100
   │                                          118.3 MiB/s   │ 105.3 MiB/s   │ 111.2 MiB/s   │ 111.2 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            19 MiB      │ 19 MiB        │ 19 MiB        │ 19 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            18 MiB      │ 18 MiB        │ 18 MiB        │ 18 MiB        │         │
   ├─ 16.00 MB data splitted into 32 pieces   8.4 ms        │ 11.19 ms      │ 9.1 ms        │ 9.147 ms      │ 100     │ 100
   │                                          59.52 MiB/s   │ 44.68 MiB/s   │ 54.94 MiB/s   │ 54.66 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            17.5 MiB    │ 17.5 MiB      │ 17.5 MiB      │ 17.5 MiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   ├─ 16.00 MB data splitted into 64 pieces   8.575 ms      │ 10.8 ms       │ 9.025 ms      │ 9.074 ms      │ 100     │ 100
   │                                          29.16 MiB/s   │ 23.14 MiB/s   │ 27.7 MiB/s    │ 27.55 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            16.75 MiB   │ 16.75 MiB     │ 16.75 MiB     │ 16.75 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            16.5 MiB    │ 16.5 MiB      │ 16.5 MiB      │ 16.5 MiB      │         │
   ├─ 16.00 MB data splitted into 128 pieces  8.552 ms      │ 10.43 ms      │ 8.989 ms      │ 9.028 ms      │ 100     │ 100
   │                                          14.62 MiB/s   │ 11.98 MiB/s   │ 13.91 MiB/s   │ 13.85 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            256.2 KiB   │ 256.2 KiB     │ 256.2 KiB     │ 256.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            16.37 MiB   │ 16.37 MiB     │ 16.37 MiB     │ 16.37 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            16.25 MiB   │ 16.25 MiB     │ 16.25 MiB     │ 16.25 MiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces  8.599 ms      │ 10.45 ms      │ 8.983 ms      │ 9.021 ms      │ 100     │ 100
   │                                          7.296 MiB/s   │ 6.004 MiB/s   │ 6.984 MiB/s   │ 6.954 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            128.5 KiB   │ 128.5 KiB     │ 128.5 KiB     │ 128.5 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            260         │ 260           │ 260           │ 260           │         │
   │                                            16.18 MiB   │ 16.18 MiB     │ 16.18 MiB     │ 16.18 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            259         │ 259           │ 259           │ 259           │         │
   │                                            16.12 MiB   │ 16.12 MiB     │ 16.12 MiB     │ 16.12 MiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces   19.09 ms      │ 22.41 ms      │ 20.33 ms      │ 20.42 ms      │ 100     │ 100
   │                                          104.7 MiB/s   │ 89.22 MiB/s   │ 98.35 MiB/s   │ 97.9 MiB/s    │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            20          │ 20            │ 20            │ 20            │         │
   │                                            38 MiB      │ 38 MiB        │ 38 MiB        │ 38 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            19          │ 19            │ 19            │ 19            │         │
   │                                            36 MiB      │ 36 MiB        │ 36 MiB        │ 36 MiB        │         │
   ├─ 32.00 MB data splitted into 32 pieces   17.86 ms      │ 22.52 ms      │ 18.98 ms      │ 19.01 ms      │ 100     │ 100
   │                                          55.98 MiB/s   │ 44.39 MiB/s   │ 52.68 MiB/s   │ 52.58 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            36          │ 36            │ 36            │ 36            │         │
   │                                            35 MiB      │ 35 MiB        │ 35 MiB        │ 35 MiB        │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            35          │ 35            │ 35            │ 35            │         │
   │                                            34 MiB      │ 34 MiB        │ 34 MiB        │ 34 MiB        │         │
   ├─ 32.00 MB data splitted into 64 pieces   17.52 ms      │ 21.57 ms      │ 18.58 ms      │ 18.62 ms      │ 100     │ 100
   │                                          28.53 MiB/s   │ 23.18 MiB/s   │ 26.91 MiB/s   │ 26.84 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                          alloc:        │               │               │               │         │
   │                                            68          │ 68            │ 68            │ 68            │         │
   │                                            33.5 MiB    │ 33.5 MiB      │ 33.5 MiB      │ 33.5 MiB      │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            67          │ 67            │ 67            │ 67            │         │
   │                                            33 MiB      │ 33 MiB        │ 33 MiB        │ 33 MiB        │         │
   ├─ 32.00 MB data splitted into 128 pieces  16.01 ms      │ 20.3 ms       │ 17.39 ms      │ 17.56 ms      │ 100     │ 100
   │                                          15.61 MiB/s   │ 12.31 MiB/s   │ 14.37 MiB/s   │ 14.24 MiB/s   │         │
   │                                          max alloc:    │               │               │               │         │
   │                                            3           │ 3             │ 3             │ 3             │         │
   │                                            512.2 KiB   │ 512.2 KiB     │ 512.2 KiB     │ 512.2 KiB     │         │
   │                                          alloc:        │               │               │               │         │
   │                                            132         │ 132           │ 132           │ 132           │         │
   │                                            32.75 MiB   │ 32.75 MiB     │ 32.75 MiB     │ 32.75 MiB     │         │
   │                                          dealloc:      │               │               │               │         │
   │                                            131         │ 131           │ 131           │ 131           │         │
   │                                            32.5 MiB    │ 32.5 MiB      │ 32.5 MiB      │ 32.5 MiB      │         │
   ╰─ 32.00 MB data splitted into 256 pieces  16.3 ms       │ 18.92 ms      │ 17.36 ms      │ 17.51 ms      │ 100     │ 100
                                              7.683 MiB/s   │ 6.618 MiB/s   │ 7.211 MiB/s   │ 7.149 MiB/s   │         │
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
Timer precision: 19 ns
full_rlnc_recoder                                                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ recode                                                                             │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces, recoding with 8 pieces      224.2 µs      │ 287.4 µs      │ 246.5 µs      │ 246.1 µs      │ 100     │ 100
   │                                                                    2.178 GiB/s   │ 1.698 GiB/s   │ 1.98 GiB/s    │ 1.984 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128 KiB     │ 128 KiB       │ 128 KiB       │ 128 KiB       │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      768 KiB     │ 768 KiB       │ 768 KiB       │ 768 KiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      704 KiB     │ 704 KiB       │ 704 KiB       │ 704 KiB       │         │
   ├─ 1.00 MB data splitted into 32 pieces, recoding with 16 pieces     240.5 µs      │ 316.9 µs      │ 276.1 µs      │ 279 µs        │ 100     │ 100
   │                                                                    2.031 GiB/s   │ 1.542 GiB/s   │ 1.77 GiB/s    │ 1.751 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      64.09 KiB   │ 64.09 KiB     │ 64.09 KiB     │ 64.09 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      640.1 KiB   │ 640.1 KiB     │ 640.1 KiB     │ 640.1 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      608 KiB     │ 608 KiB       │ 608 KiB       │ 608 KiB       │         │
   ├─ 1.00 MB data splitted into 64 pieces, recoding with 32 pieces     246.9 µs      │ 385.1 µs      │ 256 µs        │ 262.5 µs      │ 100     │ 100
   │                                                                    1.984 GiB/s   │ 1.272 GiB/s   │ 1.914 GiB/s   │ 1.866 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      32.18 KiB   │ 32.18 KiB     │ 32.18 KiB     │ 32.18 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      576.2 KiB   │ 576.2 KiB     │ 576.2 KiB     │ 576.2 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      560.1 KiB   │ 560.1 KiB     │ 560.1 KiB     │ 560.1 KiB     │         │
   ├─ 1.00 MB data splitted into 128 pieces, recoding with 64 pieces    242.3 µs      │ 337.9 µs      │ 262.3 µs      │ 264.8 µs      │ 100     │ 100
   │                                                                    2.046 GiB/s   │ 1.467 GiB/s   │ 1.89 GiB/s    │ 1.872 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      16.37 KiB   │ 16.37 KiB     │ 16.37 KiB     │ 16.37 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      544.4 KiB   │ 544.4 KiB     │ 544.4 KiB     │ 544.4 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      536.3 KiB   │ 536.3 KiB     │ 536.3 KiB     │ 536.3 KiB     │         │
   ├─ 1.00 MB data splitted into 256 pieces, recoding with 128 pieces   262.4 µs      │ 436.2 µs      │ 283.6 µs      │ 289.4 µs      │ 100     │ 100
   │                                                                    1.977 GiB/s   │ 1.189 GiB/s   │ 1.83 GiB/s    │ 1.793 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      8.751 KiB   │ 8.751 KiB     │ 8.751 KiB     │ 8.751 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      134         │ 134           │ 134           │ 134           │         │
   │                                                                      528.8 KiB   │ 528.8 KiB     │ 528.8 KiB     │ 528.8 KiB     │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      133         │ 133           │ 133           │ 133           │         │
   │                                                                      524.6 KiB   │ 524.6 KiB     │ 524.6 KiB     │ 524.6 KiB     │         │
   ├─ 16.00 MB data splitted into 16 pieces, recoding with 8 pieces     3.932 ms      │ 4.984 ms      │ 4.475 ms      │ 4.499 ms      │ 100     │ 100
   │                                                                    1.986 GiB/s   │ 1.567 GiB/s   │ 1.745 GiB/s   │ 1.736 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      12 MiB      │ 12 MiB        │ 12 MiB        │ 12 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      11 MiB      │ 11 MiB        │ 11 MiB        │ 11 MiB        │         │
   ├─ 16.00 MB data splitted into 32 pieces, recoding with 16 pieces    3.736 ms      │ 5.141 ms      │ 4.238 ms      │ 4.214 ms      │ 100     │ 100
   │                                                                    2.091 GiB/s   │ 1.519 GiB/s   │ 1.843 GiB/s   │ 1.853 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      10 MiB      │ 10 MiB        │ 10 MiB        │ 10 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      9.5 MiB     │ 9.5 MiB       │ 9.5 MiB       │ 9.5 MiB       │         │
   ├─ 16.00 MB data splitted into 64 pieces, recoding with 32 pieces    3.849 ms      │ 4.413 ms      │ 4.055 ms      │ 4.059 ms      │ 100     │ 100
   │                                                                    2.03 GiB/s    │ 1.77 GiB/s    │ 1.926 GiB/s   │ 1.924 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.1 KiB   │ 512.1 KiB     │ 512.1 KiB     │ 512.1 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      9 MiB       │ 9 MiB         │ 9 MiB         │ 9 MiB         │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      8.75 MiB    │ 8.75 MiB      │ 8.75 MiB      │ 8.75 MiB      │         │
   ├─ 16.00 MB data splitted into 128 pieces, recoding with 64 pieces   3.812 ms      │ 4.793 ms      │ 3.977 ms      │ 4 ms          │ 100     │ 100
   │                                                                    2.051 GiB/s   │ 1.631 GiB/s   │ 1.965 GiB/s   │ 1.954 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      256.3 KiB   │ 256.3 KiB     │ 256.3 KiB     │ 256.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      8.5 MiB     │ 8.5 MiB       │ 8.5 MiB       │ 8.5 MiB       │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      8.375 MiB   │ 8.375 MiB     │ 8.375 MiB     │ 8.375 MiB     │         │
   ├─ 16.00 MB data splitted into 256 pieces, recoding with 128 pieces  3.841 ms      │ 4.453 ms      │ 3.981 ms      │ 3.994 ms      │ 100     │ 100
   │                                                                    2.041 GiB/s   │ 1.761 GiB/s   │ 1.969 GiB/s   │ 1.963 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      128.7 KiB   │ 128.7 KiB     │ 128.7 KiB     │ 128.7 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      134         │ 134           │ 134           │ 134           │         │
   │                                                                      8.25 MiB    │ 8.25 MiB      │ 8.25 MiB      │ 8.25 MiB      │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      133         │ 133           │ 133           │ 133           │         │
   │                                                                      8.188 MiB   │ 8.188 MiB     │ 8.188 MiB     │ 8.188 MiB     │         │
   ├─ 32.00 MB data splitted into 16 pieces, recoding with 8 pieces     8.676 ms      │ 10.52 ms      │ 9.452 ms      │ 9.471 ms      │ 100     │ 100
   │                                                                    1.8 GiB/s     │ 1.484 GiB/s   │ 1.652 GiB/s   │ 1.649 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      4 MiB       │ 4 MiB         │ 4 MiB         │ 4 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      14          │ 14            │ 14            │ 14            │         │
   │                                                                      24 MiB      │ 24 MiB        │ 24 MiB        │ 24 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      13          │ 13            │ 13            │ 13            │         │
   │                                                                      22 MiB      │ 22 MiB        │ 22 MiB        │ 22 MiB        │         │
   ├─ 32.00 MB data splitted into 32 pieces, recoding with 16 pieces    8.099 ms      │ 9.825 ms      │ 8.868 ms      │ 8.875 ms      │ 100     │ 100
   │                                                                    1.929 GiB/s   │ 1.59 GiB/s    │ 1.761 GiB/s   │ 1.76 GiB/s    │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      2 MiB       │ 2 MiB         │ 2 MiB         │ 2 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      22          │ 22            │ 22            │ 22            │         │
   │                                                                      20 MiB      │ 20 MiB        │ 20 MiB        │ 20 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      21          │ 21            │ 21            │ 21            │         │
   │                                                                      19 MiB      │ 19 MiB        │ 19 MiB        │ 19 MiB        │         │
   ├─ 32.00 MB data splitted into 64 pieces, recoding with 32 pieces    7.968 ms      │ 9.05 ms       │ 8.416 ms      │ 8.447 ms      │ 100     │ 100
   │                                                                    1.961 GiB/s   │ 1.726 GiB/s   │ 1.856 GiB/s   │ 1.849 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      1 MiB       │ 1 MiB         │ 1 MiB         │ 1 MiB         │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      38          │ 38            │ 38            │ 38            │         │
   │                                                                      18 MiB      │ 18 MiB        │ 18 MiB        │ 18 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      37          │ 37            │ 37            │ 37            │         │
   │                                                                      17.5 MiB    │ 17.5 MiB      │ 17.5 MiB      │ 17.5 MiB      │         │
   ├─ 32.00 MB data splitted into 128 pieces, recoding with 64 pieces   7.634 ms      │ 8.788 ms      │ 8.212 ms      │ 8.22 ms       │ 100     │ 100
   │                                                                    2.047 GiB/s   │ 1.778 GiB/s   │ 1.903 GiB/s   │ 1.901 GiB/s   │         │
   │                                                                    max alloc:    │               │               │               │         │
   │                                                                      4           │ 4             │ 4             │ 4             │         │
   │                                                                      512.3 KiB   │ 512.3 KiB     │ 512.3 KiB     │ 512.3 KiB     │         │
   │                                                                    alloc:        │               │               │               │         │
   │                                                                      70          │ 70            │ 70            │ 70            │         │
   │                                                                      17 MiB      │ 17 MiB        │ 17 MiB        │ 17 MiB        │         │
   │                                                                    dealloc:      │               │               │               │         │
   │                                                                      69          │ 69            │ 69            │ 69            │         │
   │                                                                      16.75 MiB   │ 16.75 MiB     │ 16.75 MiB     │ 16.75 MiB     │         │
   ╰─ 32.00 MB data splitted into 256 pieces, recoding with 128 pieces  7.795 ms      │ 8.879 ms      │ 8.104 ms      │ 8.162 ms      │ 100     │ 100
                                                                        2.008 GiB/s   │ 1.763 GiB/s   │ 1.931 GiB/s   │ 1.918 GiB/s   │         │
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
Timer precision: 20 ns
full_rlnc_decoder                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ decode                                                   │               │               │               │         │
   ├─ 1.00 MB data splitted into 16 pieces    13.29 ms      │ 13.72 ms      │ 13.33 ms      │ 13.37 ms      │ 100     │ 100
   │                                          1.175 GiB/s   │ 1.138 GiB/s   │ 1.171 GiB/s   │ 1.168 GiB/s   │         │
   ├─ 1.00 MB data splitted into 32 pieces    26.84 ms      │ 27.76 ms      │ 26.91 ms      │ 26.98 ms      │ 100     │ 100
   │                                          1.164 GiB/s   │ 1.125 GiB/s   │ 1.161 GiB/s   │ 1.158 GiB/s   │         │
   ├─ 1.00 MB data splitted into 64 pieces    53.9 ms       │ 56.37 ms      │ 54.08 ms      │ 54.37 ms      │ 100     │ 100
   │                                          1.159 GiB/s   │ 1.108 GiB/s   │ 1.155 GiB/s   │ 1.149 GiB/s   │         │
   ├─ 1.00 MB data splitted into 128 pieces   109.2 ms      │ 113.3 ms      │ 111.8 ms      │ 111.7 ms      │ 100     │ 100
   │                                          1.144 GiB/s   │ 1.102 GiB/s   │ 1.117 GiB/s   │ 1.118 GiB/s   │         │
   ├─ 1.00 MB data splitted into 256 pieces   235.2 ms      │ 242.3 ms      │ 239.9 ms      │ 239.4 ms      │ 100     │ 100
   │                                          1.062 GiB/s   │ 1.031 GiB/s   │ 1.042 GiB/s   │ 1.044 GiB/s   │         │
   ├─ 16.00 MB data splitted into 16 pieces   238 ms        │ 246 ms        │ 240.8 ms      │ 241.3 ms      │ 100     │ 100
   │                                          1.05 GiB/s    │ 1.015 GiB/s   │ 1.037 GiB/s   │ 1.035 GiB/s   │         │
   ├─ 16.00 MB data splitted into 32 pieces   462.4 ms      │ 500.5 ms      │ 475.9 ms      │ 477.5 ms      │ 100     │ 100
   │                                          1.081 GiB/s   │ 1022 MiB/s    │ 1.05 GiB/s    │ 1.047 GiB/s   │         │
   ├─ 16.00 MB data splitted into 64 pieces   922.3 ms      │ 989.7 ms      │ 963.2 ms      │ 963.9 ms      │ 100     │ 100
   │                                          1.084 GiB/s   │ 1.01 GiB/s    │ 1.038 GiB/s   │ 1.037 GiB/s   │         │
   ├─ 16.00 MB data splitted into 128 pieces  1.902 s       │ 1.984 s       │ 1.929 s       │ 1.939 s       │ 52      │ 52
   │                                          1.051 GiB/s   │ 1.008 GiB/s   │ 1.036 GiB/s   │ 1.031 GiB/s   │         │
   ├─ 16.00 MB data splitted into 256 pieces  3.789 s       │ 3.999 s       │ 3.896 s       │ 3.904 s       │ 26      │ 26
   │                                          1.055 GiB/s   │ 1 GiB/s       │ 1.026 GiB/s   │ 1.024 GiB/s   │         │
   ├─ 32.00 MB data splitted into 16 pieces   486.6 ms      │ 535.1 ms      │ 503.5 ms      │ 504 ms        │ 100     │ 100
   │                                          1.027 GiB/s   │ 956.7 MiB/s   │ 1016 MiB/s    │ 1015 MiB/s    │         │
   ├─ 32.00 MB data splitted into 32 pieces   965.6 ms      │ 1.034 s       │ 994.7 ms      │ 995 ms        │ 100     │ 100
   │                                          1.035 GiB/s   │ 990.2 MiB/s   │ 1.005 GiB/s   │ 1.004 GiB/s   │         │
   ├─ 32.00 MB data splitted into 64 pieces   1.913 s       │ 2.037 s       │ 1.946 s       │ 1.968 s       │ 51      │ 51
   │                                          1.045 GiB/s   │ 1005 MiB/s    │ 1.027 GiB/s   │ 1.016 GiB/s   │         │
   ├─ 32.00 MB data splitted into 128 pieces  3.834 s       │ 4.031 s       │ 3.936 s       │ 3.94 s        │ 26      │ 26
   │                                          1.043 GiB/s   │ 1015 MiB/s    │ 1.016 GiB/s   │ 1.014 GiB/s   │         │
   ╰─ 32.00 MB data splitted into 256 pieces  7.732 s       │ 7.924 s       │ 7.754 s       │ 7.776 s       │ 13      │ 13
                                              1.034 GiB/s   │ 1.009 GiB/s   │ 1.031 GiB/s   │ 1.028 GiB/s   │         │
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

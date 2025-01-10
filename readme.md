# Discombobugraph

**Discombobugraph** is a Rust library for analyzing the randomness of bitstreams using various statistical tests, including Shannon entropy, frequency test, runs test, serial test, chi-square test, and autocorrelation test.

## Features
- Shannon entropy calculation
- Statistical randomness tests (Z-scores):
  - Frequency test
  - Runs test
  - Serial test
  - Chi-square test
  - Autocorrelation test
- Python bindings via `PyO3`
- FFI bindings for interoperability with C

## Installation

### Rust
Add `discombobugraph` to your project:
```bash
cargo add discombobugraph
```

### Python
Install using `maturin`:
```bash
maturin build --release
```

## Usage

### Rust Example
```rust
use discombobugraph::Discombobugraph;

fn main() {
    let analyzer = Discombobugraph::new();
    let bitstream = vec![0b11001010, 0b10101010];
    let results = analyzer.run(bitstream);
    println!("Randomness analysis results: {:?}", results);
}
```

### Python Example
```python
from discombobugraph import Discombobugraph

analyzer = Discombobugraph()
bitstream = [0b11001010, 0b10101010]
results = analyzer.run(bitstream)
print("Randomness analysis results:", results)
```

### C Example
```c
#include <stdio.h>
#include "discombobugraph.h"

int main() {
    Discombobugraph* analyzer = discombobugraph_new();
    unsigned char bitstream[] = {0b11001010, 0b10101010};
    double results[11];
    int result_len = discombobugraph_run(analyzer, bitstream, 2, results, 11);

    printf("Results:\n");
    for (int i = 0; i < result_len; ++i) {
        printf("%f\n", results[i]);
    }

    discombobugraph_free(analyzer);
    return 0;
}
```

### Commandline Tool

The library ships with a binary executable that takes in any bitstream through pipe and outputs the test result:

```bash
cat /dev/random | head -c 1024000 | ./discombobugraph
Bitstream length: 1024000
Shannon : 0.599323
Freq    : 1004.106167
Runs    : 11398.389464
Pairs   : 16775151684.525328
χ2      : 2.363281
AC (1)  : 0.013563
AC (4)  : 0.263024
AC (8)  : 0.434223
AC (10%): -142.900550
AC (25%): -357.145242
AC (50%): -714.956698
```
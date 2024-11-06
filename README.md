## Overview

RDTSC-Probe uses the CPU's RDTSC instruct to analyze timing patterns and detect virtual machines. By analyzing cycle deltas and statistical variations, it can identify when code is running inside a VM.

## Features

- Zero dependencies
- Lightweight and fast
- Single function implementation 
- Multiple detection patterns
- Statistical pattern analysis
- Cross-platform
  
## How it Works

RDTSC-Probe performs multiple timing tests:
1. Simple arithmetic operations
2. Complex floating-point calculations
3. Branch-heavy operations
4. Memory access patterns

For each test, it:
- Collects timing samples using RDTSC
- Analyzes cycle patterns
- Performs statistical analysis
- Identifies VM-characteristic patterns

## Detection Methods

Checks for several VM indicators:
- Abnormal timing variations
- Consistent sub-20 cycle operations
- Statistical anomalies in cycle patterns
- Unusual timing uniformity

## Requirements
- x86_64 architecture (RDTSC support)



## Disclaimer

This tool may vary in effectiveness across different VM configurations.

# SPL Memo Compression Extension

An extension to the [Solana Memo Program](https://github.com/solana-labs/solana-program-library/tree/master/memo) that adds ZK compression support for efficient on-chain storage.

## Requirements 

1. **Program Extension**
   - Extend SPL Memo program with compressed memo storage
   - Implement using Light Protocol's ZK compression
   - Avoid high-level Light Protocol macros (no light_program/light_accounts)

2. **Testing**
   - Rust unit tests for program validation
   - TypeScript integration tests with ZK compression
   - Test program functionality using local validator

## Implementation Details

### Program Structure

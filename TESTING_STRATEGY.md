# Testing Strategy: Soroban Cost Linter

Because this is a static analysis tool designed to catch edge-case cost heuristics in Soroban contracts, unit testing single files is insufficient. We need to validate the linter's accuracy and false-positive rate against real-world, production-grade Soroban code.

## 1. CI/CD Integration Testing Matrix

Our primary integration test suite will run automatically via GitHub Actions on every pull request.

### 1.1 The Target: `soroban-examples`
The official [stellar/soroban-examples](https://github.com/stellar/soroban-examples) repository will serve as our primary validation matrix. 
- **Setup**: In CI, we will dynamically clone the `soroban-examples` repository.
- **Execution**: We will run `cargo dylint --all` using our compiled plugin against the `soroban-examples` workspace.
- **Validation**: Since `soroban-examples` are highly optimized, they should act as our **False Positive Baseline**. If our linter triggers warnings on standard examples (unless explicitly intended), the PR will fail the CI check.

### 1.2 The Target: Known Expensive Contracts (UI Tests)
We will maintain an internal `ui_tests/` directory containing small, contrived Soroban contracts that explicitly violate our heuristics (e.g., infinite loops writing to storage).
- **Execution**: CI will run `cargo test`, triggering `dylint_testing::ui_test` which compares the compiler stderr output against a set of known `.stderr` fixture files.
- **Validation**: This ensures that our lints **actually fire** when they are supposed to (True Positives).

### 1.3 The Target: Select Testnet Contracts
As a secondary, manual validation phase for releases, we will run the linter against a curated list of open-source protocols currently deployed on the Soroban Testnet (e.g., AMMs, lending protocols).
- This will help us tune our "Confidence Scoring" and uncover new heuristic patterns.

## 2. CI Pipeline Steps
The GitHub Actions workflow (to be implemented in Phase 2) will follow these core steps:
1. `cargo build` (Compile the Dylint plugin)
2. `cargo test` (Run internal UI tests against contrived expensive contracts)
3. `git clone https://github.com/stellar/soroban-examples.git`
4. `DYLINT_LIBRARY_PATH=$(pwd)/target/debug cargo dylint --manifest-path soroban-examples/Cargo.toml --workspace`
5. Assert exit code `0` (ensure no unexpected false positives on the examples repo).

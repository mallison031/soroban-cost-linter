# Product Requirements Document: Soroban Cost Linter

## 1. Objective and Scope
The Soroban Cost Linter is a static, write-time analysis tool built as a Dylint plugin. Its primary objective is to shift cost-analysis left, enabling developers to identify resource-expensive Rust patterns in Soroban smart contracts at compile time. 

**Scope:**
*   **In-Scope**: Static detection of known expensive operations mapping to Soroban's metered resource dimensions (e.g., CPU instructions, memory allocations, ledger read/writes).
*   **Out-of-Scope**: Dynamic profiling, precise gas estimation of exact transaction costs, and security vulnerability detection (which is delegated to tools like Scout).

## 2. Initial Cost-Lints
To provide immediate value, the first iteration will focus on the following heuristic lints:

### 2.1. Unbounded Storage Writes
*   **Description**: Detects iteration constructs (e.g., `for`, `while`) that perform mutable ledger storage operations (`env.storage().persistent().set()`) where the loop bounds are not statically determinable or are bounded by user-supplied input.
*   **Impact**: Prevents unpredictable storage write costs which are heavily metered in Soroban.

### 2.2. Excessive Host-Function Calls in Loops
*   **Description**: Identifies repeated invocations of Soroban environment host functions (e.g., `env.crypto().sha256()`, or repeated cross-contract calls) inside loops.
*   **Impact**: Host function calls require crossing the WASM-to-host boundary, which incurs a fixed CPU overhead per call. 

### 2.3. Nested Iteration over Persistent Storage
*   **Description**: Flags nested loops or complex recursive structures that contain ledger read or write operations.
*   **Impact**: Exponential cost growth leading to transaction failure due to budget limits.

### 2.4. Large Memory Allocation in Single Scope
*   **Description**: Warns against creating excessively large vectors, maps, or byte arrays inside a single function scope without bounds.
*   **Impact**: Soroban strictly limits WASM memory. Large allocations can hit memory metered limits quickly.

## 3. False-Positive Strategy
Since static analysis inherently struggles with dynamic runtime state, false positives are expected. The linter will mitigate developer friction through the following strategies:

### 3.1. Developer Overrides (Allow-listing)
Developers can silence specific lints using standard Rust attribute macros, ensuring they aren't blocked by intentional edge cases.
*   **Syntax**: `#[allow(soroban_cost::unbounded_storage_writes)]`
*   This approach aligns perfectly with Dylint and Rust's native `clippy` ecosystem, allowing overrides at the function, module, or statement level.

### 3.2. Confidence Scoring
Each lint will be assigned a confidence level (e.g., `High`, `Medium`, `Low`).
*   **High Confidence**: The lint triggers a compiler `warning` by default.
*   **Medium/Low Confidence**: The lint is set to `allow` by default, requiring the user to explicitly opt-in via a `#[warn(...)]` attribute if they want stricter checks.

### 3.3. Deterministic Heuristics
Lints will explicitly look for known Soroban SDK API paths (e.g., `soroban_sdk::Env::storage`) rather than generic variable names, reducing the chance of flagging unrelated application logic.

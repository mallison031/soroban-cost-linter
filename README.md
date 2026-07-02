# Soroban Cost Linter 📉

A static, write-time cost linter for Soroban smart contracts. 

While tools like Scout excel at catching security vulnerabilities and profilers measure resource usage *after* execution, `soroban-cost-linter` shifts cost-analysis left. It acts as a Dylint-based plugin that flags resource-expensive Rust patterns at compile time—before you deploy or even run a test.

**Pairs perfectly with Scout:** Use Scout for security, use this for cost.

## 🎯 The Problem
Unpredictable deployment and execution fees are a massive pain point in smart contract development. Heuristic patterns like unbounded storage writes, excessive host-function calls, and nested iteration over persistent storage quietly drain wallets. This tool catches those patterns in your editor or CI.

## 🌊 Drips Wave 7: Phase 1 (Research & Architecture)
This repository is currently participating in the Drips Wave 7 sprint. Before we write the core Rust logic, we are dedicating this sprint to empirical research and system architecture to ensure our heuristics map perfectly to Soroban's actual 11 metered resource dimensions.

We are actively seeking contributors for:
1. **Product Requirements & Core Lint Specifications:** Defining our first 3-5 lints and our false-positive mitigation strategy.
2. **Fee-Model Research:** Mapping proposed static lints directly to Soroban CPU/memory/storage metering costs.
3. **System Architecture:** Scaffolding the underlying Dylint plumbing and our `soroban-examples` testing matrix.

## 🚀 Getting Started
*(Implementation instructions, Cargo setup, and Dylint installation steps will be published here in Phase 2 / Wave 8).*

## 🤝 Contributing
If you are participating via the Drips Wave protocol, please ensure you are officially assigned to an issue on the Drips dashboard before opening a pull request. We adhere to a strict "Definition of Done" for all merged code.

use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// **What it does:** Detects iteration constructs that perform mutable ledger storage operations
    /// where the loop bounds are not statically determinable.
    ///
    /// **Why is this bad:** Unpredictable storage write costs which are heavily metered in Soroban.
    ///
    /// **Known problems:** None.
    pub UNBOUNDED_STORAGE_WRITES,
    Warn,
    "unbounded storage writes inside loops"
}

declare_lint_pass!(UnboundedStorage => [UNBOUNDED_STORAGE_WRITES]);

impl<'tcx> LateLintPass<'tcx> for UnboundedStorage {
    fn check_expr(&mut self, _cx: &LateContext<'tcx>, _expr: &'tcx rustc_hir::Expr<'tcx>) {
        // Implementation logic for analyzing loop bounds and storage access will go here in Phase 2
    }
}

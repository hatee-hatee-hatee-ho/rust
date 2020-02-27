use crate::{EarlyContext, EarlyLintPass, LintContext};
use rustc_errors::Applicability;
use rustc_span::Span;
use syntax::ast::{Block, StmtKind};

declare_lint! {
    pub REDUNDANT_SEMICOLON,
    Warn,
    "detects unnecessary trailing semicolons"
}

declare_lint_pass!(RedundantSemicolon => [REDUNDANT_SEMICOLON]);

impl EarlyLintPass for RedundantSemicolon {
    fn check_block(&mut self, cx: &EarlyContext<'_>, block: &Block) {
        let mut seq = None;
        for stmt in block.stmts.iter() {
            match (&stmt.kind, &mut seq) {
                (StmtKind::Semi(None), None) => seq = Some((stmt.span, false)),
                (StmtKind::Semi(None), Some(seq)) => *seq = (seq.0.to(stmt.span), true),
                (_, seq) => maybe_lint_redundant_semis(cx, seq),
            }
        }
        maybe_lint_redundant_semis(cx, &mut seq);
    }
}

fn maybe_lint_redundant_semis(cx: &EarlyContext<'_>, seq: &mut Option<(Span, bool)>) {
    if let Some((span, multiple)) = seq.take() {
        cx.struct_span_lint(REDUNDANT_SEMICOLON, span, |lint| {
            let (msg, rem) = if multiple {
                ("unnecessary trailing semicolons", "remove these semicolons")
            } else {
                ("unnecessary trailing semicolon", "remove this semicolon")
            };
            lint.build(msg)
                .span_suggestion(span, rem, String::new(), Applicability::MaybeIncorrect)
                .emit();
        });
    }
}

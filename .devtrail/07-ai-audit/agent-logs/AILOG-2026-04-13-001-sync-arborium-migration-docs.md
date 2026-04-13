---
id: AILOG-2026-04-13-001
title: Synchronize arborium migration documentation
status: accepted
created: 2026-04-13
agent: claude-code-v5.4
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: []
lines_changed: 90
files_modified:
  - CLAUDE.md
  - specs/001-code-metrics-library/checklists/requirements.md
  - specs/001-code-metrics-library/contracts/public-api.md
  - specs/001-code-metrics-library/data-model.md
  - specs/001-code-metrics-library/plan.md
  - specs/001-code-metrics-library/quickstart.md
  - specs/001-code-metrics-library/research.md
  - specs/001-code-metrics-library/spec.md
  - specs/001-code-metrics-library/tasks.md
  - src/error.rs
  - tests/source_analysis.rs
observability_scope: none
tags: [documentation, arborium, parser-migration, specs, devtrail]
related: []
---

# AILOG: Synchronize arborium migration documentation

## Summary

Updated the current-facing documentation to describe Arborist's `arborium`-backed parser integration and added explicit historical notes to the original spec/planning artifacts so they remain accurate records without pretending the dependency model never changed.

## Context

The live codebase now uses `arborium` instead of direct `tree-sitter` grammar crates, but several project documents still described the old dependency surface as if it were current. This created an avoidable mismatch between the published crate behavior, the feature-flag model, and the repository documentation.

## Actions Performed

1. Updated current-facing project docs to describe `arborium`, the `arborium/lang-*` feature mapping, and the TypeScript plus TSX split behind Arborist's single `typescript` feature.
2. Added supersession notes to the original specification, plan, research, and task artifacts so their historical `tree-sitter` references remain understandable instead of silently conflicting with the implemented code.
3. Normalized a few remaining source comments and DevTrail guidance entries to match the current parser integration wording.

## Modified Files

| File | Lines Changed (+/-) | Change Description |
|------|--------------------|--------------------|
| `CLAUDE.md` | +2/-2 | Updated active technologies and recent changes to describe the arborium-based parser stack |
| `specs/001-code-metrics-library/checklists/requirements.md` | +1/-1 | Clarified that `tree-sitter` references in the original spec are historical context |
| `specs/001-code-metrics-library/contracts/public-api.md` | +1/-1 | Updated the internal API note to reference `arborium::tree_sitter` |
| `specs/001-code-metrics-library/data-model.md` | +14/-14 | Replaced direct grammar crate references with arborium binding names and updated parser type wording |
| `specs/001-code-metrics-library/plan.md` | +2/-0 | Added a historical note describing the arborium supersession |
| `specs/001-code-metrics-library/quickstart.md` | +21/-17 | Updated prerequisites and feature flag reference to match the arborium integration model |
| `specs/001-code-metrics-library/research.md` | +2/-0 | Added a historical note identifying R1 and R6 as original dependency-validation context |
| `specs/001-code-metrics-library/spec.md` | +2/-0 | Added a historical note explaining that direct `tree-sitter` dependency references are superseded |
| `specs/001-code-metrics-library/tasks.md` | +2/-0 | Added a historical note explaining that direct dependency tasks are retained as milestones |
| `src/error.rs` | +1/-1 | Updated parse-error wording to refer to the underlying parser rather than direct `tree-sitter` integration |
| `tests/source_analysis.rs` | +3/-3 | Updated syntax-error comments to refer to the arborium-backed parser behavior |

## Decisions Made

1. Historical spec/planning artifacts were annotated rather than rewritten in full, to preserve traceability while still making the current implementation status explicit.
2. Current-facing docs were updated in place so library consumers see the real dependency model (`arborium`) instead of the original implementation plan.
3. The single `typescript` public feature continues to represent both TypeScript and TSX, and the docs now state that this is implemented via `lang-typescript` plus `lang-tsx` internally.

## Impact

- **Functionality**: No runtime behavior change; documentation and comments now match the implemented parser integration.
- **Performance**: N/A.
- **Security**: N/A.
- **Privacy**: N/A.
- **Environmental**: N/A.

## Verification

- [x] Code compiles without errors
- [x] Tests pass
- [ ] Manual review performed
- [ ] Security scan passed (if risk_level: high/critical)
- [ ] Privacy review completed (if handling PII)

## Additional Notes

- Accepted DevTrail audit logs were left intact; new migration context is documented in this AILOG instead of rewriting prior accepted records.
- Verification command executed: `cargo test`.
- The remaining `tree-sitter` mentions in the historical spec set are intentional and now explicitly marked as historical or architectural context.

---

<!-- Template: DevTrail | https://strangedays.tech -->

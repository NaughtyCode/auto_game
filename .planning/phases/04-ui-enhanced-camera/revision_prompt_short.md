<revision_context>
**Phase:** 4
**Mode:** revision

<files_to_read>
- .planning/phases/04-ui-enhanced-camera/*-PLAN.md (Existing plans)
</files_to_read>

**Checker issues summary:**
1. **Task completeness:** All tasks across all 8 plans have `<acceptance_criteria>` element but missing `<verify>` element. Rename `<acceptance_criteria>` to `<verify>` in every task (or add `<verify>` if needed).
2. **Wave mismatches:**
   - Plan 04-06: depends_on ["04-01-PLAN.md"] (wave 1) → expected wave 2, actual wave 5. Adjust wave to 2 or add dependencies.
   - Plan 04-08: depends_on ["04-02-PLAN.md"] (wave 2) → expected wave 3, actual wave 4. Adjust wave to 3 or add dependencies.
3. **Verification derivation:** Plan 04-01 must_haves.truths includes implementation-focused truth "UI theme exists with colors matching UI-SPEC". Reframe as user-observable truth like "UI panels use consistent colors per design spec".

**Instructions:**
Make targeted updates to address these specific issues.
Do NOT replan from scratch.
Return what changed.
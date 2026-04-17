<revision_context>
**Phase:** 4
**Mode:** revision

<files_to_read>
- .planning/phases/04-ui-enhanced-camera/*-PLAN.md (Existing plans)
</files_to_read>

**Checker issues:** 
```yaml
issues:
  - issue:
      plan: "04-01"
      dimension: "task_completeness"
      severity: "blocker"
      description: "All tasks missing <verify> element (Task 1, Task 2)"
      fix_hint: "Add <verify> element to each task with verification command, or rename <acceptance_criteria> to <verify> if appropriate"
  - issue:
      plan: "04-02"
      dimension: "task_completeness"
      severity: "blocker"
      description: "All tasks missing <verify> element (Task 1, Task 2, Task 3)"
      fix_hint: "Add <verify> element to each task with verification command, or rename <acceptance_criteria> to <verify> if appropriate"
  - issue:
      plan: "04-03"
      dimension: "task_completeness"
      severity: "blocker"
      description: "All tasks missing <verify> element (Task 1, Task 2, Task 3)"
      fix_hint: "Add <verify> element to each task with verification command, or rename <acceptance_criteria> to <verify> if appropriate"
  - issue:
      plan: "04-04"
      dimension: "task_completeness"
      severity: "blocker"
      description: "All tasks missing <verify> element (Task 1, Task 2, Task 3)"
      fix_hint: "Add <verify> element to each task with verification command, or rename <acceptance_criteria> to <verify> if appropriate"
  - issue:
      plan: "04-05"
      dimension: "task_completeness"
      severity: "blocker"
      description: "All tasks missing <verify> element (Task 1, Task 2)"
      fix_hint: "Add <verify> element to each task with verification command, or rename <acceptance_criteria> to <verify> if appropriate"
  - issue:
      plan: "04-06"
      dimension: "task_completeness"
      severity: "blocker"
      description: "All tasks missing <verify> element (Task 1, Task 2, Task 3)"
      fix_hint: "Add <verify> element to each task with verification command, or rename <acceptance_criteria> to <verify> if appropriate"
  - issue:
      plan: "04-07"
      dimension: "task_completeness"
      severity: "blocker"
      description: "All tasks missing <verify> element (Task 1, Task 2, Task 3)"
      fix_hint: "Add <verify> element to each task with verification command, or rename <acceptance_criteria> to <verify> if appropriate"
  - issue:
      plan: "04-08"
      dimension: "task_completeness"
      severity: "blocker"
      description: "All tasks missing <verify> element (Task 1, Task 2, Task 3)"
      fix_hint: "Add <verify> element to each task with verification command, or rename <acceptance_criteria> to <verify> if appropriate"
  - issue:
      plan: "04-06"
      dimension: "dependency_correctness"
      severity: "blocker"
      description: "Wave mismatch: depends_on ['04-01-PLAN.md'] (wave 1) => expected wave 2, actual wave 5"
      fix_hint: "Adjust wave to 2 or add missing dependencies to justify wave 5"
  - issue:
      plan: "04-08"
      dimension: "dependency_correctness"
      severity: "blocker"
      description: "Wave mismatch: depends_on ['04-02-PLAN.md'] (wave 2) => expected wave 3, actual wave 4"
      fix_hint: "Adjust wave to 3 or add missing dependencies to justify wave 4"
  - issue:
      plan: "04-01"
      dimension: "verification_derivation"
      severity: "warning"
      description: "must_haves.truths are implementation-focused ('UI theme exists with colors matching UI-SPEC')"
      fix_hint: "Reframe as user-observable truth: 'UI panels use consistent colors per design spec'"
```
</revision_context>

<instructions>
Make targeted updates to address checker issues.
Do NOT replan from scratch unless issues are fundamental.
Return what changed.
</instructions>
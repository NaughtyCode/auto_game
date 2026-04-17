---
phase: 1
slug: environment-camera-setup
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-17
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Godot unit testing (GUT/GDUnit) |
| **Config file** | `addons/gut/gut_config.gd` or `addons/gdunit/gdunit_config.gd` |
| **Quick run command** | `godot --headless --script addons/gut/gut_cmdln.gd` |
| **Full suite command** | `godot --headless --script addons/gut/gut_cmdln.gd -gtest=*` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `{quick run command}`
- **After every plan wave:** Run `{full suite command}`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| {N}-01-01 | 01 | 1 | ENV-01 | — | N/A | unit | `godot --headless --script addons/gut/gut_cmdln.gd -gtest=test_camera_controls` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `test/test_camera_controls.gd` — stubs for ENV-01, CAM-01
- [ ] `test/test_environment_boundaries.gd` — stubs for ENV-03, CAM-03
- [ ] `addons/gut/` — install GUT framework if not present
- [ ] `addons/gut/gut_config.gd` — configure test runner

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Camera movement "feels smooth" | CAM-01 | Subjective experience requires human perception | Run game, move camera with mouse/keyboard, assess responsiveness |
| Environment "plausible space for insect activity" | ENV-03 | Visual/design judgement | Explore environment, verify terrain features visible and appropriate scale |

*If none: "All phase behaviors have automated verification."*

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
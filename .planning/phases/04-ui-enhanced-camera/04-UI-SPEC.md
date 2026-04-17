---
phase: 4
slug: ui-enhanced-camera
status: draft
shadcn_initialized: false
preset: none
created: 2026-04-17
reviewed_at: 
---

# Phase 4 — UI Design Contract

> Visual and interaction contract for frontend phases. Generated manually to unblock planning.

---

## Design System

| Property | Value |
|----------|-------|
| Tool | Godot UI |
| Preset | not applicable |
| Component library | Godot Control nodes |
| Icon library | none |
| Font | Default Godot font (interchangeable) |

---

## Spacing Scale

Declared values (must be multiples of 4):

| Token | Value | Usage |
|-------|-------|-------|
| xs | 4px | Icon gaps, inline padding |
| sm | 8px | Compact element spacing |
| md | 16px | Default element spacing |
| lg | 24px | Section padding |
| xl | 32px | Layout gaps |
| 2xl | 48px | Major section breaks |
| 3xl | 64px | Page-level spacing |

Exceptions: none

---

## Typography

| Role | Size | Weight | Line Height |
|------|------|--------|-------------|
| Label (small) | 14px | regular (400) | 1.5 |
| Body | 16px | regular (400) | 1.5 |
| Heading | 20px | bold (700) | 1.2 |
| Display (large readouts) | 28px | bold (700) | 1.2 |

---

## Color

| Role | Value | Usage |
|------|-------|-------|
| Dominant (60%) | #1E1E1E | UI background, overlay surfaces |
| Secondary (30%) | #2D2D2D | Panels, info panels, sidebars |
| Accent (10%) | #4CAF50 | Interactive elements, selection highlights, active tracking |
| Destructive | #F44336 | Any remove actions (none in this phase) |

Accent reserved for: Interactive buttons, selection highlights, active tracking indicators only

---

## Copywriting Contract

| Element | Copy |
|---------|------|
| Primary CTA | Track Insect |
| Insect info panel title | Insect Info |
| Observation hint heading | Observation Hint |
| Camera controls heading | Camera Controls |
| Game status heading | Game Status |
| Empty state (no selection) | Select an insect to see information |
| Error state (tracking failed) | Could not track insect. Try selecting another insect. |
| Destructive confirmation | (none in this phase) |

**Button labels:** "Track Insect", "Free Camera", "Zoom In", "Zoom Out", "Pan Left", "Pan Right", "Rotate"

---

## Visuals

**Focal point:** Insect info panel is primary visual anchor when insect is selected.

**Visual hierarchy:**
1. **Selected insect info:** Species, status, tracking button (largest, accent color when tracking)
2. **Observation hints:** Contextual guidance (secondary emphasis)
3. **Camera controls:** Camera mode toggle and movement controls
4. **Game status:** Time, observation count (smallest visual weight)

**Layout placement:**
- **Insect info panel:** Top-right corner (appears when insect selected)
- **Observation hints:** Bottom-right corner (contextual, appears based on behaviors)
- **Camera controls:** Bottom-center panel
- **Game status:** Top-left corner (always visible)
- **Tracking indicator:** Center-screen reticle when tracking active

---

## Registry Safety

| Registry | Blocks Used | Safety Gate |
|----------|-------------|-------------|
| shadcn official | none | not required |
| Godot Asset Library | none | no blocks used — vetting not required |

---

## Checker Sign-Off

- [ ] Dimension 1 Copywriting: PENDING
- [ ] Dimension 2 Visuals: PENDING
- [ ] Dimension 3 Color: PENDING
- [ ] Dimension 4 Typography: PENDING
- [ ] Dimension 5 Spacing: PENDING
- [ ] Dimension 6 Registry Safety: PENDING

**Approval:** draft
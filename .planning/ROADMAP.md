# Roadmap: Milestone v1.0 Core Simulation

**Milestone:** v1.0 Core Simulation  
**Goal:** Build the core insect simulation system for observing insect behaviors in a natural 3D environment.  
**Granularity:** standard  
**Coverage:** 20/20 requirements mapped ✓

## Phases

- [ ] **Phase 1: Environment & Camera Setup** - Player can navigate a basic 3D environment with smooth, responsive camera controls
- [ ] **Phase 2: Basic Insect Implementation** - Player can observe multiple insect species with distinct appearances and basic animations in the environment
- [ ] **Phase 3: Simulation Behaviors** - Player can observe insects exhibiting natural behaviors: moving, seeking food, interacting, and responding to stimuli
- [ ] **Phase 4: UI & Enhanced Camera** - Player can access insect information, receive observation guidance, and track insects with camera

## Phase Details

### Phase 1: Environment & Camera Setup
**Goal:** Player can navigate a basic 3D environment with smooth, responsive camera controls  
**Depends on:** Nothing (first phase)  
**Requirements:** ENV-01, ENV-03, CAM-01, CAM-03  
**Success Criteria** (what must be TRUE):
  1. Player can move the camera freely (pan, rotate, zoom) in 3D space
  2. Player can explore a grassland/soil environment with natural boundaries
  3. Camera movement feels smooth and responsive to input
  4. Environment provides a plausible space for insect activity (visible terrain features)
**Plans:** TBD

### Phase 2: Basic Insect Implementation
**Goal:** Player can observe multiple insect species with distinct appearances and basic animations in the environment  
**Depends on:** Phase 1 (needs environment and camera)  
**Requirements:** INSECT-01, INSECT-04, INSECT-05, ENV-02, CAM-04  
**Success Criteria** (what must be TRUE):
  1. Player can see at least three different insect species (e.g., cricket, ant, spider) in the environment
  2. Player can identify each insect type by its visual appearance
  3. Player can see basic insect animations (walking, idle)
  4. Player can see insects positioned appropriately on terrain surfaces
  5. Camera automatically maintains a reasonable viewing distance when insects are present
**Plans:** TBD

### Phase 3: Simulation Behaviors
**Goal:** Player can observe insects exhibiting natural behaviors: moving, seeking food, interacting, and responding to stimuli  
**Depends on:** Phase 2 (needs insects present)  
**Requirements:** SIM-01, SIM-02, SIM-03, SIM-04, INSECT-02, INSECT-03  
**Success Criteria** (what must be TRUE):
  1. Player can observe insects moving with natural, species-appropriate movement patterns
  2. Player can observe insects seeking and consuming food sources placed in the environment
  3. Player can observe basic interactions between different insects (e.g., approaching, avoiding)
  4. Player can observe insects responding to environmental stimuli (e.g., light changes, obstacles)
  5. Player can see different feeding behaviors for different insect species
  6. Player can distinguish insect species by their movement patterns
**Plans:** TBD

### Phase 4: UI & Enhanced Camera
**Goal:** Player can access insect information, receive observation guidance, and track insects with camera  
**Depends on:** Phase 3 (needs behaviors to observe), Phase 2 (needs insects to track)  
**Requirements:** UI-01, UI-02, UI-03, UI-04, CAM-02  
**Success Criteria** (what must be TRUE):
  1. Player can select/focus on an insect and see its information (species, status)
  2. Player receives contextual hints about interesting insect behaviors to observe
  3. Player can access camera controls via on-screen UI elements
  4. Player can see basic game status (time, observation count, etc.)
  5. Player can track a specific insect with the camera follow mode
**Plans:** TBD  
**UI hint:** yes

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Environment & Camera Setup | 0/? | Not started | - |
| 2. Basic Insect Implementation | 0/? | Not started | - |
| 3. Simulation Behaviors | 0/? | Not started | - |
| 4. UI & Enhanced Camera | 0/? | Not started | - |

---
*Roadmap created: 2026-04-17 for Milestone v1.0 Core Simulation*
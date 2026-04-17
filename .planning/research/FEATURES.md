# Feature Landscape

**Domain:** 3D turn-based cricket fighting game
**Researched:** 2026-04-17
**Confidence:** MEDIUM (Based on project requirements and general game design knowledge; web research tools unavailable)

## Table Stakes

Features users expect in any turn-based combat game. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Clear turn indicator | Players need to know whose turn it is and when they can act | Low | Simple UI element showing current actor; could be player vs AI alternation or speed-based initiative |
| Action point (AP) system | Core mechanic of the game; players expect to manage limited resources per turn | Medium | Need AP counter, action costs, visual feedback; AP refreshes each turn |
| Health bars | Players must see remaining health of combatants to make strategic decisions | Low | Standard UI element with numeric display; color change at low health |
| Basic attack action | Fundamental combat action; expected in any battle system | Low | Simple damage calculation based on attack vs defense; animation and sound |
| Defend action | Standard option to reduce incoming damage; adds tactical depth | Low | Could be a flat damage reduction or temporary defense boost; may cost AP |
| Special skill action | Players expect characters to have unique abilities beyond basic attacks | Medium | Need design of 1-2 special skills per cricket type; balance with AP costs |
| AI opponent | Single-player game requires competent AI to provide challenge | High | AI must make sensible decisions based on AP, health, position; avoid being too predictable or unfair |
| Win/lose conditions | Clear victory (defeat all enemies) and defeat (player cricket dies) conditions | Low | Simple state tracking and end-of-battle screen; option to restart |
| Visual feedback for actions | Animations, hit effects, and screen shakes to make actions feel impactful | Medium | Requires animation system and VFX; can start with simple transforms |
| Sound effects | Audio feedback for attacks, hits, UI interactions, and background music | Medium | Need sound assets and audio system integration; ambient sounds for environment |
| Action UI | Buttons or menu showing available actions and their AP costs | Medium | UI design and integration with input system; disable actions when insufficient AP |
| Camera control (3D) | Players expect to rotate/zoom camera to view battlefield from different angles | Medium | Godot camera controller with orbit/pan; optional zoom limits |
| Pause/quit options | Standard game functionality for player control | Low | Simple pause menu with resume/quit to main menu |
| Tutorial/tooltips | New players need explanation of controls and game mechanics | Low | Can be simple tooltips or a guided tutorial sequence; explain AP, actions, win condition |
| Character attributes display | Players need to see stats (power, speed, defense, HP) and how they affect combat | Low | UI panel showing numeric values; maybe with tooltips explaining each attribute |
| Turn order display | If using speed-based initiative, show queue of upcoming turns | Medium | Optional; can be added later; increases clarity |
| Environmental context | Players need to see what environment they're in (grass, sand, etc.) | Low | Visual backdrop and name label; possibly affect gameplay |

## Differentiators

Features that set this cricket fighting game apart. Not expected, but valued.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Anthropomorphic cricket characters | Visual charm and personality; makes crickets relatable and memorable | High | Requires 3D models with clothing/hats, rigging, animations; key to humorous tone |
| Environment interactions | Battle arenas with interactive elements (grass to hide in, rocks to climb) add tactical depth | High | Need physics/collision detection and interaction system; can start with simple stat modifiers |
| Cricket-specific abilities | Abilities based on real cricket traits (jump high, chirp distract, antennae sense) enhance thematic coherence | Medium | Design abilities that feel authentic to crickets; e.g., "Leap" ignores terrain, "Chirp" lowers opponent accuracy |
| Multiple battle environments | Variety keeps gameplay fresh; different environments offer different strategic options | Medium | Each environment needs unique visuals and interactives; start with 2-3 environments |
| Attribute system with visible impact | Power, speed, defense, HP stats that clearly affect combat decisions | Low-Medium | Stats must be balanced and their effects transparent; speed could affect turn order |
| Strategic AP management | Deeper than simple action selection; players can save AP for defense or combo moves | Medium | AP system design with carry-over or bonus mechanics; encourage planning ahead |
| Humorous animations/sounds | Adds lighthearted fun; crickets doing human-like actions with insect quirks | Medium | Requires creative animation and sound design; e.g., cricket tipping hat before attack |
| Day/night or weather effects | Visual variety and potential gameplay modifiers (e.g., night reduces accuracy) | Medium | Lighting system and effect handlers; could be cosmetic only initially |
| Cricket species variety | Different cricket types (e.g., mole cricket, field cricket) with different base stats | Low | Simple stat variations, possibly different models; adds replayability |
| Morale system (optional) | Cricket's morale affects performance; adds another layer of strategy | High | Adds complexity; could be deferred; might be represented as a temporary stat boost |
| Lighthearted, humorous tone | Differentiates from serious RPGs; appeals to casual players | Low | Achieved through character design, animations, sound effects, and UI style |
| Fast-paced combat with quick turns | Reduces downtime; keeps engagement high | Low | Design turns to be snappy with minimal waiting; auto-resolve animations quickly |
| Interactive environmental effects on gameplay | Terrain affects attributes or actions (e.g., sand reduces speed) | Medium | Adds layer of strategy; players must adapt to environment; can be simple stat modifiers |

## Anti-Features

Features to explicitly NOT build.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Complex skill trees | Out of scope; adds development time and complexity without core value | Stick to simple attribute system and a few fixed skills |
| Equipment system | Out of scope; shifts focus from tactical combat to inventory management | Use inherent cricket attributes and environmental interactions |
| Story mode | Out of scope; narrative development distracts from core combat loop | Focus on pure battle gameplay; maybe add simple "arcade" mode |
| Multiplayer (online) | Out of scope; networking adds significant complexity and testing burden | Single-player vs AI only; consider local hotseat later if needed |
| Grindy progression | Players shouldn't need to grind to enjoy balanced combat | Keep progression minimal (maybe unlock new cricket types via wins) |
| Pay-to-win microtransactions | Not appropriate for a small indie game; damages player trust | Monetize via one-time purchase (if at all) |
| Real-time elements | Would conflict with turn-based nature; confuse players | Keep strictly turn-based; consider "quick time events" only if they fit |
| Overly complicated status effects | Adds cognitive load and balance issues | Limit to simple buffs/debuffs (e.g., defense up, speed down) |
| Random loot boxes | Ethical concerns, predatory design, regulatory risk | Avoid entirely; if monetization needed, use direct cosmetic purchases |
| Complex stat systems | Overwhelms casual players, difficult to balance | Stick to four core attributes; avoid secondary stats like critical chance, evasion, etc. |

## Feature Dependencies

```
Core combat system (turn manager, AP system) → Action UI (requires AP data)
Core combat system → AI opponent (needs game state)
3D models and animations → Visual feedback for actions
Attribute system → Special skill actions (skills use attributes)
Environment interactions → Physics/collision system
Camera control → 3D scene setup
Turn order system → Speed attribute (if speed affects turn order)
Interactive environments → Environmental effects on gameplay
```

**Dependency Notes:**
- **Combat actions require Character attributes:** Actions need to reference attributes for damage calculation, hit chance, etc.
- **Action point system requires Turn order system:** AP allocation per turn depends on turn sequencing; need to define when AP refreshes.
- **Interactive environments enhance Combat actions:** Environment can modify action outcomes (e.g., defense bonus in grass).
- **AI opponent requires Combat actions system:** AI needs to evaluate available actions and their effects.
- **Visual/audio feedback requires Animation system:** Need animation rigging and sound effect triggers for each action.
- **Camera control independent:** Can be added anytime after 3D scene is set up.

## MVP Recommendation

Prioritize:
1. **Core combat system** (turn manager, AP, basic attack/defend) - table stakes foundation
2. **AI opponent** (basic AI) - required for single-player gameplay
3. **Anthropomorphic cricket characters** (one cricket model with basic animations) - key differentiator

Defer: 
- **Multiple cricket species**: Start with one balanced cricket type
- **Day/night effects**: Add visual polish after core gameplay works
- **Morale system**: Too complex for MVP

**Note on environment interactions:** User explicitly requested interactive environments (grass, sand, indoor with interactive elements). While research suggests deferring complex interactions for MVP, the user's vision includes environment effects on gameplay. Consider including simple terrain modifiers (e.g., sand reduces speed, grass provides defense bonus) as part of MVP to align with user requirements.

**MVP Feature Set:**
- Turn order system (simple player-AI alternation)
- Four combat actions (normal attack, heavy attack, defense, special skill)
- Health and action point resources
- Basic UI showing HP, AP, actions
- One cricket character model with basic animations
- One environment (grass) with simple interactive elements or terrain modifiers
- AI that uses random actions (upgrade later)
- Win/lose conditions and restart
- Camera rotation controls (simple)

## Sources

- Project requirements from `.planning/PROJECT.md`
- General game design knowledge of turn-based combat systems
- Analysis of similar games (e.g., XCOM, Final Fantasy Tactics, Pokemon)
- Common turn-based combat game patterns (training data)
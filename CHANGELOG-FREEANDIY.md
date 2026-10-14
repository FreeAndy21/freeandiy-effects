<!-- Modified-by: FreeANDIY (Free Andy Agency), 2026. New file, not part of upstream EffectCraft. -->
# FreeANDIY Effects: changes from upstream EffectCraft

This file lists what the FreeANDIY fork changes in EffectCraft (Apache-2.0 section 4(b)). The
exact changes are generated from `freeandiy/` (see `NOTICE`); `git diff upstream-main main`
shows every changed file.

## Unreleased

### Branding
- Product name **FreeANDIY Effects** everywhere users see it: window title, menus, About, the Home
  screen, messages, translations, installers, desktop entries, the web page and its manifest.
- Application id `agency.freeandy.effects` (was `ai.storyteller.effectcraft`), so FreeANDIY builds
  install next to EffectCraft.
- Executables `freeandiy-effects` and `freeandiy-effects-cli`; the cargo packages keep their names.
- New app icon set, in-app mark and web icon (the angel Andy, Free Andy Agency) and DMG background.
- The three themes keep their neutral greys with FreeANDIY's stamp-rose accent, nearly square
  windows and no soft shadows.
- UI font IBM Plex Sans (OFL 1.1) on the desktop, upstream's Inter as its fallback.

### Changed
- Home and Help list the other FreeANDIY apps (`help.sibling` takes `photo`, `vector`, `film`,
  `light`, `pdf`, `layout`).

### Removed
- The ArtCraft name, wordmark and mark (`docs/brand/`, the header, About), as their terms require
  of forks.
- The ArtCraft Discord button and links (header, Help menu, About, Home).
- Upstream's release and platform workflows.

### Added
- About: attribution to upstream EffectCraft and the Free Andy Agency seal.
- `NOTICE` section and `ATTRIBUTION.md` rows for FreeANDIY's assets and fonts; packages now ship
  `NOTICE`, `ATTRIBUTION.md`, the font licences and the brand terms.

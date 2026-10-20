//! Project links (shown in Help ▸, the About dialog, the start screen and the header).

pub const APP_NAME: &str = "FreeANDIY Effects";
pub const APP_SLUG: &str = "effectcraft";
pub const WEBSITE: &str = "https://freeandiy-web.vercel.app";
pub const APP_PAGE: &str = "https://freeandiy-web.vercel.app/effects";
pub const GITHUB: &str = "https://github.com/FreeAndy21/freeandiy-effects";
pub const ISSUES: &str = "https://github.com/FreeAndy21/freeandiy-effects/issues";

/// The other FreeANDIY apps: (name, slug on the website and in the `freeandiy-<slug>` repository).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("FreeANDIY Photo", "photo"),
    ("FreeANDIY Vector", "vector"),
    ("FreeANDIY Film", "film"),
    ("FreeANDIY Light", "light"),
    ("FreeANDIY PDF", "pdf"),
    ("FreeANDIY Layout", "layout"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/FreeAndy21/freeandiy-{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("{WEBSITE}/{slug}")
}

//! Vyntaris: the desktop's colours, as the Craft theme the Vyntaris shell writes
//! (`~/.config/ArtCraft/themes/library/vyntaris.json`). Read again when the file changes, at most once
//! a second; none (another desktop) leaves VDoc's own theme.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use craft::theme::ThemeDef;

struct Cache {
    looked: Option<Instant>,
    stamp: Option<SystemTime>,
    theme: Option<Arc<ThemeDef>>,
}

static CACHE: Mutex<Cache> = Mutex::new(Cache { looked: None, stamp: None, theme: None });

fn path() -> Option<PathBuf> {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(config.join("ArtCraft/themes/library/vyntaris.json"))
}

/// The Vyntaris theme, when the shell wrote one that parses.
pub fn current() -> Option<Arc<ThemeDef>> {
    let mut c = CACHE.lock().ok()?;
    if c.looked.is_some_and(|t| t.elapsed() < Duration::from_secs(1)) {
        return c.theme.clone();
    }
    c.looked = Some(Instant::now());
    let path = path()?;
    let stamp = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
    if stamp != c.stamp {
        c.stamp = stamp;
        c.theme = stamp
            .and_then(|_| std::fs::read(&path).ok())
            .and_then(|bytes| craft::theme::format::parse(&bytes).ok())
            .map(|file| Arc::new(file.theme));
    }
    c.theme.clone()
}

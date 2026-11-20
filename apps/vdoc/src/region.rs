//! The system region new documents follow (#838): the paper File › New starts on, the default
//! measurement unit and the decimal separator.
//!
//! `VDOC_REGION` names a region outright; otherwise it is the system's regional format,
//! read once for desktop and web by [`vdoc_ui_egui::i18n::system_region`].

/// The system region.
pub fn system() -> vdoc_engine::Region {
    let named = std::env::var("VDOC_REGION").ok().filter(|v| !v.trim().is_empty());
    // Runs without preferences (agents' and screenshot runs) behave the same on every machine:
    // en-US unless `VDOC_REGION` says otherwise.
    if std::env::var_os("VDOC_NO_PREFS").is_some() && named.is_none() {
        return vdoc_engine::Region::default();
    }
    if let Some(tag) = named {
        return vdoc_engine::Region::from_tag(&tag);
    }
    vdoc_ui_egui::i18n::system_region()
}

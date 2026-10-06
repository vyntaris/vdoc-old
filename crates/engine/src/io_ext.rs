//! Format crates registered with the I/O dispatcher.

use vdoc_doc::Document;

/// Open a format other than plain text / JSON.
pub fn open(ext: &str, _bytes: &[u8]) -> Option<Result<Document, String>> {
    match ext {
        _ => None,
    }
}

/// Save to a format other than plain text / JSON.
pub fn save(ext: &str, doc: &Document) -> Option<Result<Vec<u8>, String>> {
    match ext {
        "png" => Some(render_png(doc, 0, 2.0)),
        _ => None,
    }
}

/// Render page `page` as PNG at `scale` px/pt.
pub fn render_png(doc: &Document, page: usize, scale: f32) -> Result<Vec<u8>, String> {
    let l = vdoc_layout::layout(doc, &mut vdoc_layout::LayoutCache::new(), &Default::default());
    let p = l.pages.get(page).ok_or_else(|| format!("no page {}", page + 1))?;
    let img = vdoc_render::render_page(doc, p, scale, &Default::default());
    Ok(img.to_png())
}

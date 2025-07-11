pub fn html(groups: &[Vec<&FileEntry>], path: &Path) -> anyhow::Result<()> {
    let mut out = String::new();
    out.push_str("<!doctype html><title>Deduper Report</title><style>body{font-family:sans-serif}</style>");
    let mut saved = 0u64;
    for g in groups {
        let size = std::fs::metadata(&g[0].path)?.len();
        saved += size * (g.len() as u64 - 1);
        out.push_str(&format!("<h3>{} duplicates ({} bytes each)</h3><ul>",
                              g.len(), size));
        for f in g { out.push_str(&format!("<li>{}</li>", f.path)); }
        out.push_str("</ul>");
    }
    out.push_str(&format!("<hr><b>Potential savings: {} MB</b>",
                          saved / 1_048_576));
    std::fs::write(path, out)?;
    Ok(())
}

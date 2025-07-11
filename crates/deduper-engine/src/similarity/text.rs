use triple_accel::levenshtein::levenshtein;
pub fn is_similar(a: &str, b: &str, max_dist: u32) -> bool {
    levenshtein(a.as_bytes(), b.as_bytes()) <= max_dist
}

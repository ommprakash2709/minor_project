use img_hash::{HasherConfig, HashAlg};

pub fn phash(path: &std::path::Path) -> anyhow::Result<String> {
    let img = image::open(path)?;
    let hasher = HasherConfig::new().hash_alg(HashAlg::Gradient).to_hasher();
    Ok(hasher.hash_image(&img).to_base64())
}

pub fn hamming(a: &str, b: &str) -> anyhow::Result<u32> {
    use img_hash::ImageHash;
    let ha = ImageHash::from_base64(a)?;
    let hb = ImageHash::from_base64(b)?;
    Ok(ha.dist(&hb))
}

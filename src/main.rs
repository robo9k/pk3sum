use md4::Digest as _;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pak0 = std::fs::File::open("/usr/lib/ioquake3/baseq3/pak0.pk3")?;
    println!("pak0 {:?}", pak0);

    let mut zip = zip::ZipArchive::new(pak0)?;
    //println!("zip {:?}", zip);

    let zipfiles = zip.len();
    let mut crcs: Vec<std::ffi::c_int> = Vec::with_capacity(zipfiles);

    for idx in 0..zipfiles {
        let pak = zip.by_index_raw(idx)?;
        if pak.size() > 0 {
            crcs.push(pak.crc32() as std::ffi::c_int);
        }
    }

    let mut hasher = md4::Md4::new();
    let (head, body, tail) = unsafe { crcs.align_to::<u8>() };
    assert!(head.is_empty());
    assert!(tail.is_empty());

    hasher.update(body);
    let digest = hasher.finalize();
    println!("digest {:?}", digest);

    let digest_0 = std::ffi::c_int::from_ne_bytes(digest[0..4].try_into()?);
    let digest_1 = std::ffi::c_int::from_ne_bytes(digest[4..8].try_into()?);
    let digest_2 = std::ffi::c_int::from_ne_bytes(digest[8..12].try_into()?);
    let digest_3 = std::ffi::c_int::from_ne_bytes(digest[12..16].try_into()?);

    let checksum: std::ffi::c_int = digest_0 ^ digest_1 ^ digest_2 ^ digest_3;
    println!("checksum {:?}", checksum);
    assert_eq!(1566731103, checksum);

    Ok(())
}

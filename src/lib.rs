pub fn find_center_width<R: std::io::Read>(img: R) -> Option<f32> {
    let mut decoder = png::Decoder::new(img);
    // DO NOT STRIP ALPHA
    decoder.set_transformations(png::Transformations::IDENTITY);
    let (info, mut reader) = decoder.read_info().unwrap();

    let bytes_per_pixel = reader.info().bytes_per_pixel();

    let mut last_row: Vec<u8> = vec![];
    while let Ok(Some(row)) = reader.next_row() {
        last_row = row.to_vec();
    }

    let chunked: Vec<&[u8]> = last_row.chunks(bytes_per_pixel).collect();

    let mut first_column_with_color = None;
    for (i, c) in chunked.iter().enumerate() {
        if let Some(last_byte) = c.iter().last() {
            if *last_byte != 0 {
                first_column_with_color = Some(i);
            }
        }
    }
    first_column_with_color.map(|c| (c as f32) / (info.width as f32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    const FILES: &[&str] = &[
        "tests/forest_tree_a.png",
        "tests/forest_tree_b.png",
        "tests/forest_tree_c.png",
        "tests/forest_tree_d.png",
        "tests/forest_tree_e.png",
        "tests/forest_tree_f.png",
    ];

    #[test]
    fn various_files() {
        for f in FILES.iter() {
            let result = find_center_width(File::open(f).unwrap());
            assert!(result.is_some());
            let actual = result.unwrap();
            assert!(actual > 0.0);
        }
    }
}

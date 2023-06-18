struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn main() {
    let width = 1024;
    let height = 576;

    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}

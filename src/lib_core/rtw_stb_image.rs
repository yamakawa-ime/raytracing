use image::RgbImage;

pub struct RtwImage {
    data: RgbImage,
}

impl RtwImage {
    pub fn load(path: &str) -> Self {
        let img = image::open(path).expect("failed to load image").to_rgb8();

        Self { data: img }
    }

    pub fn width(&self) -> u32 {
        self.data.width()
    }

    pub fn height(&self) -> u32 {
        self.data.height()
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> [u8; 3] {
        let p = self.data.get_pixel(x, y);
        [p[0], p[1], p[2]]
    }
}

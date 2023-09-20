pub fn blur_image(image: &mut image::DynamicImage, percentage: f32) -> Result<(), blurslice::SliceSizeError> {
    let width = image.width() as usize;
    let height = image.height() as usize;

    // INFO: `/ 15.0` is a bit of a magick number that I felt was just right
    let blur_radius = (std::cmp::min(width, height) as f32 / 15.0) * percentage;

    let rgb_image = image.as_mut_rgb8().unwrap();

    let samples = rgb_image.as_flat_samples_mut();
    blurslice::gaussian_blur_bytes::<3>(samples.samples, width, height, blur_radius)
}

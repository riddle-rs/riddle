use ::image::DynamicImage;

pub trait ImageImageExt {
	fn image_from_dynimage(img: DynamicImage) -> Self;
	fn image_rgbaimage(&self) -> &::image::RgbaImage;
}

use apriltag::{image_buf::DEFAULT_ALIGNMENT_U8, Image};
use image::{
    flat::{FlatSamples, SampleLayout},
    ColorType, ImageBuffer, Luma, Pixel,
};
use std::ops::Deref;

pub trait ImageExt {
    fn from_flat_samples<Buffer>(from: &FlatSamples<Buffer>) -> Self
    where
        Buffer: AsRef<[u8]>;

    fn to_flat_samples(&self) -> FlatSamples<Vec<u8>>;

    fn from_image_buffer<Container>(from: &ImageBuffer<Luma<u8>, Container>) -> Self
    where
        Container: Deref<Target = [u8]>;

    fn to_image_buffer(&self) -> ImageBuffer<Luma<u8>, Vec<u8>>;
}

impl ImageExt for Image {
    fn from_flat_samples<Buffer>(from: &FlatSamples<Buffer>) -> Self
    where
        Buffer: AsRef<[u8]>,
    {
        match from.color_hint {
            Some(ColorType::L8) => (),
            _ => panic!("color type {:?} is not supported", from.color_hint),
        }

        let SampleLayout { width, height, .. } = from.layout;
        let mut image =
            Image::zeros_with_alignment(width as usize, height as usize, DEFAULT_ALIGNMENT_U8)
                .unwrap();
        let stride = image.stride();

        let sample_iter = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .map(|(x, y)| *from.get_sample(0, x, y).unwrap());
        let buffer_index_iter = (0..height)
            .flat_map(|y| (0..width).map(move |x| (x as usize, y as usize)))
            .map(|(x, y)| y * stride + x);

        buffer_index_iter
            .zip(sample_iter)
            .for_each(|(buffer_index, sample)| {
                image.as_mut()[buffer_index] = sample;
            });

        image
    }

    fn to_flat_samples(&self) -> FlatSamples<Vec<u8>> {
        let width = self.width();
        let height = self.height();
        let stride = self.stride();

        let mut samples = vec![];
        samples.extend_from_slice(self.as_ref());

        FlatSamples {
            samples,
            layout: SampleLayout {
                channels: 1,
                channel_stride: 1,
                width: width as u32,
                width_stride: 1,
                height: height as u32,
                height_stride: stride,
            },
            color_hint: Some(ColorType::L8),
        }
    }

    fn from_image_buffer<Container>(from: &ImageBuffer<Luma<u8>, Container>) -> Self
    where
        Container: Deref<Target = [u8]>,
    {
        let width = from.width() as usize;
        let height = from.height() as usize;
        let mut image = Image::zeros_with_alignment(width, height, DEFAULT_ALIGNMENT_U8).unwrap();

        from.enumerate_pixels().for_each(|(x, y, pixel)| {
            let component = pixel.channels()[0];
            image[(x as usize, y as usize)] = component;
        });
        image
    }

    fn to_image_buffer(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let width = self.width();
        let height = self.height();
        ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            Luma::from([self[(x as usize, y as usize)]])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{
        flat::{FlatSamples, SampleLayout},
        ColorType, ImageBuffer, Luma,
    };

    #[test]
    fn image_clone() {
        let width = 80;
        let height = 60;
        let from_image = diagonal_image(width, height);
        let to_image = from_image.clone();
        assert_eq!(from_image.width(), to_image.width());
        assert_eq!(from_image.height(), to_image.height());
        assert_eq!(from_image.stride(), to_image.stride());

        // invert color of to_image
        (0..height)
            .into_iter()
            .flat_map(|y| (0..width).into_iter().map(move |x| (x, y)))
            .for_each(|(x, y)| {
                assert_eq!(from_image[(x, y)], to_image[(x, y)]);
            });
    }

    #[test]
    fn convert_flat_samples_vs_image() {
        let width = 64;
        let height = 28;
        let stride = ((width - 1) / DEFAULT_ALIGNMENT_U8 + 1) * DEFAULT_ALIGNMENT_U8;

        let flat_from = {
            let mut samples = Vec::<u8>::new();
            (0..height).into_iter().for_each(|y| {
                let mut row = vec![];
                row.resize(stride, 0);
                (0..width).into_iter().for_each(|x| {
                    if x == y {
                        row[x] = 255;
                    }
                });
                samples.append(&mut row);
            });
            assert_eq!(samples.len(), height * stride);

            FlatSamples {
                samples,
                layout: SampleLayout {
                    channels: 1,
                    channel_stride: 1,
                    width: width as u32,
                    width_stride: 1,
                    height: height as u32,
                    height_stride: stride,
                },
                color_hint: Some(ColorType::L8),
            }
        };

        let image = Image::from_flat_samples(&flat_from);
        (0..height)
            .into_iter()
            .flat_map(|y| (0..width).into_iter().map(move |x| (x, y)))
            .for_each(|(x, y)| {
                if x == y {
                    assert_eq!(image[(x, y)], 255);
                } else {
                    assert_eq!(image[(x, y)], 0);
                }
            });

        let flat_to = image.to_flat_samples();
        assert_eq!(flat_from.color_hint, flat_to.color_hint);
        assert_eq!(flat_from.layout, flat_to.layout);
        assert_eq!(flat_from.samples.len(), flat_to.samples.len());
        assert!({
            flat_from
                .samples
                .iter()
                .zip(flat_from.samples.iter())
                .all(|(lhs, rhs)| lhs == rhs)
        });
    }

    #[test]
    fn convert_image_buffer_vs_image() {
        let width = 120;
        let height = 80;
        let image_buf_from = {
            let mut buf = ImageBuffer::<Luma<u8>, _>::new(width, height);
            (0..(width.min(height))).into_iter().for_each(|idx| {
                buf[(idx, idx)][0] = 255;
            });
            buf
        };

        let image = Image::from_image_buffer(&image_buf_from);
        (0..height)
            .into_iter()
            .flat_map(|y| (0..width).into_iter().map(move |x| (x, y)))
            .for_each(|(x, y)| {
                if x == y {
                    assert_eq!(image[(x as usize, y as usize)], 255);
                } else {
                    assert_eq!(image[(x as usize, y as usize)], 0);
                }
            });

        let image_buf_to = image.to_image_buffer();
        assert_eq!(image_buf_from.width(), image_buf_to.width());
        assert_eq!(image_buf_from.height(), image_buf_to.height());
        assert!({
            image_buf_from
                .pixels()
                .zip(image_buf_to.pixels())
                .all(|(lhs, rhs)| lhs == rhs)
        });
    }

    fn diagonal_image(width: usize, height: usize) -> Image {
        let mut image = Image::zeros_with_alignment(width, height, DEFAULT_ALIGNMENT_U8).unwrap();
        (0..width.min(height)).into_iter().for_each(|index| {
            image[(index, index)] = 255;
        });
        image
    }
}

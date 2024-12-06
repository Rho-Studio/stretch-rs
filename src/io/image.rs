use std::fs::File;

use std::path::PathBuf;

use super::raw::read_raw_image;
use crate::io::{FileFormat, OutputParams};

use crate::stretch::apply_histogram_stretch;
use image::error::{EncodingError, ImageFormatHint, LimitError, LimitErrorKind, UnsupportedError};
use image::{ColorType, DynamicImage, EncodableLayout, ImageError, ImageFormat};
use tiff::encoder::{colortype, TiffEncoder};
use tiff::TiffError;

pub fn read_image(path: &String) -> DynamicImage {
    let image = image::open(path);

    match image {
        Ok(image) => image,
        Err(_) => {
            read_raw_image(path).expect("Failed to load image: Unexpected format encountered")
        }
    }
}

pub fn write_image_buffer(
    output: &OutputParams,
    data: &DynamicImage,
    source_color: ColorType,
) -> Result<(), ImageError> {
    let data = match source_color {
        ColorType::Rgba32F => data,
        _ => &apply_histogram_stretch(data),
    };

    match output.format {
        FileFormat::JPEG => {
            let buffer = data.to_rgb8();

            image::save_buffer(
                output.path.clone(),
                buffer.as_bytes(),
                buffer.width(),
                buffer.height(),
                ColorType::Rgb8,
            )
        }
        FileFormat::OpenEXR => {
            let buffer = data.to_rgb32f();

            image::save_buffer(
                output.path.clone(),
                buffer.as_bytes(),
                buffer.width(),
                buffer.height(),
                ColorType::Rgb32F,
            )
        }
        FileFormat::TIFF => {
            let output_file = File::create::<PathBuf>(output.into()).unwrap();
            let mut encoder = TiffEncoder::new(&output_file).unwrap();

            match source_color {
                ColorType::Rgb8 => {
                    let mut image = encoder
                        .new_image::<colortype::RGB8>(data.width(), data.height())
                        .unwrap();

                    image.rows_per_strip(4).unwrap();

                    let buffer = data.to_rgb8();
                    let buffer = buffer.as_raw();

                    let mut idx = 0;
                    while image.next_strip_sample_count() > 0 {
                        let sample_count = image.next_strip_sample_count() as usize;
                        image.write_strip(&buffer[idx..idx + sample_count]).unwrap();

                        idx += sample_count;
                    }

                    image.finish().map_err(|err| -> ImageError {
                        match err {
                            TiffError::FormatError(_error) => ImageError::Encoding(
                                EncodingError::from_format_hint(ImageFormatHint::Unknown),
                            ),
                            TiffError::UnsupportedError(_) => ImageError::Unsupported(
                                UnsupportedError::from(ImageFormatHint::Exact(ImageFormat::Tiff)),
                            ),
                            TiffError::IoError(error) => ImageError::IoError(error),
                            TiffError::LimitsExceeded => ImageError::Limits(LimitError::from_kind(
                                LimitErrorKind::InsufficientMemory,
                            )),
                            TiffError::IntSizeError => ImageError::Limits(LimitError::from_kind(
                                LimitErrorKind::DimensionError,
                            )),
                            TiffError::UsageError(_error) => ImageError::Encoding(
                                EncodingError::from_format_hint(ImageFormatHint::Unknown),
                            ),
                        }
                    })
                }
                ColorType::Rgb16 => {
                    let mut image = encoder
                        .new_image::<colortype::RGB16>(data.width(), data.height())
                        .unwrap();

                    let buffer = data.to_rgb16();
                    let buffer = buffer.as_raw();

                    image.rows_per_strip(4).unwrap();

                    let mut idx = 0;
                    while image.next_strip_sample_count() > 0 {
                        let sample_count = image.next_strip_sample_count() as usize;
                        image.write_strip(&buffer[idx..idx + sample_count]).unwrap();

                        idx += sample_count;
                    }

                    image.finish().map_err(|err| -> ImageError {
                        match err {
                            TiffError::FormatError(_error) => ImageError::Encoding(
                                EncodingError::from_format_hint(ImageFormatHint::Unknown),
                            ),
                            TiffError::UnsupportedError(_) => ImageError::Unsupported(
                                UnsupportedError::from(ImageFormatHint::Exact(ImageFormat::Tiff)),
                            ),
                            TiffError::IoError(error) => ImageError::IoError(error),
                            TiffError::LimitsExceeded => ImageError::Limits(LimitError::from_kind(
                                LimitErrorKind::InsufficientMemory,
                            )),
                            TiffError::IntSizeError => ImageError::Limits(LimitError::from_kind(
                                LimitErrorKind::DimensionError,
                            )),
                            TiffError::UsageError(_error) => ImageError::Encoding(
                                EncodingError::from_format_hint(ImageFormatHint::Unknown),
                            ),
                        }
                    })
                }
                ColorType::Rgb32F => {
                    let mut image = encoder
                        .new_image::<colortype::RGB32Float>(data.width(), data.height())
                        .unwrap();

                    let buffer = data.to_rgb32f();
                    let buffer = buffer.as_raw();

                    image.rows_per_strip(4).unwrap();

                    let mut idx = 0;
                    while image.next_strip_sample_count() > 0 {
                        let sample_count = image.next_strip_sample_count() as usize;
                        image.write_strip(&buffer[idx..idx + sample_count]).unwrap();

                        idx += sample_count;
                    }

                    image.finish().map_err(|err| -> ImageError {
                        match err {
                            TiffError::FormatError(_error) => ImageError::Encoding(
                                EncodingError::from_format_hint(ImageFormatHint::Unknown),
                            ),
                            TiffError::UnsupportedError(_) => ImageError::Unsupported(
                                UnsupportedError::from(ImageFormatHint::Exact(ImageFormat::Tiff)),
                            ),
                            TiffError::IoError(error) => ImageError::IoError(error),
                            TiffError::LimitsExceeded => ImageError::Limits(LimitError::from_kind(
                                LimitErrorKind::InsufficientMemory,
                            )),
                            TiffError::IntSizeError => ImageError::Limits(LimitError::from_kind(
                                LimitErrorKind::DimensionError,
                            )),
                            TiffError::UsageError(_error) => ImageError::Encoding(
                                EncodingError::from_format_hint(ImageFormatHint::Unknown),
                            ),
                        }
                    })
                }
                _ => todo!(),
            }
        }
    }
}

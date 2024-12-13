use crate::extensions::NDArrayBuffer;
use image::DynamicImage;
use ndarray::Array3;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Stretcher {
    y: f32,
    midtones: f32,
    input_shadow: f32,
    output_shadow: f32,
    input_highlight: f32,
    output_highlight: f32,
}

impl Stretcher {
    pub fn from_image(image: &DynamicImage) -> Self {
        let luminance = image.to_luma32f();

        let input_shadow = luminance.iter().copied().reduce(f32::min).unwrap();
        let input_highlight = luminance.iter().copied().reduce(f32::max).unwrap();

        let data = image.to_nd_array_buffer();

        let (gamma, midtones) = Self::gamma_from_image_buffer(&data);

        Self {
            y: gamma,
            midtones,
            input_shadow,
            output_shadow: 0.,
            input_highlight,
            output_highlight: 1.,
        }
    }

    fn gamma_correction(&self) -> f32 {
        1. / self.y
    }

    fn gamma_from_image_buffer(data: &Array3<f32>) -> (f32, f32) {
        let mut gamma = 1.;

        let (mut vec, _) = data.clone().into_raw_vec_and_offset();
        vec.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let mut midtone = vec[vec.len() / 2];

        let input_midtone = midtone;

        if midtone < 0.5 {
            midtone *= 2.;
            gamma = (1. + (1.2 * (1. - midtone))).min(1.5);
        } else if midtone > 0.5 {
            midtone = (midtone * 2.) - 1.;
            gamma = (1. - midtone).max(0.01);
        }

        (gamma, input_midtone)
    }

    pub fn apply(&self, data: &mut Array3<f32>) {
        data.par_iter_mut().for_each(|x| {
            *x = (*x - self.input_shadow) / (self.input_highlight - self.input_shadow);

            if self.midtones != 0.5 {
                *x = x.powf(self.gamma_correction());
            }

            *x = *x * (self.output_highlight - self.output_shadow) + self.output_shadow;

            *x = x.clamp(0., 1.);
        });
    }
}

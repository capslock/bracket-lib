use bevy::{
    prelude::*,
    render::texture::{ImageSampler, ImageSamplerDescriptor},
};

#[derive(Resource)]
pub(crate) struct ImagesToLoad(pub(crate) Vec<UntypedHandle>);

pub(crate) fn fix_images(mut fonts: ResMut<ImagesToLoad>, mut images: ResMut<Assets<Image>>) {
    if fonts.0.is_empty() {
        return;
    }

    for (handle, img) in images.iter_mut() {
        let mut to_remove = Vec::new();
        if let Some(i) = fonts.0.iter().enumerate().find(|(_i, h)| h.id() == handle) {
            img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                label: Some("LeaveItAlone".to_string()),
                address_mode_u: bevy::render::texture::ImageAddressMode::ClampToEdge,
                address_mode_v: bevy::render::texture::ImageAddressMode::ClampToEdge,
                address_mode_w: bevy::render::texture::ImageAddressMode::ClampToEdge,
                mag_filter: bevy::render::texture::ImageFilterMode::Nearest,
                min_filter: bevy::render::texture::ImageFilterMode::Nearest,
                mipmap_filter: bevy::render::texture::ImageFilterMode::Nearest,
                ..Default::default()
            });
            to_remove.push(i.0);
        }
        to_remove.iter().for_each(|i| {
            fonts.0.remove(*i);
        });
    }
}

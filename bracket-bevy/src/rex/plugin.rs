use bevy::{
    asset::{io::Reader, AssetApp, AssetLoader, AsyncReadExt as _},
    prelude::{App, Plugin},
};
use bracket_rex::prelude::XpFile;

#[derive(Default)]
pub struct RexAssetPlugin;

impl Plugin for RexAssetPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(RexAssetLoader {})
            .init_asset::<XpFile>();
    }
}

#[derive(Default)]
struct RexAssetLoader;

impl AssetLoader for RexAssetLoader {
    type Asset = XpFile;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = XpFile::read(&mut bytes.as_slice())?;
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xp"]
    }
}

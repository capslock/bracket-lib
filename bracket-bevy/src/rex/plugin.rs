use bevy::{
    asset::{AddAsset, AssetLoader, LoadedAsset},
    prelude::{App, Plugin},
};
use bracket_rex::prelude::XpFile;

struct RexAssetPlugin;

impl Plugin for RexAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<XpFile>()
            .add_asset_loader(RexAssetLoader {});
    }
}

struct RexAssetLoader;

impl AssetLoader for RexAssetLoader {
    fn load<'a>(
        &'a self,
        mut bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let asset = XpFile::read(&mut bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xp"]
    }
}

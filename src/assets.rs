use heapless::Vec;
use paste::paste;

use crate::graphics::GraphicAsset;

pub type AssetsManagerIdent = usize;

pub struct AssetsManager<const N: usize> {
    assets: AssetVec<N>,
}

impl<const N: usize> AssetsManager<N> {
    pub fn register_asset(&mut self, asset: &'static Asset) -> AssetsManagerIdent {
        self.assets.register(asset)
    }
}

pub struct AssetVec<const N: usize>{
    vec: Vec<&'static Asset, N>,
    removed: Vec<AssetsManagerIdent, N>,
}

impl<const N: usize> AssetVec<N> {
    pub fn register(&mut self, asset: &'static Asset) -> AssetsManagerIdent{
        if self.removed.is_empty() {
            self.vec.push(asset).unwrap();
            (self.vec.len() - 1)
        } else {
            let id = self.removed.pop().unwrap();
            *self.vec.get_mut(id).unwrap() = asset;
            id
        }
    }

    pub fn unregister(&mut self, ind: AssetsManagerIdent) {
        self.removed.push(ind).expect("A0");
    }

    pub fn get(&self, ind: AssetsManagerIdent) -> Option<&&'static Asset> {
        if self.removed.contains(&ind) {
            None
        } else {
            self.vec.get(ind)
        }
    }
}

#[macro_export]
macro_rules! declare_asset {
    ($name:ident, $fname:expr, $size:expr, $type:expr) => {
        paste! {
           static [< $name _DATA >] : &'static [u8] = include_bytes!($fname);
           static $name : Asset = Asset::create([< $name _DATA >], $size, $type);
        }
    }
}

declare_asset!(HEART_ICON, "../../rusty_gbadev_test/build/icons/icon.bmp", 12426, AssetType::Graphic(GraphicAsset::Bitmap(64, 64)));

#[derive(Debug)]
pub struct Asset {
    data: &'static [u8],
    dcp_size: usize,
    pub asset_type: AssetType,
}

impl Asset {
    pub const fn create(data: &'static [u8], dcp_size: usize, asset_type: AssetType) -> Asset {
        Asset {
            data,
            dcp_size,
            asset_type,
        }
    }
}

#[derive(Debug)]
pub enum AssetType {
    Graphic(GraphicAsset)
}

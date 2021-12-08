
static data : Asset = Asset::create(include_bytes!("../../rusty_gbadev_test/build/icons/icon.bmp"), 12426, AssetType::Graphic(GraphicAsset::Bitmap(64, 64, 24)), AssetCompression::Lze);

pub struct Asset {
    data: &'static [u8],
    dcp_size: usize,
    asset_type: AssetType,
    compression: AssetCompression,
}

impl Asset {
    pub const fn create(data: &'static [u8], dcp_size: usize, asset_type: AssetType, compression: AssetCompression) -> Asset {
        Asset {
            data,
            dcp_size,
            asset_type,
            compression
        }
    }

    pub fn is_graphical(&self) -> bool {
        matches!(self.asset_type, AssetType::Graphic(_))
    }

    pub fn load(&self, dst: *mut u8) {
        self.compression.decompress(self.data, self.dcp_size, dst);
    }
}

pub enum AssetCompression {
    Blz,
    Huffman,
    Lze,
    Lzss,
    Lzx,
    Rle,
}

impl AssetCompression {
    fn build_prefix(&self, dcp_size: u32) -> u32 {
        dcp_size << 8 | (1 << 4) | (0)
    }

    pub fn decompress(&self, src: &'static [u8], dcp_size: usize, dst: *mut u8) {
        // TODO      Find a way to use BIOS function for decompression
        //pub unsafe fn LZ77UnCompReadNormalWrite8bit(src: *const u32, dst: *mut u8) {
        todo!();
    }
}

pub enum AssetType {
    Graphic(GraphicAsset)
}

pub enum GraphicAsset {
    Bitmap(width, height, color_depth),
}

impl GraphicAsset {
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        todo!();
    }
}


//
// pub struct AssetsManager<T: GameAssets, const N: usize> {
//     assets: [Asset; N],
//     game_assets: T,
// }
//
// impl<T: GameAssets, const N: usize> AssetsManager<T, N> {
//     pub const fn new() -> AssetsManager<T, N> {
//         AssetsManager {
//             assets: (0..N).map(|n| T::get_asset(n)).collect(),
//             game_assets: T::new(),
//         }
//     }
// }
//
// pub type AssetId = u32;
//
// pub trait GameAssets { //: SpriteAssets { // : SoundsAssets + MusicAssets + SpriteAssets + BackgroundAssets
//     fn get_asset(nb: usize) -> Asset;
//     fn new() -> Self where Self: Sized;
// }
// //
// // pub trait SpriteAssets {
// //     type SpriteVariant;
// //     fn get_sprite(&self, asked: SpriteVariant) -> AssetId;
// // }
//

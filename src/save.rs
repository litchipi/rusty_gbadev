use core::mem::{size_of, transmute_copy};
use gba::prelude::{use_flash_128k, SaveAccess};

use crate::system::{GameState, GbaSystem};

const SAVE_OFFSET: usize = 0;
const FLASH_WRITE_BLOCKSIZE: usize = 512;

impl<T: GameState> GbaSystem<T> {
    pub fn save(&self) {
        use_flash_128k();
        let access = SaveAccess::new().unwrap();

        let gsave = self.game.get_gamesave();
        let data: &[u8] = unsafe { transmute_copy(&gsave) };
        let datalen = size_of::<T::SaveType>();

        let mut current = SAVE_OFFSET;
        let end = SAVE_OFFSET + datalen;
        access.prepare_write(current..end).unwrap();

        while current != end {
            let cur_len = if end - current > FLASH_WRITE_BLOCKSIZE {
                FLASH_WRITE_BLOCKSIZE
            } else {
                end - current
            };
            access
                .write(current, &data[current..(current + cur_len)])
                .unwrap();
            current += cur_len;
        }
    }

    pub fn load(&mut self) {
        use_flash_128k();
        let access = SaveAccess::new().unwrap();

        let gsave = self.game.get_gamesave();
        let data: &mut [u8] = unsafe { transmute_copy(&gsave) };
        let datalen = size_of::<T::SaveType>();

        let mut current = SAVE_OFFSET;
        let end = SAVE_OFFSET + datalen;

        while current != end {
            let cur_len = if end - current > FLASH_WRITE_BLOCKSIZE {
                FLASH_WRITE_BLOCKSIZE
            } else {
                end - current
            };
            access
                .read(current, &mut data[current..(current + cur_len)])
                .unwrap();
            current += cur_len;
        }
        self.game.load_gamesave(gsave);
    }
}

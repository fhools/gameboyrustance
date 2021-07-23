use std::fs;
use std::io::{self, Read};
use std::fs::File;
use std::path::Path;

pub struct GbaCartridgeHeader {
    pub rom_entry_point: u32,
    pub nintendo_logo: [u8; 156],
    pub game_title: [u8; 12],
    pub game_code: [u8; 4],
    pub maker_code: [u8; 2],
    pub fixed_value: u8,
    pub main_unit_code: u8,
    pub device_type: u8,
    pub reserved_area1: [u8; 7],
    pub software_version: u8,
    pub checksum: u8,
    pub reserved_area2: [u8; 2],
    pub ram_entry_point: u32,
    pub boot_mode: u8,
    pub slave_id_number: u8,
    pub not_used_padding: [u8; 26],
    pub joybus_entry_point: u32,
}

impl GbaCartridgeHeader {
    const ROM_ENTRY_POINT_OFFSET: usize = 0;
    const NINTENDO_LOGO_OFFSET: usize = 0x4;
    const GAME_TITLE_LOGO_OFFSET: usize = 0xA0;
    const GAME_CODE_OFFSET: usize = 0xAC;
    const MAKER_CODE_OFFSET: usize = 0xB0;
    const FIXED_VALUE_OFFSET: usize = 0xB2;
    const MAIN_UNIT_CODE_OFFSET: usize = 0xB3;
    const DEVICE_TYPE_OFFSET: usize = 0xB4;
    const RESERVED_AREA1_OFFSET: usize = 0xB5;
    const SOFTWARE_VERSION_OFFSET: usize = 0xBC;
    const CHECKSUM_OFFSET: usize = 0xBD;
    const RESERVED_AREA2_OFFSET: usize = 0xBE;
    const RAM_ENTRY_POINT_OFFSET: usize = 0xC0;
    const BOOT_MODE_OFFSET: usize = 0xC4;
    const SLAVE_ID_NUMBER_OFFSET: usize = 0xC5;
    const NOT_USED_PADDING_OFFSET: usize = 0xC6;
    const JOYBUS_ENTRY_POINT_OFFSET: usize = 0xE0;
}

pub struct GbaData {
    header: GbaCartridgeHeader,
    data: Vec<u8>
}

impl GbaData  {
    fn open<T: AsRef<Path>>(filepath: T) -> Result<Self, io::Error>  {
        let mut f = File::open(filepath.as_ref())?;
        let fsmeta = fs::metadata(filepath.as_ref())?;
        let fsize = fsmeta.len();
        let mut buf = vec![0u8; fsize as usize];
        let len = f.read(&mut buf[..]);
        if let Err(err) = len {
            Err(err)
        } else {
            Ok(GbaData { data: buf })
        }
    }

    fn from_slice<T: AsRef<[u8]>>(buf: T) -> Self {
        GbaData {
            header: Default::default(),
            data: buf.as_ref().to_vec()
        }
    }

    fn rom_entry(&self) -> u32 {
        let mut addr: u32 = self.data[0] as u32;
        addr |= (self.data[1] as u32) << 8;
        addr |= (self.data[2] as u32) << 16;
        addr |= (self.data[3] as u32) << 24;
        addr
    }
}

mod tests {
    use super::*;
    #[test]
    fn gbadata_open() {
        let gba = GbaData::open("blah.gba");
        assert!(gba.is_err());
    }


    #[test]
    fn gbadata_from_slice() {
        let gba = GbaData::from_slice(&vec![0xEF, 0xBE, 0xAD, 0xDE]);
        println!("addr: {:x}", gba.rom_entry())
    }
}

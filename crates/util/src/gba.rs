use std::str;
use std::fs;
use std::convert::TryInto;
use std::io::{self, Read};
use std::fs::File;
use std::path::Path;

pub struct GbaCartridgeHeader {
    pub rom_entry_branch_instr: u32,
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
    pub ram_entry_branch_instr: u32,
    pub boot_mode: u8,
    pub slave_id_number: u8,
    pub not_used_padding: [u8; 26],
    pub joybus_entry_point: u32,
}

impl GbaCartridgeHeader {
    const ROM_ENTRY_BRANCH_INSTR: usize = 0;
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
    const RAM_ENTRY_BRANCH_INSTR_OFFSET: usize = 0xC0;
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
            if len.unwrap() < GbaCartridgeHeader::RAM_ENTRY_BRANCH_INSTR_OFFSET as usize {
                Err(io::Error::new(io::ErrorKind::InvalidData, "insufficient data"))
            } else {
                Ok(GbaData::from_slice(&buf[..]))
            }
        }
    }

    fn from_slice<T: AsRef<[u8]>>(buf: T) -> Self {
        let mut gba_header  =  GbaCartridgeHeader {
            rom_entry_branch_instr: 0,
            nintendo_logo: [0; 156],
            game_title: [0; 12],
            game_code: [0; 4],
            maker_code: [0; 2],
            fixed_value: 0,
            main_unit_code: 0,
            device_type: 0,
            reserved_area1: [0; 7],
            software_version: 0,
            checksum: 0,
            reserved_area2: [0; 2],
            ram_entry_branch_instr: 0,
            boot_mode: 0,
            slave_id_number: 0,
            not_used_padding: [0; 26],
            joybus_entry_point: 0
        };
        let mut buf2: &[u8] = &buf.as_ref()[GbaCartridgeHeader::ROM_ENTRY_BRANCH_INSTR..GbaCartridgeHeader::NINTENDO_LOGO_OFFSET];
        gba_header.rom_entry_branch_instr = u32::from_le_bytes(buf2.try_into().unwrap());

        buf2 = &buf.as_ref()[GbaCartridgeHeader::NINTENDO_LOGO_OFFSET..GbaCartridgeHeader::GAME_TITLE_LOGO_OFFSET]; 
        gba_header.nintendo_logo.clone_from_slice(buf2);

        buf2 = &buf.as_ref()[GbaCartridgeHeader::GAME_TITLE_LOGO_OFFSET..GbaCartridgeHeader::GAME_CODE_OFFSET];
        gba_header.game_title.clone_from_slice(buf2);

        buf2 = &buf.as_ref()[GbaCartridgeHeader::GAME_CODE_OFFSET..GbaCartridgeHeader::MAKER_CODE_OFFSET];
        gba_header.game_code.clone_from_slice(buf2);

        buf2 = &buf.as_ref()[GbaCartridgeHeader::MAKER_CODE_OFFSET..GbaCartridgeHeader::FIXED_VALUE_OFFSET];
        gba_header.maker_code.clone_from_slice(buf2); 

        gba_header.fixed_value = buf.as_ref()[GbaCartridgeHeader::FIXED_VALUE_OFFSET];
        gba_header.main_unit_code = buf.as_ref()[GbaCartridgeHeader::MAIN_UNIT_CODE_OFFSET];
        gba_header.device_type = buf.as_ref()[GbaCartridgeHeader::DEVICE_TYPE_OFFSET];

        buf2 = &buf.as_ref()[GbaCartridgeHeader::RESERVED_AREA1_OFFSET..GbaCartridgeHeader::SOFTWARE_VERSION_OFFSET];
        gba_header.reserved_area1.clone_from_slice(buf2);
        
        gba_header.software_version = buf.as_ref()[GbaCartridgeHeader::SOFTWARE_VERSION_OFFSET];
        gba_header.checksum = buf.as_ref()[GbaCartridgeHeader::CHECKSUM_OFFSET];
       
        // TODO: Implement Multiboot stuff
        if buf.as_ref().len() > GbaCartridgeHeader::RAM_ENTRY_BRANCH_INSTR_OFFSET {
            buf2 = &buf.as_ref()[GbaCartridgeHeader::RESERVED_AREA2_OFFSET..GbaCartridgeHeader::RAM_ENTRY_BRANCH_INSTR_OFFSET];
            gba_header.reserved_area1.clone_from_slice(buf2);
        }

        GbaData { header: gba_header, data: vec![]}
    }

    fn rom_entry(&self) -> u32 {
        self.header.rom_entry_branch_instr
    }

    fn nintendo_logo(&self) -> Vec<u8> {
        self.header.nintendo_logo.to_vec()
    }

    fn game_title(&self) -> String {
        let game_title = str::from_utf8(&self.header.game_title[..]).unwrap();
        game_title.to_owned()
    }

    fn game_code(&self) -> Vec<u8> {
        self.header.game_code.to_vec()
    }

    fn maker_code(&self) -> String {
        let maker_code = str::from_utf8(&self.header.maker_code[..]).unwrap();
        maker_code.to_owned()
    }

    fn fixed_value(&self) -> u8 {
        assert!(self.header.fixed_value == 0x96);
        self.header.fixed_value
    }

    fn main_unit_code(&self) -> u8 {
        assert!(self.header.main_unit_code == 0x00);
        self.header.main_unit_code
    }
    
    fn device_type(&self) -> u8 {
        // Normally this is 0, but with Nintendo hardware debugger bit 7
        // identifies debug handler entry point and size of Debugging
        // And Communication System size
        // Bit 7 = 0 
        //      Address: 0x9FFC000
        //      DACS Size: 8-Mbit
        // Bit 7 = 1
        //      Address: 0x9FE2000
        //      DACS Size: 1-Mbit
        self.header.device_type
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
        let mut data: Vec<u8> = vec![0x0u8; GbaCartridgeHeader::RAM_ENTRY_BRANCH_INSTR_OFFSET];
        data.splice(0..4, [0xEFu8, 0xBEu8, 0xADu8, 0xDEu8].iter().cloned());
        let gba = GbaData::from_slice(&data[..]);
        println!("addr: {:x}", gba.rom_entry());
        let nintendo_logo = gba.nintendo_logo();
        println!("logo: {:?}", nintendo_logo);
        println!("title: {}", gba.game_title());
    }
}

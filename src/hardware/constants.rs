#[macro_export]
macro_rules! byte_size {
    ($val:literal b) => {
        $val
    };
    ($val:literal kb) => {
        $val * 1024usize
    };
    ($val:literal mb) => {
        $val * 1024usize * 1024
    };
    ($val:literal gb) => {
        $val * 1024usize * 1024 * 1024
    };
    ($val:literal tb) => {
        $val * 1024usize * 1024 * 1024 * 1024 * 1024
    };
}

pub mod cpu {
    pub const RAM_SIZE: usize = byte_size!(2 kb);
    pub const STACK_OFFSET: u16 = 0x100;

    #[rustfmt::skip]
    pub mod flags {
        pub const CARRY             :u8 = 0b00000001;
        pub const ZERO              :u8 = 0b00000010;
        pub const INTERRUPT_DISABLE :u8 = 0b00000100;
        pub const DECIMAL_MODE      :u8 = 0b00001000;
        pub const BREAK             :u8 = 0b00010000;
        pub const UNUSED            :u8 = 0b00100000;
        pub const OVERFLOW          :u8 = 0b01000000;
        pub const NEGATIVE          :u8 = 0b10000000;
    }
}

pub mod cartrige {
    pub const NES_MAGIC_NUMBERS: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
    pub const PRG_ROM_BANK_SIZE: usize = byte_size!(16 kb);
    pub const CHR_ROM_BANK_SIZE: usize = byte_size!(8 kb);
    pub const PRG_RAM_BANK_SIZE: usize = byte_size!(8 kb);

    pub const FLAG6_NAMETABLE: u8 = 1 << 0;
    pub const FLAG6_BATTERY: u8 = 1 << 1;
    pub const FLAG6_TRAINER: u8 = 1 << 2;
    pub const FLAG6_FOUR_SCREEN: u8 = 1 << 3;
    pub const FLAG7_VS_UNISYSTEM: u8 = 1 << 0;
    pub const FLAG7_PLAYCHOICE_10: u8 = 1 << 1;
    pub const FLAG7_NES2_SIGNATURE_MASK: u8 = (1 << 3) | (1 << 2);
    pub const FLAG7_NES2_SIGNATURE_VALUE: u8 = 1 << 3;
    pub const FLAG9_TV_SYSTEM: u8 = 1 << 0;
    pub const FLAG10_TV_SYSTEM_MASK: u8 = (1 << 1) | (1 << 0);
}

pub mod ppu {
    pub const PALLET_SIZE: usize = 0x20;
    pub const NAMETABLE_SIZE: usize = byte_size!(1 kb);

    /// read more here: https://www.nesdev.org/wiki/PPU_scrolling
    #[rustfmt::skip]
    pub mod vram_sections{
        pub const COARSE_X                  : u16 = 0b0000000000011111;
        pub const COARSE_Y                  : u16 = 0b0000001111100000;
        pub const BASE_NAMETABLE_ADDRESS_X  : u16 = 0b0000010000000000;
        pub const BASE_NAMETABLE_ADDRESS_Y  : u16 = 0b0000100000000000;
        pub const BASE_NAMETABLE_ADDRESS    : u16 = 0b0000110000000000;
        pub const NAMETABLE_OFFSET          : u16 = 0b0000111111111111;
        pub const FINE_Y                    : u16 = 0b0111000000000000;
    }

    #[rustfmt::skip]
    pub mod control_flags {
        pub const BASE_NAMETABLE_ADDRESS    : u8 = 0b00000011;
        pub const VRAM_INC                  : u8 = 0b00000100;
        pub const SPRITE_PATTERN_TABLE_ADDR : u8 = 0b00001000;
        pub const BG_PATTERN_TABLE_ADDR     : u8 = 0b00010000;
        pub const SPRITE_SIZE               : u8 = 0b00100000;
        pub const MASTER_SLAVE_SELECT       : u8 = 0b01000000;
        pub const VBLANK_NMI                : u8 = 0b10000000;
    }

    #[rustfmt::skip]
    pub mod mask_flags {
        pub const GRAYSCALE                 : u8 = 0b00000001;
        pub const SHOW_TOP_LEFT_BACKGROUND  : u8 = 0b00000010;
        pub const SHOW_TOP_LEFT_SPRITE      : u8 = 0b00000100;
        pub const ENABLE_BG_RENDERING       : u8 = 0b00001000;
        pub const ENABLE_SPRITE_RENDERING   : u8 = 0b00010000;
        pub const EMPHASIZE_RED             : u8 = 0b00100000;
        pub const EMPHASIZE_GREEN           : u8 = 0b01000000;
        pub const EMPHASIZE_BLUE            : u8 = 0b10000000;
    }

    #[rustfmt::skip]
    pub mod status_flags {
        pub const OPEN_BUS          : u8 = 0b00011111;
        pub const SPRITE_OVERFLOW   : u8 = 0b00100000;
        pub const SPRITE_0_HIT      : u8 = 0b01000000;
        pub const VBLANK            : u8 = 0b10000000;
    }

    #[rustfmt::skip]
    pub mod sprite_tile_id{
        pub const BANK      : u8 = 0b00000001;
        pub const TILE_ID   : u8 = 0b11111110;
    }

    #[rustfmt::skip]
    pub mod sprite_attributes{
        pub const PALLETE           : u8 = 0b00000011;
        pub const UNUSED            : u8 = 0b00011100;
        pub const PRIORITY          : u8 = 0b00100000;
        pub const FLIP_HORIZONTALLY : u8 = 0b01000000;
        pub const FLIP_VERTICALLY   : u8 = 0b10000000;
    }

    #[rustfmt::skip]
    pub const COLORS: [u32; 64] =
    [
        0x545454, 0x001e74, 0x081090, 0x300088, 0x440064, 0x5c0030, 0x540400, 0x3c1800,
        0x202a00, 0x083a00, 0x004000, 0x003c00, 0x00323c, 0x000000, 0x000000, 0x000000,
        0x989698, 0x084cc4, 0x3032ec, 0x5c1ee4, 0x8814b0, 0xa01464, 0x982220, 0x783c00,
        0x545a00, 0x287200, 0x087c00, 0x007628, 0x006678, 0x000000, 0x000000, 0x000000,
        0xeceeec, 0x4c9aec, 0x787cec, 0xb062ec, 0xe454ec, 0xec58b4, 0xec6a64, 0xd48820,
        0xa0aa00, 0x74c400, 0x4cd020, 0x38cc6c, 0x38b4cc, 0x3c3c3c, 0x000000, 0x000000,
        0xeceeec, 0xa8ccec, 0xbcbcec, 0xd4b2ec, 0xecaeec, 0xecaed4, 0xecb4b0, 0xe4c490,
        0xccd278, 0xb4de78, 0xa8e290, 0x98e2b4, 0xa0d6e4, 0xa0a2a0, 0x000000, 0x000000,
    ];
}

// #[rustfmt::skip]
// pub const PPU_COLORS: [u32; 64] =
// [
//     0x666666, 0x002A88, 0x1412A7, 0x3B00A4, 0x5C007E, 0x6E0040, 0x6C0600, 0x561D00,
//     0x333500, 0x0B4800, 0x005200, 0x004F08, 0x00404D, 0x000000, 0x000000, 0x000000,
//     0xADADAD, 0x155FD9, 0x4240FF, 0x7527FE, 0xA01ACC, 0xB71E7B, 0xB53120, 0x994E00,
//     0x6B6D00, 0x388700, 0x0C9300, 0x008F32, 0x007C8D, 0x000000, 0x000000, 0x000000,
//     0xFFFEFF, 0x64B0FF, 0x9290FF, 0xC676FF, 0xF36AFF, 0xFE6ECC, 0xFE8170, 0xEA9E22,
//     0xBCBE00, 0x88D800, 0x5CE430, 0x45E082, 0x48CDDE, 0x4F4F4F, 0x000000, 0x000000,
//     0xFFFEFF, 0xC0DFFF, 0xD3D2FF, 0xE8C8FF, 0xFBC2FF, 0xFEC4EA, 0xFECCC5, 0xF7D8A5,
//     0xE4E594, 0xCFEF96, 0xBDF4AB, 0xB3F3CC, 0xB5EBF2, 0xB8B8B8, 0x000000, 0x000000
// ];

use std::{cell::RefCell, rc::Rc};

use crate::hardware::{
    bit_ops::BitOps,
    cartrige::{Cartrige, cartrige_access::CartrigeAccess},
    constants::{
        self,
        ppu::{NAMETABLE_SIZE, control_flags, mask_flags, status_flags, vram_sections::*},
    },
    cpu::Cpu,
    ppu::pallet_memory::PalletMemory,
};

pub mod pallet_memory;

pub type BackgroundSprite = [[u8; 8]; 8];
pub type PatternTable = [[BackgroundSprite; 16]; 32];

// TODO: open bus
pub struct Ppu {
    cpu: Option<Rc<RefCell<Cpu>>>,
    cartrige: Option<Rc<RefCell<Cartrige>>>,
    scanline: u32,
    dot: u32,
    pub pallet_memory: PalletMemory,
    nametable_memory: [u8; NAMETABLE_SIZE * 4],
    open_bus: u8,
    vram_address: u16,
    temp_vram_address: u16,
    fine_x: u8,
    is_writing_low_byte: bool,
    /// more info about this: https://www.nesdev.org/wiki/PPU_registers#:~:text=avoid%20wrong%20scrolling.-,the%20ppudata%20read%20buffer,-Reading%20from%20PPUDATA
    ppu_data_read_buffer: u8,
    pub control_register: u8,
    mask_register: u8,
    status_register: u8,
    renderer_sprite_id: u8,
    renderer_attribute_lsb: u8,
    renderer_attribute_msb: u8,
    renderer_pattern_msb: u8,
    renderer_pattern_lsb: u8,
    renderer_shift_pattern_msb: u16,
    renderer_shift_pattern_lsb: u16,
    renderer_shift_attribute_lsb: u16,
    renderer_shift_attribute_msb: u16,
    is_odd_frame: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            cpu: None,
            cartrige: None,
            scanline: 0,
            dot: 0,
            pallet_memory: PalletMemory::new(),
            nametable_memory: [0; NAMETABLE_SIZE * 4],
            open_bus: 0,
            vram_address: 0,
            temp_vram_address: 0,
            fine_x: 0,
            is_writing_low_byte: false,
            ppu_data_read_buffer: 0,
            control_register: 0,
            mask_register: 0,
            status_register: 0,
            renderer_sprite_id: 0,
            renderer_attribute_lsb: 0,
            renderer_attribute_msb: 0,
            renderer_pattern_msb: 0,
            renderer_pattern_lsb: 0,
            renderer_shift_pattern_msb: 0,
            renderer_shift_pattern_lsb: 0,
            renderer_shift_attribute_lsb: 0,
            renderer_shift_attribute_msb: 0,
            is_odd_frame: false,
        }
    }

    pub fn insert_cartrige(&mut self, cartrige: Rc<RefCell<Cartrige>>) {
        self.cartrige = Some(cartrige);
    }

    pub fn connect_cpu(&mut self, cpu: Rc<RefCell<Cpu>>) {
        self.cpu = Some(cpu);
    }

    pub fn read_register(&mut self, address: u16) -> u8 {
        self.read_register_inner(address, false)
    }

    pub fn peek_register(&mut self, address: u16) -> u8 {
        self.read_register_inner(address, true)
    }

    pub(crate) fn read_register_inner(&mut self, address: u16, peek: bool) -> u8 {
        if address == 0x4014 {
            todo!() // TODO: implement OAMDMA
        }
        let out = match address % 0x8 {
            0x2 => {
                if !peek {
                    self.is_writing_low_byte = false;
                }
                let out = self.status_register.get_bitmasked(!status_flags::OPEN_BUS)
                    + (self.open_bus & status_flags::OPEN_BUS);
                if !peek {
                    self.status_register
                        .set_flag_enabled(status_flags::VBLANK, false);
                }
                out
            }
            // TODO: OAMDATA
            0x7 => {
                // TODO: pallete memory is handled differently: https://www.nesdev.org/wiki/PPU_registers#:~:text=the%20next%20read.-,reading%20palette%20ram,-Later%20PPUs%20added
                // if address < 0x3F00 {
                let out = self.ppu_data_read_buffer;
                if !peek {
                    self.ppu_data_read_buffer = self.read_ppu_bus(self.vram_address);
                }
                out
                // } else {
                // }
            }
            _ => self.open_bus, // TODO: impl rest of registers
        };
        if !peek {
            self.open_bus = out;
        }
        out
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        self.open_bus = value;

        if address == 0x4014 {
            todo!() // TODO: implement OAMDMA
        }
        match address % 0x8 {
            // TODO: IMPL PROPERLY
            0x0 => {
                self.control_register = value;
                let base_nametable_address = self
                    .control_register
                    .get_bitfield(control_flags::BASE_NAMETABLE_ADDRESS);
                self.temp_vram_address
                    .set_bitfield(BASE_NAMETABLE_ADDRESS, base_nametable_address as u16);
            }
            0x1 => {
                self.mask_register = value;
            }
            0x5 => {
                if !self.is_writing_low_byte {
                    self.fine_x = value & 0b111;
                    self.temp_vram_address
                        .set_bitfield(COARSE_X, value as u16 >> 3);
                    self.is_writing_low_byte = true;
                } else {
                    self.temp_vram_address
                        .set_bitfield(FINE_Y, value as u16 & 0b111);
                    self.temp_vram_address
                        .set_bitfield(COARSE_Y, value as u16 >> 3);
                    self.is_writing_low_byte = false;
                }
            }
            0x6 => {
                if !self.is_writing_low_byte {
                    self.temp_vram_address =
                        ((value as u16) << 8) + (self.temp_vram_address.get_bitmasked(0xFF));
                    self.is_writing_low_byte = true;
                } else {
                    self.temp_vram_address =
                        (self.temp_vram_address.get_bitmasked(0xFF00)) + (value as u16);
                    self.is_writing_low_byte = false;
                    self.vram_address = self.temp_vram_address;
                }
            }
            0x7 => {
                self.write(self.vram_address, value);

                let mut inc_ammount = 1;
                if self
                    .control_register
                    .get_flag_enabled(control_flags::VRAM_INC)
                {
                    inc_ammount = 32;
                }
                self.vram_address = self.vram_address.wrapping_add(inc_ammount);
            }
            _ => (), // TODO: impl rest of register writes
        };
    }

    pub fn read_ppu_bus(&self, address: u16) -> u8 {
        let result = match address {
            0x0..0x2000 => self
                .cartrige
                .as_ref()
                .map(|c| c.borrow_mut().read(CartrigeAccess::PpuAccess { address }))
                .flatten()
                .unwrap_or(0x0),
            0x2000..0x3F00 => {
                self.nametable_memory[self.map_nametable_address(address) as usize - 0x2000]
            }
            0x3F00..0x4000 => {
                println!("accessing 1");
                self.pallet_memory.read_address(address)
            }
            _ => 0,
        };
        return result;
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0..0x2000 => {
                _ = self.cartrige.as_ref().map(|c| {
                    c.borrow_mut()
                        .write(CartrigeAccess::PpuAccess { address }, value)
                })
            }
            0x2000..0x3F00 => {
                self.nametable_memory[self.map_nametable_address(address) as usize - 0x2000] = value
            }
            0x3F00..0x4000 => {
                self.pallet_memory.write_address(address, value);
            }
            _ => (),
        };
    }

    pub fn tick(&mut self) -> Option<(u32, u32, u8, u8)> {
        let enabled_rendering = self
            .mask_register
            .get_flag_enabled(mask_flags::ENABLE_BG_RENDERING)
            || self
                .mask_register
                .get_flag_enabled(mask_flags::ENABLE_SPRITE_RENDERING);

        let scanline_visible = matches!(self.scanline, (0..240) | 261);
        let dot_fetch = matches!(self.dot, (1..258) | (321..337));

        if enabled_rendering {
            // https://www.nesdev.org/w/images/default/4/4f/Ppu.svg
            if scanline_visible && dot_fetch {
                self.renderer_shift_attribute_lsb <<= 1;
                self.renderer_shift_attribute_msb <<= 1;
                self.renderer_shift_pattern_lsb <<= 1;
                self.renderer_shift_pattern_msb <<= 1;

                match (self.dot - 1) % 8 + 1 {
                    // load shifters + last tick of NT
                    2 => {
                        self.renderer_sprite_id = self.read_ppu_bus(
                            0x2000 | (self.vram_address.get_bitfield(NAMETABLE_OFFSET)),
                        )
                    }
                    // last tick of AT
                    4 => {
                        let mut attributes = self.read_ppu_bus(
                            0x23C0
                                | self.vram_address.get_bitmasked(BASE_NAMETABLE_ADDRESS)
                                | (self.vram_address.get_bitfield(COARSE_X) >> 2)
                                | (self.vram_address.get_bitfield(COARSE_Y) >> 2 << 3),
                        );

                        if (self.vram_address.get_bitfield(COARSE_Y) & 2) != 0 {
                            attributes >>= 4;
                        }
                        if (self.vram_address.get_bitfield(COARSE_X) & 2) != 0 {
                            attributes >>= 2;
                        }
                        self.renderer_attribute_msb = (attributes >> 1) & 1;
                        self.renderer_attribute_lsb = attributes & 1;
                    }
                    // last tick of BG LSBIT
                    6 => {
                        // info on pattern tables: https://www.nesdev.org/wiki/PPU_pattern_tables
                        self.renderer_pattern_lsb = self.read_ppu_bus(
                            self.get_background_nametable_address()
                                + self.renderer_sprite_id as u16 * 16
                                + self.vram_address.get_bitfield(FINE_Y),
                        )
                    }
                    // last tick of BG MSBIT + increment horizontaly/vertically
                    8 => {
                        self.renderer_pattern_msb = self.read_ppu_bus(
                            self.get_background_nametable_address()
                                + self.renderer_sprite_id as u16 * 16
                                + self.vram_address.get_bitfield(FINE_Y)
                                + 8,
                        );

                        // read more about incrementation: https://www.nesdev.org/wiki/PPU_scrolling#Wrapping_around
                        let mut coarse_x = self.vram_address.get_bitfield(COARSE_X);
                        if coarse_x == 31 {
                            self.vram_address ^= BASE_NAMETABLE_ADDRESS_X;
                            coarse_x = 0;
                        } else {
                            coarse_x += 1;
                        }
                        self.vram_address.set_bitfield(COARSE_X, coarse_x);

                        if self.dot == 256 {
                            let mut fine_y = self.vram_address.get_bitfield(FINE_Y);
                            if fine_y < 7 {
                                fine_y += 1;
                            } else {
                                fine_y = 0;
                                let mut coarse_y = self.vram_address.get_bitfield(COARSE_Y);
                                if coarse_y == 29 {
                                    coarse_y = 0;
                                    self.vram_address ^= BASE_NAMETABLE_ADDRESS_Y;
                                } else if coarse_y == 31 {
                                    coarse_y = 0;
                                } else {
                                    coarse_y += 1;
                                }
                                self.vram_address.set_bitfield(COARSE_Y, coarse_y);
                            }
                            self.vram_address.set_bitfield(FINE_Y, fine_y);
                        }

                        self.renderer_shift_pattern_msb = (self.renderer_shift_pattern_msb
                            & 0xFF00)
                            | self.renderer_pattern_msb as u16;
                        self.renderer_shift_pattern_lsb = (self.renderer_shift_pattern_lsb
                            & 0xFF00)
                            | self.renderer_pattern_lsb as u16;

                        self.renderer_shift_attribute_msb = (self.renderer_shift_attribute_msb
                            & 0xFF00)
                            | self.renderer_attribute_msb as u16 * 0xFF;
                        self.renderer_shift_attribute_lsb = (self.renderer_shift_attribute_lsb
                            & 0xFF00)
                            | self.renderer_attribute_lsb as u16 * 0xFF;
                    }
                    _ => (),
                }
            }

            if scanline_visible && self.dot == 257 {
                self.vram_address.set_bitmasked(
                    COARSE_X | BASE_NAMETABLE_ADDRESS_X,
                    self.temp_vram_address
                        .get_bitmasked(COARSE_X | BASE_NAMETABLE_ADDRESS_X),
                );
            }
        }

        if self.scanline == 241 && self.dot == 1 {
            if self
                .control_register
                .get_flag_enabled(control_flags::VBLANK_NMI)
                && let Some(cpu) = self.cpu.as_ref()
            {
                cpu.borrow_mut().is_triggered_nmi = true;
            }
            self.status_register
                .set_flag_enabled(status_flags::VBLANK, true);
        }
        if self.scanline == 261 && self.dot == 1 {
            self.status_register
                .set_flag_enabled(status_flags::VBLANK, false);
            self.status_register
                .set_flag_enabled(status_flags::SPRITE_0_HIT, false);
            self.status_register
                .set_flag_enabled(status_flags::SPRITE_OVERFLOW, false);
        }
        if enabled_rendering && self.scanline == 261 && matches!(self.dot, (280..305)) {
            self.vram_address.set_bitmasked(
                COARSE_Y | FINE_Y | BASE_NAMETABLE_ADDRESS_Y,
                self.temp_vram_address
                    .get_bitmasked(COARSE_Y | FINE_Y | BASE_NAMETABLE_ADDRESS_Y),
            );
        }

        let mut out = None;

        if enabled_rendering && matches!(self.dot, (1..=256)) && matches!(self.scanline, (0..=239))
        {
            let fine_x_selector = 1 << (15 - self.fine_x);

            let pattern_lsb = self
                .renderer_shift_pattern_lsb
                .get_flag_enabled(fine_x_selector) as u8;
            let pattern_msb = self
                .renderer_shift_pattern_msb
                .get_flag_enabled(fine_x_selector) as u8;

            let pattern = (pattern_msb << 1) | pattern_lsb;

            let attrib_lsb = self
                .renderer_shift_attribute_lsb
                .get_flag_enabled(fine_x_selector) as u8;
            let attrib_msb = self
                .renderer_shift_attribute_msb
                .get_flag_enabled(fine_x_selector) as u8;

            let attrib = (attrib_msb << 1) | attrib_lsb;

            out = Some((self.dot - 1, self.scanline, pattern, attrib));
        }

        if enabled_rendering && self.scanline == 261 && self.dot == 339 && self.is_odd_frame {
            self.dot = 0;
            self.scanline = 0;
            self.is_odd_frame = !self.is_odd_frame;
        } else {
            self.dot += 1;
            if self.dot > 340 {
                self.scanline += 1;
                if self.scanline > 261 {
                    self.scanline = 0;
                    self.is_odd_frame = !self.is_odd_frame;
                }
                self.dot = 0;
            }
        }

        out
    }

    pub fn get_pixel_color(&self, i: usize, j: usize) -> u32 {
        let i_tile = i / 8;
        let j_tile = j / 8;
        let index = i_tile * 32 + j_tile;
        let sprite = self.nametable_memory[index];
        let pixel_i = i % 8;
        let pixel_j = j % 8;
        let pallet_collor_id = self.get_sprite_pixel_pallet(sprite, pixel_i as u8, pixel_j as u8);

        let attr_index = i_tile / 4 * 8 + j_tile / 4;
        let attr_value = self.nametable_memory[0x3c0 + attr_index as usize];
        let shift = ((i_tile / 2) % 2) * 4 + ((j_tile / 2) % 2) * 2;
        let pallet_index = (attr_value >> shift) & 0b11;
        let color_id = self
            .pallet_memory
            .read_index(pallet_index as u16, pallet_collor_id as u16);
        constants::ppu::COLORS[color_id as usize]
    }

    fn get_background_nametable_address(&self) -> u16 {
        if self
            .control_register
            .get_flag_enabled(control_flags::BG_PATTERN_TABLE_ADDR)
        {
            0x1000
        } else {
            0x0
        }
    }

    fn get_sprite_pixel_pallet(&self, sprite: u8, pixel_i: u8, pixel_j: u8) -> u8 {
        let mut background_nametable_address = 0;
        if self
            .control_register
            .get_flag_enabled(control_flags::BG_PATTERN_TABLE_ADDR)
        {
            background_nametable_address = 0x1000;
        }
        let first_byte = self
            .read_cartrige(background_nametable_address + (sprite as u16) * 16 + pixel_i as u16);
        let second_byte = self.read_cartrige(
            background_nametable_address + (sprite as u16) * 16 + pixel_i as u16 + 8,
        );

        let lsb = (first_byte >> (7 - pixel_j)) & 1;
        let msb = (second_byte >> (7 - pixel_j)) & 1;
        (msb << 1) + lsb
    }

    fn read_cartrige(&self, address: u16) -> u8 {
        self.cartrige
            .as_ref()
            .map(|c| c.borrow_mut().read(CartrigeAccess::PpuAccess { address }))
            .flatten()
            .unwrap_or(0)
    }

    pub fn process_pattern_table(&self) -> PatternTable {
        let mut out: [[[[u8; 8]; 8]; 16]; 32] = [[[[0; 8]; 8]; 16]; 32];
        for i in 0..32 {
            for j in 0..16 {
                out[i as usize][j as usize] = self.process_sprite(i, j);
            }
        }
        out
    }

    fn process_sprite(&self, sprite_i: u16, sprite_j: u16) -> BackgroundSprite {
        let mut out = [[0; 8]; 8];
        for i in 0..8 {
            let first_byte_address = sprite_i * 256 + sprite_j * 16 + i;
            let second_byte_address = first_byte_address + 8;
            let first_byte = self
                .cartrige
                .as_ref()
                .map(|c| {
                    c.borrow_mut().read(CartrigeAccess::PpuAccess {
                        address: first_byte_address,
                    })
                })
                .flatten()
                .unwrap_or(0);

            let second_byte = self
                .cartrige
                .as_ref()
                .map(|c| {
                    c.borrow_mut().read(CartrigeAccess::PpuAccess {
                        address: second_byte_address,
                    })
                })
                .flatten()
                .unwrap_or(0);

            for j in 0..8 {
                let lsb = (first_byte >> (7 - j)) & 1;
                let msb = (second_byte >> (7 - j)) & 1;

                let palette = (msb << 1) + lsb;
                out[i as usize][j as usize] = palette;
            }
        }
        out
    }

    fn map_nametable_address(&self, address: u16) -> u16 {
        self.cartrige
            .as_ref()
            .map(|c| c.borrow().map_nametable(address))
            .unwrap_or_else(|| address)
    }
}

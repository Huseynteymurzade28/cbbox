use rand::Rng;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; REGISTERS_COUNT],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Emu {
            pc: 0x200,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; REGISTERS_COUNT],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
        };

        new_emu.ram[..80].copy_from_slice(&FONTSET);

        new_emu.ram[0x200] = 0xC0;
        new_emu.ram[0x201] = 0xFF;

        // 8XY4 -> 8014 -> V0 = V0 + V1 (Toplama testi için)
        // Önce V1'e bir değer verelim (Manuel)
        new_emu.v_reg[1] = 10;
        new_emu.ram[0x202] = 0x80;
        new_emu.ram[0x203] = 0x14;

        new_emu
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        let high_byte = self.ram[self.pc as usize] as u16;
        let low_byte = self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        (high_byte << 8) | low_byte
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // 0000 - NOP (No Operation)
            (0, 0, 0, 0) => return,

            // 00E0 - CLS (Clear Screen)
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }

            // 00EE - RET (Return from subroutine)
            (0, 0, 0xE, 0xE) => {
                if self.sp == 0 {
                    panic!("Stack Underflow! No return address found.");
                }
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            // 1NNN - JMP NNN (Jump to address NNN)
            (1, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.pc = nnn;
            }

            // 2NNN - CALL NNN (Call subroutine at NNN)
            (2, _, _, _) => {
                let nnn = op & 0x0FFF;
                // Store current PC to stack
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                // Jump to address
                self.pc = nnn;
            }

            // 3XNN - Skip next instruction if VX == NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            }

            // 4XNN - Skip next instruction if VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            }

            // 5XY0 - Skip next instruction if VX == VY
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }

            // 6XNN - SET VX = NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x] = nn;
            }

            // 7XNN - ADD VX, NN (Does not affect carry flag)
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }

            // 8XY0 - SET VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            }

            // 8XY1 - OR VX, VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            }

            // 8XY2 - AND VX, VY
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            }

            // 8XY3 - XOR VX, VY
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            }

            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (result, overflow) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                self.v_reg[x] = result;
                self.v_reg[0xF] = if overflow { 1 } else { 0 };
            }

            // 8XY5 - SUB VX, VY (Set VF = NOT borrow)
            // Eğer VX > VY ise VF = 1 (Borç almaya gerek yok)
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                self.v_reg[0xF] = if self.v_reg[x] > self.v_reg[y] { 1 } else { 0 };
                self.v_reg[x] = self.v_reg[x].wrapping_sub(self.v_reg[y]);
            }

            // 8XY6 - SHR VX (Shift Right by 1)
            // En sağdaki bit (LSB) VF'ye kaydedilir, sonra sayı 2'ye bölünür.
            (8, _, _, 6) => {
                let x = digit2 as usize;
                self.v_reg[0xF] = self.v_reg[x] & 0x1;
                self.v_reg[x] >>= 1;
            }

            // 8XY7 - SUBN VX, VY (Set VX = VY - VX)
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                self.v_reg[0xF] = if self.v_reg[y] > self.v_reg[x] { 1 } else { 0 };
                self.v_reg[x] = self.v_reg[y].wrapping_sub(self.v_reg[x]);
            }

            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                self.v_reg[0xF] = (self.v_reg[x] >> 7) & 0x1;
                self.v_reg[x] <<= 1;
            }

            // 9XY0 - Skip next instruction if VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }

            // ANNN - SET I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.i_reg = nnn;
            }

            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                let rng: u8 = rand::random();
                self.v_reg[x] = rng & nn;
            }

            // DXYN - DRAW (Display n-byte sprite starting at memory location I at (VX, VY))
            (0xD, _, _, _) => {
                let x_idx = digit2 as usize;
                let y_idx = digit3 as usize;
                let height = digit4 as u16;

                let x_coord = self.v_reg[x_idx] as u16;
                let y_coord = self.v_reg[y_idx] as u16;

                // Reset collision flag
                self.v_reg[0xF] = 0;

                for row in 0..height {
                    let sprite_byte = self.ram[(self.i_reg + row) as usize];
                    for col in 0..8 {
                        let pixel_bit = (sprite_byte >> (7 - col)) & 1;
                        if pixel_bit != 0 {
                            let draw_x = (x_coord + col) as usize;
                            let draw_y = (y_coord + row) as usize;

                            // Standard Chip-8 clipping
                            if draw_x < SCREEN_WIDTH && draw_y < SCREEN_HEIGHT {
                                let idx = draw_y * SCREEN_WIDTH + draw_x;
                                if self.screen[idx] {
                                    self.v_reg[0xF] = 1;
                                }
                                self.screen[idx] ^= true;
                            }
                        }
                    }
                }
            }

            // Unhandled Opcode
            _ => println!("Unimplemented Opcode: {:04X}", op),
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }
    pub fn load_rom(&mut self, data: &[u8]) {
        let start = 0x200;
        let end = 0x200 + data.len();

        if end > RAM_SIZE {
            panic!("ROM too large for RAM");
        }

        self.ram[start..end].copy_from_slice(data);
    }
    // For test math operations
    /*  pub fn test_math_setup(&mut self) {
        self.pc = 0x200;
        self.v_reg[1] = 10;


        self.ram[0x200] = 0xC0;
        self.ram[0x201] = 0xFF;

        self.ram[0x202] = 0x80;
        self.ram[0x203] = 0x14;
    } */
}

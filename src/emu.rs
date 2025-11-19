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
            // NOP
            (0, 0, 0, 0) => return,

            // CLS - Clear screen
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }

            // JMP NNN
            (1, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.pc = nnn;
            }

            // SET VX, NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x] = nn;
            }

            // ADD VX, NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            // SET I
            (0xA, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.i_reg = nnn;
            }

            // DXYN - Draw sprite
            (0xD, _, _, _) => {
                let x_reg_idx = digit2 as usize;
                let y_reg_idx = digit3 as usize;
                let height = digit4 as u16;

                let x_coord = self.v_reg[x_reg_idx] as u16;
                let y_coord = self.v_reg[y_reg_idx] as u16;

                self.v_reg[0xF] = 0;

                for row in 0..height {
                    let sprite_byte = self.ram[(self.i_reg + row) as usize];

                    for col in 0..8 {
                        let pixel_bit = (sprite_byte >> (7 - col)) & 1;

                        if pixel_bit != 0 {
                            let draw_x = (x_coord + col) as usize;
                            let draw_y = (y_coord + row) as usize;

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

            _ => println!("Unknown opcode: {:04X}", op),
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
}

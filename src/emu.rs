pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

// 0-F arasındaki karakterlerin hex piksel karşılıkları (Her sayı 5 byte yer kaplar)
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

        // Fontları RAM'in başına yükle
        new_emu.ram[..80].copy_from_slice(&FONTSET);

        // --- GÖRSEL TEST ROM ---
        
        // 1. I Registerını Font setinin başına (0 karakterine) ayarla
        // Opcode: A000
        new_emu.ram[0x200] = 0xA0;
        new_emu.ram[0x201] = 0x00;

        // 2. Koordinatları ayarla (V0=0, V1=0) -> Sol üst köşe
        // Opcode: 6000 (V0 = 0)
        new_emu.ram[0x202] = 0x60;
        new_emu.ram[0x203] = 0x00;
        // Opcode: 6100 (V1 = 0)
        new_emu.ram[0x204] = 0x61;
        new_emu.ram[0x205] = 0x00;

        // 3. Çiz! (V0, V1 koordinatına, 5 satır yüksekliğinde)
        // Opcode: D015
        new_emu.ram[0x206] = 0xD0;
        new_emu.ram[0x207] = 0x15;
        // -----------------------

        new_emu
    }

    pub fn tick(&mut self) {
        // 1. Fetch
        let op = self.fetch();
        // 2. Decode & Execute
        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        let high_byte = self.ram[self.pc as usize] as u16;
        let low_byte = self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;
        (high_byte << 8) | low_byte
    }

    fn execute(&mut self, op: u16) {
        // Opcode'u parçalarına ayır (Nibbles)
        // Örnek Opcode: 0x7C15
        let digit1 = (op & 0xF000) >> 12; // 0x7
        let digit2 = (op & 0x0F00) >> 8; // 0xC (X register)
        let digit3 = (op & 0x00F0) >> 4; // 0x1 (Y register)
        let digit4 = op & 0x000F; // 0x5 (N sabiti)

        match (digit1, digit2, digit3, digit4) {
            // 0000 - NOP (Hiçbir şey yapma)
            (0, 0, 0, 0) => return,

            // 00E0 - CLS (Ekranı temizle)
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }

            // 1NNN - JMP NNN (Adrese atla)
            (1, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.pc = nnn;
            }

            // 6XNN - SET VX, NN (V[x] registerına NN değerini ata)
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x] = nn;
            }

            // 7XNN - ADD VX, NN (V[x] registerına NN değerini ekle)
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                // wrapping_add önemli, çünkü 255'i geçerse hata vermemeli, başa dönmeli (overflow)
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            // ANNN - Set Index (I registerını ayarla)
            (0xA, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.i_reg = nnn;
            }

            // DXYN - Draw Sprite
            // (VX, VY) koordinatına N byte yüksekliğinde şekil çiz
            (0xD, _, _, _) => {
                // Opcode'dan parametreleri al
                let x_reg_idx = digit2 as usize;
                let y_reg_idx = digit3 as usize;
                let height = digit4 as u16; // Şeklin yüksekliği (satır sayısı)

                // Koordinatları al
                let x_coord = self.v_reg[x_reg_idx] as u16;
                let y_coord = self.v_reg[y_reg_idx] as u16;

                // Çarpışma bayrağını (VF) sıfırla. (Bir piksel silinirse 1 olacak)
                self.v_reg[0xF] = 0;

                // Y ekseninde (Satırlar) döngü
                for row in 0..height {
                    // Hafızadan o satırın verisini (byte) al
                    // I register adresi + o anki satır
                    let sprite_byte = self.ram[(self.i_reg + row) as usize];

                    // X ekseninde (Pikseller) döngü (Bir byte = 8 bit)
                    for col in 0..8 {
                        // O anki bit 1 mi? (En soldan başlayarak kontrol et)
                        let pixel_bit = (sprite_byte >> (7 - col)) & 1;

                        // Eğer çizilecek piksel doluysa (1 ise) işlem yap
                        if pixel_bit != 0 {
                            let draw_x = (x_coord + col) as usize;
                            let draw_y = (y_coord + row) as usize;

                            // Ekranın dışına taşmıyorsak çiz
                            if draw_x < SCREEN_WIDTH && draw_y < SCREEN_HEIGHT {
                                let idx = draw_y * SCREEN_WIDTH + draw_x;

                                // Çarpışma Kontrolü:
                                // Eğer ekrandaki piksel zaten beyazsa (true) ve biz de beyaz (1) çiziyorsak
                                // XOR kuralı gereği orası siyaha döner (silinir).
                                // Bu bir "çarpışma"dır. VF = 1 yaparız.
                                if self.screen[idx] {
                                    self.v_reg[0xF] = 1;
                                }

                                // XOR işlemi: Pikseli tersle
                                self.screen[idx] ^= true;
                            }
                        }
                    }
                }
            }

            _ => println!("Tanımsız Opcode: {:X}", op),
        }
        println!("V1: {}", self.v_reg[1]);
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }
}

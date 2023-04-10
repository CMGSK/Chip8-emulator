const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;
const FONT_SIZE: usize = 80;

const FONTSET: [u8; FONT_SIZE] = [
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

pub struct Emulator {
    ram: [u8; RAM_SIZE],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    pc: u16,
    sp: u16,
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    keys: [bool; NUM_KEYS],
    st: u8,
    dt: u8,
}

impl Emulator {
    pub fn new() -> Self {
        let mut new_emulator = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            keys: [false; NUM_KEYS],
            i_reg: 0,
            v_reg: [0; NUM_REGS],
            stack: [0; STACK_SIZE],
            sp: 0,
            dt: 0,
            st: 0,
        };

        new_emulator.ram[..FONT_SIZE].copy_from_slice(&FONTSET);
        new_emulator
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte: u16 = self.ram[self.pc as usize] as u16;
        let lower_byte: u16 = self.ram[(self.pc + 1) as usize] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        opcode
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    fn execute(&mut self, opcode: u16) {
        let digit1 = (opcode & 0xF000) >> 12;
        let digit2 = (opcode & 0x0F00) >> 8;
        let digit3 = (opcode & 0x00F0) >> 4;
        let digit4 = opcode & 0x000F;
        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            (1, _, _, _) => {
                let nnn = opcode & 0xFFF;
                self.pc = nnn;
            }
            (6, _, _, _) => {
                let x = digit2 as usize;
                let kk = (opcode & 0xFF) as u8;
                self.v_reg[x] = kk;
            }
            (7, _, _, _) => {
                let x = digit2 as usize;
                let kk = (opcode & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(kk);
            }
            (0xA, _, _, _) => {
                let nnn = opcode & 0xFFF;
                self.i_reg = nnn;
            }
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;
                let n_rows = digit4;
                let mut flipped: bool = false;
                for y_line in 0..n_rows {
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];
                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;
                            let pixel_index = x + SCREEN_WIDTH * y;
                            flipped = flipped | self.screen[pixel_index];
                            self.screen[pixel_index] = true;
                        }
                    }
                }
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented Opcode: {:#06x}", opcode),
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
}

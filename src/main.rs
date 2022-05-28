
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // Rust allows to use `usize` as index
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        
        // If not OR operation is not done then the left shift sets all
        // of the bits to 0
        // Example
        // 8  = 0000 1000 
        // 20 = 0001 0100
        // OR result = 28
        println!("{} and {} for OR operation = {}", op_byte1, op_byte2, 8 | op_byte2);
        
        // First do `op_byte1 << 8`
        // then `| op_byte2`
        println!("Result = {}", op_byte1 << 8 | op_byte2);
        op_byte1 << 8 | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        // `overflowing_add` returns an u8 value and a bool value
        // the former holds the result and the latter indicates
        // if an overflow was detected
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

    }
    
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;
            
            // Extract nibbles from opcode (CHIP-8 standard)
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            
            match (c, x, y, d) {
                (0, 0, 0, 0)     => { return; },
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _                => todo!("opcode {:04x}", opcode),
            }
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    // These are the operands or the numbers to use for adding operation
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    // The opcode 0x8014 adds register 1 to register 0 
    // register[0] + register[1]
    mem[0] = 0x80; mem[1] = 0x14;
    
    // The opcode 0x8024 adds register 2 to register 0
    // register[0] + register[2]
    mem[2] = 0x80; mem[3] = 0x24;
    
    // The opcode 0x8034 adds register 3 to register 0
    // register[0] + register[3]
    mem[4] = 0x80; mem[5] = 0x34;


    // Execute the operation
    cpu.run();
    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}

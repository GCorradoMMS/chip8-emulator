  
struct Chip8 {
  program_counter: u16,                  // Program counter starts at 0x200
  opcode: u16,                               // Reset current opcode	
  index_register: u16,                       // Reset index register
  stack: u16,                      // Reset stack pointer
  sp: u16,
  memory: Vec<u8>,
  cpu_registers: [u8 ; 16],
  gfx: [u8; 2048],
  delay_timer: u8,
  sound_timer: u8,
  keypad: u8,
  chip8_fontset : [u8 ; 80] 
}

  impl Chip8 {
    fn push_to_memory(&mut self, fontset: u8 ) {
      self.memory.push(fontset);
    }
  }
  
  pub fn initialize() //i think this should be a struct
  {
    let mut cpu = Chip8{
      program_counter: 0x200,          
      opcode: 0,                   
      index_register: 0,           
      stack: 0,                    
      sp: 0,
      memory: Vec::with_capacity(4096),
      cpu_registers: [0,1,2,3,4,5,6,7,8,9,0xA,0xB,0xC,0xD,0xE,0xF],
      gfx: [0; 2048],
      delay_timer: 0,
      sound_timer: 0,
      keypad: 0,
      chip8_fontset : [ 
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
         0xF0, 0x80, 0xF0, 0x80, 0x80  // F
      ] 
    };

      // let mut _program_counter : u16 = 0x200;                  // Program counter starts at 0x200
      // let mut _opcode : u16 = 0;                               // Reset current opcode	
      // let mut _index_register : u16 = 0;                       // Reset index register
      // let mut _stack : u16 = 0;                          // Reset stack pointer
      // let mut _sp : u16;
      // let mut _memory : Vec<u8> = Vec::with_capacity(4096);
      // let mut _cpu_registers : [u8 ; 16];
      // let mut _gfx : [u8; 2048];
      // let mut _delay_timer : u8;
      // let mut _sound_timer : u8;
      // let mut _keypad : u8;
      // let mut _chip8_fontset : [u8 ; 80] = 
      // [ 
        // 0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        // 0x20, 0x60, 0x20, 0x20, 0x70, // 1
        // 0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        // 0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        // 0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        // 0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        // 0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        // 0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        // 0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        // 0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        // 0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        // 0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        // 0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        // 0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        // 0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        // 0xF0, 0x80, 0xF0, 0x80, 0x80  // F
      // ];

      cpu.chip8_fontset.map(|x| cpu.memory.push(x));
      // chip8.chip8_fontset.map(|x| _memory.push(x));
      println!("{:?}", cpu.memory);
  }

  pub fn emulate_cycle(opcode: u16)
  {

    match opcode & 0xF000
    {
      0x0000 => println!("Implement"),
      0x000E => println!("Implement"),
      _      => println!("Invalid opcode {}", opcode)
    }

  }
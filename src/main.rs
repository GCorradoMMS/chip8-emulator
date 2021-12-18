use std::fs::File;
use std::io::Read;
mod chip8; //######### CPU CORE

fn main() {
  
  // let mut file=File::open("Cargo.toml").unwrap(); experimenting with binary reading
  // let mut buf=[0u8;12];
  // file.read(&mut buf).unwrap();

  chip8::initialize();
  // Set up render system and register input callbacks
//   setupGraphics();
//   setupInput();
 
  // Initialize the Chip8 system and load the game into the memory  
//   myChip8.initialize();
//   myChip8.loadGame("pong"); -> Take with args
 
  // Emulation loop
//   for(;;) -> LOOP
//   {
//     // Emulate one cycle
//     myChip8.emulateCycle();
 
//     // If the draw flag is set, update the screen
//     if(myChip8.drawFlag)
//       drawGraphics();
 
//     // Store key press state (Press and Release)
//     myChip8.setKeys();	
//   }


// 0x00E0 – Clears the screen
// 0xDXYN – Draws a sprite on the screen
}
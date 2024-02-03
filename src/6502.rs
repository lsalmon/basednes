pub struct CPU {
	// Registers
	r_SP: u8,
	r_A: u8,
	r_X: u8,
	r_Y: u8,

	// Program Counter
	r_PC: u16,

	// Status Flags
	f_C: bool,
	f_Z: bool,
	f_I: bool,
	f_D: bool,
	f_V: bool,
	f_N: bool,

	m_pendingNMI: bool,
	m_pendingIRQ: bool, 
	
	m_skipCycles: i32,
	m_cycles: i32,
}

enum InterruptType {
	IRQ,
	NMI,
	BRK_,
}

impl CPU {
	fn step() {
	
	}

	fn reset(addr: u16) {
	
	}

	fn getPC(&self) -> u16 {
		self.r_PC
	}
	
	fn interrupt(type: InterruptType) {
	
	}

	fn executeI(opcode: u8) {
	
	}
	fn executeB(opcode: u8) {

	}
	fn executeT0(opcode: u8) {

	}
	fn executeT1(opcode: u8) {

	}
	fn executeT2(opcode: u8) {
		
	}
	
	fn readAddr(addr: u16) {
	
	}

	fn pushStack(value: u8) {

	}
	
	fn pullStack() -> u8 {

	}
}

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

	// Memory containing CPU instructions
	// TODO: use real memory struct
	mem: Vec<u8>,
}

enum InterruptType {
	IRQ,
	NMI,
	BRK_,
}

enum OpcodeType {
	ORA,
	AND,
	EOR,
	ADC,
	STA,
	LDA,
	CMP,
	LBC
}

enum Addressing {
	IMM,
	ZPG,
	ZPG_X,
	ABS,
	ABS_X,
	ABS_Y,
	IND_X,
	IND_Y
}

impl CPU {
	fn alu(addr_mode: u8, opcode_index: u8) {
		let addressing : Addressing = match addr_mode {
			0 => IND_X,
			1 => ZPG,
			2 => IMM,
			3 => ABS,
			4 => IND_Y,
			5 => ZPG_X,
			6 => ABS_Y,
			7 => ABS_X
		};
		let operation : OpcodeType = match opcode_index {
			0 => ORA,
			1 => AND,
			2 => EOR,
			3 => ADC,
			4 => STA,
			5 => LDA,
			6 => CMP,
			7 => LBC
		};
	}

	fn step() {
		if self.m_cycles > 0 {
			self.m_cycles = self.m_cycles - 1;
			return;
		}

		let inst = match self.mem.get(self.r_PC) {
			None => {
				self.r_PC = self.r_PC.wrapping_add(1);
				return;
			},
			Some(inst) => inst
		};

		let inst_type = inst & 0b0000_0011;
		let addr_mode = (inst & 0b0001_1100) >> 2;
		let opcode_index = (inst & 0b1110_0000) >> 5;

		match inst_type {
			0b00 => None, // Control block, TODO
			0b01 => self.alu(addr_mode, opcode_index)
		}

		self.r_PC = self.r_PC.wrapping_add(1);
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

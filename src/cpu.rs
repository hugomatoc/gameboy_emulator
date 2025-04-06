pub struct CPU {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0x100, // Punto de entrada típico de la ROM
            sp: 0xFFFE,
        }
    }

    pub fn tick(&mut self) {
        // Leer opcode, ejecutar, avanzar ciclo
    }
}

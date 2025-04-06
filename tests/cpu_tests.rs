use gameboy_emulator::cpu::CPU; // Importar la CPU desde el módulo

#[test]
fn test_addition() {
    let mut cpu = CPU::new();
    cpu.a = 1;
    cpu.b = 2;
    // Ejecutar opcode 0x80 (ADD A, B)
    // Verificar resultado en cpu.a
}

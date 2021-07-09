pub mod memory_builder {
    use super::super::memory::memory::Memory;

    pub fn build_new_memory() -> Memory {
        Memory::new()
    }
}

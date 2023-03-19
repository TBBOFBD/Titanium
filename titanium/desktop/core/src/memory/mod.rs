pub type MemResult<T> = std::io::Result<T>;

titaniumutils::allow!(
    target_os = "windows",
    "This module is only available on Windows"
);

mod raw;

pub struct MemoryManager {
    pub(crate) raw: raw::RawMemoryManager
}

impl MemoryManager {
    pub fn new<T: ToString>(name: T) -> MemResult<Self> {
        let name: String = name.to_string();
        let raw = raw::RawMemoryManager::new(name)?;
        Ok(Self { raw })
    }

    pub fn write<T>(&self, address: u32, value: T) -> MemResult<()> {
        self.raw.write_memory(address, value)
    }

    pub fn read<T>(&self, address: u32) -> MemResult<T> {
        self.raw.read_memory(address)
    }
    
    pub fn get_module_address<T: ToString>(&self, name: T) -> MemResult<u32> {
        let name: String = name.to_string();
        self.raw.get_module_address(name)
    }

    pub fn read_raw<T>(address: u64) -> MemResult<T> {
        raw::RawMemoryManager::read_raw(address)
    }
}

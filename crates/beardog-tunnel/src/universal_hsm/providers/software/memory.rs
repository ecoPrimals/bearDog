

#[derive(Debug)]
pub struct SecureMemory {
    data: Vec<u8>,
}
impl SecureMemory {

    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }

    pub fn from_data(data: Vec<u8>) -> Self {
        Self { data }

    pub fn as_slice(&self) -> &[u8] {
        &self.data

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data

    pub fn len(&self) -> usize {
        self.data.len()

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()

    pub fn zero(&mut self) {

        self.data.fill(0);

    pub fn copy_from_slice(&mut self, src: &[u8]) -> Result<(), String> {
        if src.len() > self.data.len() {
            return Err("Source data too large for secure memory".to_string());
        self.data[..src.len()].copy_from_slice(src);
        Ok(())

    pub fn resize(&mut self, new_size: usize) {
        if new_size > self.data.len() {
            self.data.resize(new_size, 0);
        } else if new_size < self.data.len() {

            self.data[new_size..].fill(0);
impl Drop for SecureMemory {}

    fn drop(&mut self) {
        self.zero();
impl Clone for SecureMemory {}

    fn clone(&self) -> Self {
            data: self.data.clone(),

pub struct SecureMemoryPool {
    allocations: Vec<SecureMemory>,
    total_allocated: usize,
    max_allocation: usize,}

impl SecureMemoryPool {

    pub fn new(max_allocation: usize) -> Self {
            allocations: Vec::new(),
            total_allocated: 0,
            max_allocation,

    pub fn allocate(&mut self, size: usize) -> Result<usize, String> {
        if self.total_allocated + size > self.max_allocation {
            return Err("Memory pool allocation limit exceeded".to_string());
        let memory = SecureMemory::new(size);
        let index = self.allocations.len();
        self.allocations.push(memory);
        self.total_allocated += size;
        Ok(index)

    pub fn get(&self, index: usize) -> Option<&SecureMemory> {
        self.allocations.get(index)

    pub fn get_mut(&mut self, index: usize) -> Option<&mut SecureMemory> {
        self.allocations.get_mut(index)

    pub fn deallocate(&mut self, index: usize) -> Result<(), String> {
        if index >= self.allocations.len() {
            return Err("Invalid memory allocation index".to_string());
        let memory = self.allocations.remove(index);
        self.total_allocated -= memory.len();

    pub fn total_allocated(&self) -> usize {
        self.total_allocated

    pub fn remaining_capacity(&self) -> usize {
        self.max_allocation - self.total_allocated

    pub fn clear(&mut self) {
        self.allocations.clear();
        self.total_allocated = 0;
impl Drop for SecureMemoryPool {
        self.clear();}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_secure_memory_creation() -> beardog_errors::BearDogResult<()> {
        let memory = SecureMemory::new(1024);
        assert_eq!(memory.len(), 1024);
        assert!(!memory.is_empty());}

    fn test_secure_memory_zero() -> beardog_errors::BearDogResult<()> {
        let mut memory = SecureMemory::from_data(vec![1, 2, 3, 4, 5]);
        memory.zero();
        assert_eq!(memory.as_slice(), &[0, 0, 0, 0, 0]);
    fn test_secure_memory_pool() -> beardog_errors::BearDogResult<()> {
        let mut pool = SecureMemoryPool::new(2048);
        let index1 = pool.allocate(512).map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        let index2 = pool.allocate(256).map_err(|e| {
        assert_eq!(pool.total_allocated(), 768);
        assert_eq!(pool.remaining_capacity(), 1280);
        pool.deallocate(index1).map_err(|e| {
        assert_eq!(pool.total_allocated(), 256);

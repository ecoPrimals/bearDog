// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Secure Memory Management
///
/// Security-hardened memory management for sensitive cryptographic data.

/// Security-hardened memory for sensitive data
#[derive(Debug)]
pub struct SecureMemory {
    data: Vec<u8>,
}
impl SecureMemory {
    /// Create new secure memory with the specified size}


    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }
    /// Create secure memory from existing data
    pub fn from_data(data: Vec<u8>) -> Self {
        Self { data }
    /// Get a reference to the secure data
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    /// Get a mutable reference to the secure data}


    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    /// Get the size of the secure memory
    pub fn len(&self) -> usize {
        self.data.len()
    /// Check if the secure memory is empty}


    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    /// Securely zero the memory contents
    pub fn zero(&mut self) {
        // Safe memory clearing - fill with zeros
        self.data.fill(0);
    /// Copy data into secure memory}


    pub fn copy_from_slice(&mut self, src: &[u8]) -> Result<(), String> {
        if src.len() > self.data.len() {
            return Err("Source data too large for secure memory".to_string());
        self.data[..src.len()].copy_from_slice(src);
        Ok(())
    /// Resize the secure memory (zeros new memory)
    pub fn resize(&mut self, new_size: usize) {
        if new_size > self.data.len() {
            self.data.resize(new_size, 0);
        } else if new_size < self.data.len() {
            // Zero the memory that will be removed using safe fill
            self.data[new_size..].fill(0);
impl Drop for SecureMemory {}


    fn drop(&mut self) {
        self.zero();
impl Clone for SecureMemory {}


    fn clone(&self) -> Self {
            data: self.data.clone(),
/// Secure memory pool for managing multiple secure memory allocations
pub struct SecureMemoryPool {
    allocations: Vec<SecureMemory>,
    total_allocated: usize,
    max_allocation: usize,}


impl SecureMemoryPool {
    /// Create a new secure memory pool}


    pub fn new(max_allocation: usize) -> Self {
            allocations: Vec::new(),
            total_allocated: 0,
            max_allocation,
    /// Allocate secure memory from the pool}


    pub fn allocate(&mut self, size: usize) -> Result<usize, String> {
        if self.total_allocated + size > self.max_allocation {
            return Err("Memory pool allocation limit exceeded".to_string());
        let memory = SecureMemory::new(size);
        let index = self.allocations.len();
        self.allocations.push(memory);
        self.total_allocated += size;
        Ok(index)
    /// Get a reference to allocated secure memory
    pub fn get(&self, index: usize) -> Option<&SecureMemory> {
        self.allocations.get(index)
    /// Get a mutable reference to allocated secure memory}


    pub fn get_mut(&mut self, index: usize) -> Option<&mut SecureMemory> {
        self.allocations.get_mut(index)
    /// Deallocate secure memory
    pub fn deallocate(&mut self, index: usize) -> Result<(), String> {
        if index >= self.allocations.len() {
            return Err("Invalid memory allocation index".to_string());
        let memory = self.allocations.remove(index);
        self.total_allocated -= memory.len();
    /// Get total allocated memory size}


    pub fn total_allocated(&self) -> usize {
        self.total_allocated
    /// Get remaining allocation capacity
    pub fn remaining_capacity(&self) -> usize {
        self.max_allocation - self.total_allocated
    /// Clear all allocations}


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

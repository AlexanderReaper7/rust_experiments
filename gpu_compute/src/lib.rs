#![allow(non_snake_case)]
#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

#[cfg(not(target_arch = "spirv"))]
#[macro_use]
pub extern crate spirv_std_macros;

use glam::UVec3;

#[allow(unused_attributes)]
#[spirv(compute(threads(1)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] text: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] hash: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] iter: &[u32],
) {
    // The sha specification loops in blocks of 512
    let num_loops: usize = iter[gid.x as usize] as usize;

    // Calculate where the memory offset for each kernel instance
    // which depends upon the number of iterations required by all previous
    // kernel invocations
    let mut offset = 0;
    for i in 0..gid.x as usize {
        offset += iter[i] as usize - 1;
    }

    for i in 0..num_loops {
        hash_fn(text, hash, gid.x as usize, offset, i);
    }
}

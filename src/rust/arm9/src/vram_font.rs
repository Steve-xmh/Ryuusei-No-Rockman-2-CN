use alloc::boxed::Box;
use nitro::println;

use crate::global_data;

#[derive(Debug, Default)]
pub struct VRamGraphEntry {
    graph_id: u16,
    vram_id: u16,
}

#[derive(Debug, Default)]
pub struct VRamFontLoader<const SIZE: usize = 128> {
    pub vram_base_addr: usize,
    pub vram_cache: Box<uluru::LRUCache<VRamGraphEntry, SIZE>>,
}
impl<const SIZE: usize> VRamFontLoader<SIZE> {
    pub fn reset(&mut self, tile_base_addr: usize) {
        self.vram_base_addr = tile_base_addr;
        self.vram_cache.clear();
        unsafe {
            nitro::sys::MI_CpuFill8(self.vram_base_addr as *mut _, 0, SIZE as u32 * 0x40);
        }
    }

    pub fn fetch(&mut self, graph_index: u16) -> u16 {
        if let Some(cache) = self.vram_cache.find(|x| x.graph_id == graph_index) {
            cache.vram_id
        } else {
            let graph_data = global_data().font_loader.get_graph_font2(graph_index);
            let next_vram_id = if self.vram_cache.len() == SIZE {
                self.vram_cache.iter().last().unwrap().vram_id
            } else {
                self.vram_cache.len() as u16
            };
            let save_addr = self.vram_base_addr + next_vram_id as usize * 0x40;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    graph_data.as_ptr(),
                    save_addr as *mut u8,
                    graph_data.len(),
                );
            }
            println!(
                "New graph {} has beed copied to {:08X} with index {}",
                graph_index, save_addr, next_vram_id
            );
            self.vram_cache.insert(VRamGraphEntry {
                graph_id: graph_index,
                vram_id: next_vram_id,
            });
            next_vram_id
        }
    }
}

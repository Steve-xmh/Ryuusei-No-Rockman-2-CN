use core::fmt::Debug;

use alloc::boxed::Box;
use nitro::fs::File;

#[derive(Debug)]
pub struct FontLoader {
    pub font1_file: File,
    pub font2_file: File,
    pub font3_file: File,
    pub font3_width_file: File,
    pub font1_cache: Box<uluru::LRUCache<GraphCache<0x40>, 256>>,
    pub font2_cache: Box<uluru::LRUCache<GraphCache<0x40>, 256>>,
    pub font3_cache: Box<uluru::LRUCache<GraphCache<0x80>, 256>>,
    pub font3_width_cache: Box<uluru::LRUCache<GraphCache<1>, 256>>,
}

trait LRUCacheExt<T> {
    fn get_or_insert(&mut self, pred: impl FnMut(&T) -> bool, f: impl FnOnce() -> T) -> &T;
}

impl<T, const N: usize> LRUCacheExt<T> for uluru::LRUCache<T, N> {
    #[inline(always)]
    fn get_or_insert(&mut self, pred: impl FnMut(&T) -> bool, f: impl FnOnce() -> T) -> &T {
        if self.find(pred).is_none() {
            let data = f();
            self.insert(data);
        }
        self.front().expect("LRUCache is empty")
    }
}

pub struct GraphCache<const SIZE: usize> {
    graph_id: u16,
    graph_data: [u8; SIZE],
}

impl<const SIZE: usize> Debug for GraphCache<SIZE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GraphCache")
            .field("graph_id", &self.graph_id)
            .field("graph_data", &self.graph_data.len())
            .finish()
    }
}

impl Default for FontLoader {
    fn default() -> Self {
        Self {
            font1_file: File::open("fonts/font1.bin\0"),
            font2_file: File::open("fonts/font2.bin\0"),
            font3_file: File::open("fonts/font3.bin\0"),
            font3_width_file: File::open("fonts/font3_width.bin\0"),
            font1_cache: Default::default(),
            font2_cache: Default::default(),
            font3_cache: Default::default(),
            font3_width_cache: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FontId {
    Font1,
    Font2,
    Font3,
}

impl FontLoader {
    #[inline(always)]
    pub fn get_graph(&mut self, graph_id: u16, font_id: FontId) -> &[u8] {
        match font_id {
            FontId::Font1 => self.get_graph_font1(graph_id),
            FontId::Font2 => self.get_graph_font2(graph_id),
            FontId::Font3 => self.get_graph_font3(graph_id),
        }
    }

    pub fn get_graph_font1(&mut self, graph_id: u16) -> &[u8] {
        &self
            .font1_cache
            .get_or_insert(
                |x| x.graph_id == graph_id,
                || {
                    let mut graph_data = [0; 0x40];
                    self.font1_file.seek(
                        graph_id as isize * 0x40,
                        nitro::fs::FSSeekFileMode::FS_SEEK_SET,
                    );
                    self.font1_file.read(&mut graph_data);
                    GraphCache {
                        graph_id,
                        graph_data,
                    }
                },
            )
            .graph_data
    }

    pub fn get_graph_font2(&mut self, graph_id: u16) -> &[u8] {
        &self
            .font2_cache
            .get_or_insert(
                |x| x.graph_id == graph_id,
                || {
                    let mut graph_data = [0; 0x40];
                    self.font2_file.seek(
                        graph_id as isize * 0x40,
                        nitro::fs::FSSeekFileMode::FS_SEEK_SET,
                    );
                    self.font2_file.read(&mut graph_data);
                    GraphCache {
                        graph_id,
                        graph_data,
                    }
                },
            )
            .graph_data
    }

    pub fn get_graph_font3(&mut self, graph_id: u16) -> &[u8] {
        &self
            .font3_cache
            .get_or_insert(
                |x| x.graph_id == graph_id,
                || {
                    let mut graph_data = [0; 0x80];
                    self.font3_file.seek(
                        graph_id as isize * 0x80,
                        nitro::fs::FSSeekFileMode::FS_SEEK_SET,
                    );
                    self.font3_file.read(&mut graph_data);
                    GraphCache {
                        graph_id,
                        graph_data,
                    }
                },
            )
            .graph_data
    }

    pub fn get_graph_font3_width(&mut self, graph_id: u16) -> u8 {
        self.font3_width_cache
            .get_or_insert(
                |x| x.graph_id == graph_id,
                || {
                    let mut graph_data = [0; 1];
                    self.font3_width_file
                        .seek(graph_id as isize, nitro::fs::FSSeekFileMode::FS_SEEK_SET);
                    self.font3_width_file.read(&mut graph_data);
                    GraphCache {
                        graph_id,
                        graph_data,
                    }
                },
            )
            .graph_data[0]
    }
}

const FONT1_POS: usize = 0x020B7898;
const FONT2_POS: usize = 0x020C0098;
const FONT3_POS: usize = 0x020C8898;
const FONT1_END_POS: usize = FONT1_POS + 0x40 * 0x1E3;
const FONT2_END_POS: usize = FONT2_POS + 0x40 * 0x1E3;
const FONT3_END_POS: usize = FONT3_POS + 0x80 * 0x1E3;
const FONT1_END_POS_E: usize = FONT1_END_POS - 1;
const FONT2_END_POS_E: usize = FONT2_END_POS - 1;
const FONT3_END_POS_E: usize = FONT3_END_POS - 1;

pub fn get_font_graph_id_by_addr(addr: usize) -> Option<(FontId, usize)> {
    match addr {
        FONT1_POS..=FONT1_END_POS_E => Some((FontId::Font1, (addr - FONT1_POS) / 0x40)),
        FONT2_POS..=FONT2_END_POS_E => Some((FontId::Font2, (addr - FONT2_POS) / 0x40)),
        FONT3_POS..=FONT3_END_POS_E => Some((FontId::Font3, (addr - FONT3_POS) / 0x80)),
        _ => None,
    }
}

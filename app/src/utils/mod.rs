use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use std::num::NonZeroU64;
use std::ops::{Deref, DerefMut};

pub mod shader_compiler;


pub fn green_blink() {
    const ESC: &str = "\x1B[";
    const RESET: &str = "\x1B[0m";
    eprint!("\r{}42m{}K{}\r", ESC, ESC, RESET);
    std::io::stdout().flush().unwrap();
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(50));
        eprint!("\r{}40m{}K{}\r", ESC, ESC, RESET);
        std::io::stdout().flush().unwrap();
    });
}

pub trait NonZeroSized: Sized {
    const SIZE: NonZeroU64 = unsafe { NonZeroU64::new_unchecked(std::mem::size_of::<Self>() as _) };
}

//将常量赋予满足Sized trait的类型
impl<T> NonZeroSized for T where T: Sized {}


#[derive(Debug)]
pub struct ContinuousHashMap<K, V>(HashMap<K, Vec<V>>);

impl<K, V> Deref for ContinuousHashMap<K, V> {
    type Target = HashMap<K, Vec<V>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for ContinuousHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> Default for ContinuousHashMap<K, V> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<K, V> ContinuousHashMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K: Eq + Hash, V> ContinuousHashMap<K, V> {
    pub fn push_value(&mut self, key: K, value: V) {
        //如果不存在对应key的vec数组，则新建
        self.0.entry(key).or_insert_with(Vec::new).push(value);
    }
}

pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
    pub unpadded_bytes_per_row: u32,
    pub padded_bytes_per_row: u32
}

impl ImageDimensions {
    pub fn new(width: u32, height: u32, align: u32) -> Self {
        let height = height.saturating_sub(height % 2);
        let width = width.saturating_sub(width % 2);
        let bytes_per_pixel = std::mem::size_of::<[u8; 4]>() as u32;
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }

    pub fn linear_size(&self) -> u64 {
        self.padded_bytes_per_row as u64 * self.height as u64
    }
}
// parser.rs - Parsing utilities

pub fn to_upper(s: &str) -> [u8; 128] {
    let mut result = [0u8; 128];
    let bytes = s.as_bytes();
    let len = bytes.len().min(128);
    
    for i in 0..len {
        result[i] = if bytes[i] >= b'a' && bytes[i] <= b'z' {
            bytes[i] - 32
        } else {
            bytes[i]
        };
    }
    result
}

pub fn var_index(name: &str) -> Option<usize> {
    let name = name.trim();
    if name.len() == 1 {
        let ch = name.chars().next()?;
        let upper_ch = if ch >= 'a' && ch <= 'z' {
            ((ch as u8) - b'a' + b'A') as char
        } else {
            ch
        };
        
        if upper_ch >= 'A' && upper_ch <= 'Z' {
            return Some((upper_ch as usize) - ('A' as usize));
        }
    }
    None
}

pub fn array_index(name: &str) -> Option<usize> {
    let name = name.trim();
    if name.len() == 1 {
        let ch = name.chars().next()?;
        let upper_ch = if ch >= 'a' && ch <= 'z' {
            ((ch as u8) - b'a' + b'A') as char
        } else {
            ch
        };
        
        if upper_ch >= 'A' && upper_ch <= 'J' {
            return Some((upper_ch as usize) - ('A' as usize));
        }
    }
    None
}

pub trait ByteArrayHelper {
    fn find_bytes(&self, needle: &[u8]) -> Option<usize>;
}

impl ByteArrayHelper for [u8; 128] {
    fn find_bytes(&self, needle: &[u8]) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }
        
        for i in 0..=(128 - needle.len()) {
            let mut found = true;
            for j in 0..needle.len() {
                if self[i + j] != needle[j] {
                    found = false;
                    break;
                }
            }
            if found {
                return Some(i);
            }
        }
        None
    }
}

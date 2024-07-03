use core::slice;
use std::{
    fs::{File, Metadata},
    os::{fd::AsRawFd, raw::c_void},
    ptr,
};

use libc::munmap;

pub struct Map<'a> {
    #[allow(dead_code)]
    file: File,

    file_metadata: Metadata,
    mapped_bytes: &'a mut [u8],
    byte_location: usize,
}

impl<'a> Map<'a> {
    pub fn new(file: File) -> Option<Self> {
        let file_data = file.metadata();
        let file_data: Metadata = match file_data {
            Ok(d) => d,
            Err(_) => return None,
        };
        let file_size: usize = file_data.len() as usize;

        let map: &mut [u8] = unsafe {
            let map_address = libc::mmap(
                ptr::null_mut(),
                file_size,
                libc::PROT_READ,
                libc::MAP_PRIVATE,
                file.as_raw_fd(),
                0,
            );

            if map_address == libc::MAP_FAILED {
                return None;
            }

            slice::from_raw_parts_mut(map_address as *mut u8, file_size)
        };

        Some(Self {
            file,
            file_metadata: file_data,
            mapped_bytes: map,
            byte_location: 0,
        })
    }
}

impl<'a> Drop for Map<'a> {
    fn drop(&mut self) {
        unsafe {
            munmap(
                self.mapped_bytes.as_ptr() as *mut c_void,
                self.mapped_bytes.len(),
            );
        };
    }
}

impl<'a> Iterator for Map<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let mut built_line: String = String::new();
        for location in self.byte_location.. {
            let current_char = self.mapped_bytes.get(location);
            match current_char {
                // unix line endings
                Some(b'\n') => {
                    self.byte_location = location + 1;
                    return Some(built_line);
                }
                // dos and legacy mac line endings
                Some(b'\r') => {
                    match self.mapped_bytes.get(location + 1) {
                        // dos CRLF line ending
                        Some(b'\n') => {
                            self.byte_location = location + 2;
                            return Some(built_line);
                        }
                        // mac legacy CR line ending (RARE)
                        Some(_) => {
                            self.byte_location = location + 1;
                            return Some(built_line);
                        }
                        // potential EOF
                        None => {
                            self.byte_location = location + 1;
                            return Some(built_line);
                        }
                    };
                }
                Some(letter) => {
                    built_line.push(*letter as char);
                    continue;
                }
                None => {
                    if location <= self.file_metadata.len() as usize {
                        self.byte_location = location + 1;
                        return Some(built_line);
                    } else {
                        return None;
                    }
                }
            };
        }
        None
    }
}

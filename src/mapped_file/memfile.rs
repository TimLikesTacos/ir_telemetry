use std::time::Duration;

use windows::Win32::Foundation::{CloseHandle, HANDLE};

use windows::Win32::System::Memory::{UnmapViewOfFile, MEMORY_MAPPED_VIEW_ADDRESS};
use windows::Win32::System::{
    Memory::{self, MapViewOfFile, OpenFileMappingA},
    Threading::{OpenEventA, WaitForSingleObject, SYNCHRONIZATION_SYNCHRONIZE},
};
use windows_core::{s, PCSTR};

use crate::error::{IrError, Result};
use crate::ir_data_inner::IrDataInner;
use crate::mapped_file::header::Header;
use crate::utils::byte_array_to_rust_string;

use super::var_header::VarHeaderData;

const MEM_MAP_FILENAME: PCSTR = s!("Local\\IRSDKMemMapFileName");
const DATA_EVENT_NAME: PCSTR = s!("Local\\IRSDKDataValidEvent");

#[derive(Debug)]
pub(crate) struct FileMap {
    mapping: HANDLE,
    shared_mem: MEMORY_MAPPED_VIEW_ADDRESS,
    data_event: HANDLE,
}

impl Drop for FileMap {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.data_event);
            let _ = UnmapViewOfFile(self.shared_mem);
            let _ = CloseHandle(self.mapping);
            log::debug!("Dropped FileMap");
        }
    }
}

// Implementation of FileMap
impl FileMap {
    pub(crate) fn new() -> Result<Self> {
        //Opens file mapping object
        // IRSDK: hMemMapFile = OpenFileMapping( FILE_MAP_READ, FALSE, IRSDK_MEMMAPFILENAME);
        // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-openfilemappinga#remarks
        let access = Memory::FILE_MAP_READ;
        let mapping = unsafe {
            OpenFileMappingA(access.0, false, MEM_MAP_FILENAME)
                .map_err(|e| IrError::MemMappingError(e.to_string()))?
        };

        // Create File View
        // IRSDK: pSharedMem = (const char *)MapViewOfFile(hMemMapFile, FILE_MAP_READ, 0, 0, 0);
        let shared_mem = unsafe { MapViewOfFile(mapping, access, 0, 0, 0) };

        #[cfg(feature = "create_datadump")]
        {
            use std::fs::File;
            use std::io::Write;
            log::info!("Creating dumpfile");

            let buf_count = 3usize;
            let max_vars = 4096usize;
            let buf_length = max_vars * 6usize;
            let session_str_len: usize = 131072; //128k

            let shared_mem_size = std::mem::size_of::<Header>()
                + session_str_len
                + (max_vars * std::mem::size_of::<VarHeaderData>())
                + (buf_count * buf_length);

            let mut file = File::create("iracing_dumpfile.dat").unwrap();
            let data: &[u8] = unsafe {
                std::slice::from_raw_parts(shared_mem.Value as *const u8, shared_mem_size)
            };

            // This is not normally configured, so unwraping and panicking is fine. We want to know something is wrong.
            file.write(data).unwrap();
            file.flush().unwrap();
            unsafe {
                let _ = UnmapViewOfFile(shared_mem);
                let _ = CloseHandle(mapping);
            }
            log::info!("Dumpfile created");
        }

        // IRSDK: hDataValidEvent = OpenEvent(SYNCHRONIZE, false, IRSDK_DATAVALIDEVENTNAME);
        let data_event = unsafe {
            OpenEventA(SYNCHRONIZATION_SYNCHRONIZE, false, DATA_EVENT_NAME).map_err(|e| {
                let _ = UnmapViewOfFile(shared_mem);
                let _ = CloseHandle(mapping);
                IrError::MemMappingError(e.to_string())
            })?
        };

        log::debug!("Connected to mapped file");
        Ok(Self {
            mapping,
            shared_mem,
            data_event,
        })
    }

    pub(crate) fn header(&self) -> Header {
        // Safety: The header is a valid pointer to a Header struct
        // Using read_volatile to ensure we get the latest data and optimazation does not remove the read.
        unsafe { std::ptr::read_volatile(self.shared_mem.Value as *const Header) }
    }

    pub(crate) fn is_connected(&self) -> bool {
        self.header().is_connected()
    }

    pub(crate) fn wait_for_update(&self, timeout: Duration) {
        // timout in milliseconds returns a u128, and windows call expects a u32.
        let milliseconds = u32::try_from(timeout.as_millis()).unwrap_or_else(|_| {
            log::warn!(
                "Timeout too large, using max value of {} seconds",
                Duration::from_millis(u32::MAX as u64).as_secs_f64()
            );
            u32::MAX
        });
        unsafe {
            WaitForSingleObject(self.data_event, milliseconds);
        }
    }

    /// Since the backing data is volatile, we will grab the header and copy from the buffer.
    /// This allows the caller to own the data at this point.
    pub(crate) fn get_new_data(&self) -> IrDataInner {
        let header = self.header();
        let buffer = header.most_recent_buffer();
        let buffer_len = header.buf_len as usize;
        let offset = buffer.offset as usize;
        let data_pointer = unsafe { self.shared_mem.Value.add(offset) };

        // Safety: Data is aligned, buffer is valid, and lifetime does not exceed the scope of this function. Data is copied into a new Vec.
        let data: &[u8] =
            unsafe { std::slice::from_raw_parts(data_pointer as *const u8, buffer_len) };
        // Copy the date and return an owned copy.  Gets us out of the unsafe code for the rest of the data's usage.
        IrDataInner::new(data.to_vec(), header, buffer.tick_count)
    }

    pub(crate) fn get_var_headers<'a, T>(&'a self) -> Vec<T>
    where
        T: From<&'a VarHeaderData>,
    {
        let header = self.header();
        let offset = header.var_header_offset as usize;
        let num_vars = header.num_vars as usize;

        let slice = unsafe {
            std::slice::from_raw_parts(
                self.shared_mem.Value.add(offset) as *const VarHeaderData,
                num_vars - 1,
            )
        };
        slice.iter().map(|vh| T::from(vh)).collect()
    }

    pub(crate) fn session_info(&self) -> String {
        let header = self.header();
        let offset = header.session_info_offset as usize;
        let len = header.session_info_len as usize;
        let slice = unsafe {
            std::slice::from_raw_parts(self.shared_mem.Value.add(offset) as *const u8, len)
        };
        byte_array_to_rust_string(slice)
    }
}

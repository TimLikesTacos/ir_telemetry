use bitflags::bitflags;
const MAX_BUFFERS: usize = 4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Buffer {
    pub(crate) tick_count: i32,
    pub(crate) offset: i32, // From the header
    pub(crate) pad: [i32; 2],
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct StatusField:i32 {
        const CONNECTED = 1;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct Header {
    ver: i32,                  // this api header version, see IRSDK_VER
    status: StatusField,       // bitfield using irsdk_StatusField
    pub(crate) tick_rate: i32, // ticks per second (60 or 360 etc)

    // session information, updated periodicaly
    pub(crate) session_info_update: i32, // Incremented when session info changes
    pub(crate) session_info_len: i32,    // Length in bytes of session info string
    pub(crate) session_info_offset: i32, // Session info, encoded in YAML format

    // State data, output at tickRate
    pub(crate) num_vars: i32, // length of array pointed to by varHeaderOffset
    pub(crate) var_header_offset: i32, // offset to irsdk_varHeader[numVars] array, Describes the variables received in varBuf
    pub(crate) num_buf: i32,           // <= IRSDK_MAX_BUFS (3 for now)
    pub(crate) buf_len: i32,           // length in bytes for one line
    pub(crate) pad1: [i32; 2],         // (16 byte align)
    pub(crate) var_buf: [Buffer; MAX_BUFFERS], // buffers of data being written to
}

impl Header {
    pub(crate) fn is_connected(&self) -> bool {
        self.status.contains(StatusField::CONNECTED)
    }

    pub(crate) fn most_recent_buffer(&self) -> &Buffer {
        assert!(!self.var_buf.is_empty()); // Something is wrong.
                                           // Unwrap is save as we have at least one buffer.
        self.var_buf.iter().max_by_key(|b| b.tick_count).unwrap()
    }

    pub(crate) fn session_info_update(&self) -> i32 {
        self.session_info_update
    }
}

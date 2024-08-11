use crate::mapped_file::header::Header;

#[derive(Debug, Clone)]
pub(crate) struct IrDataInner {
    header: Header,
    tick: i32,
    pub(crate) data: Vec<u8>,
}

impl IrDataInner {
    pub(crate) fn new(data: Vec<u8>, header: Header, tick: i32) -> Self {
        Self { header, tick, data }
    }

    pub(crate) fn header(&self) -> &Header {
        &self.header
    }

    pub(crate) fn tick(&self) -> i32 {
        self.tick
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn session_info_tick(&self) -> i32 {
        let header = self.header();
        header.session_info_update()
    }
}

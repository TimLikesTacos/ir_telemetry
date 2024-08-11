use crate::ir_data::IrData;
use crate::ir_data_inner::IrDataInner;
use crate::mapped_file::memfile::FileMap;
use crate::mapped_file::var_header::VarHeader;
use std::collections::HashMap;
use std::{
    sync::mpsc::{Receiver, SyncSender},
    time::Duration,
};

#[derive(Debug, Clone)]
/// An update packet will be sent over the channel.    
/// `UpdatePacket::Data` will be sent in intervals approximately equal to the request update frequency  
/// `UpdatePacket::SessionInfo` will be sent if the session data was updated. This is checked each data update interval.  
/// Session info is in the yaml formatted string found in the data. Since the data in session info changes, it is kept as a string to aid in custom
/// deserialization.  
/// `UpdatePacket::NotConnected` will be send when the session is not connected. This can be useful for reseting display data
/// between sessions.  
#[non_exhaustive]
pub enum UpdatePacket {
    Data(IrData),
    SessionInfo(String),
    VariableHeaders(HashMap<String, VarHeader>),
    NotConnected,
}

/// Used to conect to iRacing and get updates.
/// ```
/// use ir_telemetry::client::*;
/// // Update twice / second
/// let rx = Client::connect(2.);
///
///let mut var_headers = std::collections::HashMap::new();
/// # return; // exit out of doc test
///for packet in rx {
///    match packet {
///        UpdatePacket::Data(data) => {
///          println!("Speed: {}", data.get::<f32>(var_headers.get("Speed")).unwrap());
///        },
///        UpdatePacket::SessionInfo(session) => println!("Session: {}", session),
///        UpdatePacket::NotConnected => println!("Session is not open"),
///        UpdatePacket::VariableHeaders(v_header) => {
///            println!("# of variables in this session: {:?}", v_header.len());
///            var_headers = v_header;
///        }
///        _ => (),
///    }
///}
///```
pub struct Client {}

impl Client {
    /// Connect to iRacing and get updates. Utilizes a mpsc channel to send updates.  Sends an `UpdatePacket` when data interval is met.
    pub fn connect(update_rate: f32) -> Receiver<UpdatePacket> {
        let (tx, rx) = std::sync::mpsc::sync_channel(4);
        let mut initial = true;

        std::thread::spawn(move || loop {
            if !initial {
                log::warn!("Waiting for iRacing session to load");
                std::thread::sleep(Duration::from_secs(10));
            }
            initial = false;
            let mut client_inner = ClientInner::connect(update_rate, tx.clone());
            client_inner.start();
        });

        rx
    }
}
struct ClientInner {
    connection: FileMap,
    tick_count: i32,
    session_info_tick: i32,
    update_interval: std::time::Duration,
    last_update: std::time::Instant,
    tx: SyncSender<UpdatePacket>,
}

impl ClientInner {
    fn new(update_rate: f32, connection: FileMap, tx: SyncSender<UpdatePacket>) -> Self {
        let update_per_second = std::time::Duration::from_secs(100) / (update_rate * 100.) as u32;
        Self {
            connection,
            update_interval: update_per_second,
            last_update: std::time::Instant::now(),
            tick_count: i32::MIN,
            session_info_tick: i32::MIN,
            tx,
        }
    }

    fn connect(updates_per_second: f32, tx: SyncSender<UpdatePacket>) -> Self {
        let delay = std::time::Duration::from_secs(10);
        if let Ok(connection) = FileMap::new() {
            log::info!("Connected to iRacing shared memory map");
            Self::new(updates_per_second, connection, tx)
        } else {
            log::debug!("Waiting for iRacing to start");
            std::thread::sleep(delay);
            Self::connect(updates_per_second, tx)
        }
    }

    fn update(&mut self) {
        let new_data = self.connection.get_new_data();
        self.update_session_info(&new_data);
        self.update_data_packet(new_data);
    }

    /// Update the data packet if the data_tick (about 60 times a second) has changed
    fn update_data_packet(&mut self, new_data: IrDataInner) {
        self.last_update = std::time::Instant::now();
        if new_data.tick() != self.tick_count {
            self.tick_count = new_data.tick();
            self.tx
                .send(UpdatePacket::Data(IrData::new(new_data)))
                .expect("Channel closed");
        }
    }

    /// Update the session info packet if the session_info_tick (around 1 sec, but depends on what is happening in the seesion) has changed
    fn update_session_info(&mut self, data: &IrDataInner) {
        let session_tick = data.session_info_tick();
        if session_tick != self.session_info_tick {
            log::trace!("Updating session info, tick: {}", session_tick);
            let sess = self.connection.session_info();
            self.session_info_tick = session_tick;
            self.tx
                .send(UpdatePacket::SessionInfo(sess))
                .expect("Channel closed");
        }
    }

    fn update_variable_headers(&mut self) {
        let headers: Vec<VarHeader> = self.connection.get_var_headers();
        let mut map = HashMap::new();
        for header in headers {
            map.insert(header.name.clone(), header);
        }
        self.tx
            .send(UpdatePacket::VariableHeaders(map))
            .expect("Channel closed")
    }

    fn start(&mut self) {
        let mut new_session = true;
        loop {
            if !self.connection.is_connected() {
                self.tx
                    .send(UpdatePacket::NotConnected)
                    .expect("Channel closed");
                return;
            } else if self.last_update.elapsed() > self.update_interval {
                if new_session {
                    self.update_variable_headers();
                    new_session = false;
                }
                self.update();
            } else {
                self.connection.wait_for_update(
                    self.update_interval
                        .saturating_sub(self.last_update.elapsed()),
                );
            }
        }
    }
}

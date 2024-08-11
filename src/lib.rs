//! # iRacing Telemetry
//! Accesses the memory mapped file of iRacing to get telemetry data.
//!   
//! This crate handles two methods iRacing stores data.  
//! One is live telemetry updated about 60 times a second.  
//! The other is session data which is stored in a yaml formatted string in the memory mapped file. This is updated at varying intervals.  
//!
//! This crate abstracts the unsafe code and provides a safe interface with copied and owned data.  
//! Users of this crate recieve updates over a channel.  The updates are in the form of `UpdatePacket`.
//! The `UpdatePacket` enum has four variants:
//! - `Data(IrData)`: This is the telemetry data.  This is sent at the interval specified when connecting to iRacing.
//! - `SessionInfo(String)`: This is the session info.  This is sent when the session info is updated.  This is checked each data update interval.
//! This string is in yaml format.  Since the data in session info changes, it is kept as a string to aid in custom deserialization.
//! A struct `Session` is provided to deserialize this string.  This struct is not exhaustive and is recommended to use a custom deserialization method.
//! - `VariableHeaders(HashMap<String, VarHeader>)`: This is the variable headers.  The variables in the session are different car to car, but they remain
//! the same for the session.  Therefore, this packet is sent when each session is loaded.  This is useful for getting the data types and units of the variables.
//! - `NotConnected`: This is sent when the session is not connected.  This can be useful for reseting display data between sessions.
//! ```
//!
//! use std::collections::HashMap;
//! use ir_telemetry::{Client, UpdatePacket, IrData, VarHeader};
//! fn main() {
//!     println!("Starting iRacing Telemetry.  Ensure iRacing is running");
//!
//!     // Request updates two times a second. Can do up to 60 updates/ second
//!     let rx = Client::connect(2.);
//!
//!     let mut var_headers = std::collections::HashMap::new();
//!     # return; // exit out of doc test
//!     for packet in rx {
//!         match packet {
//!             UpdatePacket::Data(data) => handle_data_update(&data, &var_headers),
//!             UpdatePacket::SessionInfo(session) => println!("Obtained session data!"),
//!             UpdatePacket::NotConnected => println!("Session is not open"),
//!             UpdatePacket::VariableHeaders(v_header) => {
//!                 println!("# of variables in this session: {:?}", v_header.len());
//!                 var_headers = v_header;
//!             }
//!             _ => (),
//!         }
//!         
//!     }
//!}
//!
//! fn handle_data_update(data: &IrData, var_headers: &HashMap<String, VarHeader>) {
//!     let var_details = var_headers.get("Speed").unwrap();
//!     println!(
//!       "{}: {:.2} {} (or {} {} rounded) {}",
//!        var_details.name(),
//!        data.get::<f32>(Some(var_details)).unwrap(),
//!        var_details.units(),
//!        data.get::<i32>(Some(var_details)).unwrap(),
//!        var_details.units(),
//!        var_details.description()
//!     );
//! }
//! ```

mod error;
pub mod types;
pub mod mapped_file {
    pub mod full_value;
    pub(crate) mod header;
    pub(crate) mod memfile;
    pub(crate) mod values;
    pub mod var_header;
}

pub(crate) mod utils;

pub mod client;
pub mod ir_data;
pub(crate) mod ir_data_inner;
pub mod session_info {
    pub mod camera_info;
    pub mod driver_info;
    pub mod values {
        pub mod bool;
        pub mod measurement;
        pub mod percent;
    }
    pub mod qualify_results;
    pub mod radio_info;
    pub mod session;
    #[allow(clippy::module_inception)]
    pub mod session_info;
    pub mod split_time_info;
    pub mod weekend_info;
}

pub use client::{Client, UpdatePacket};
pub use ir_data::IrData;
pub use mapped_file::var_header::VarHeader;
pub use session_info::session::Session;
pub use types::*;

use client::UpdatePacket;
use ir_telemetry::*;
use mapped_file::var_header;
use session_info::session;

fn main() {
    pretty_env_logger::init();
    println!("Starting iRacing Telemetry.  Ensure iRacing is running");

    // Request updates two times a second. Can do up to 60 updates/ second
    let rx = client::Client::connect(2.);

    let mut var_headers = std::collections::HashMap::new();

    for packet in rx {
        match packet {
            UpdatePacket::Data(data) => handle_data_update(&data, &var_headers),
            // Not expected to happen often, but when session data is updated, this will be recieved
            UpdatePacket::SessionInfo(session) => handle_session_update(&session),
            UpdatePacket::NotConnected => println!("Session is not open"),
            UpdatePacket::VariableHeaders(v_header) => {
                println!("# of variables in this session: {:?}", v_header.len());
                var_headers = v_header;
            }
            _ => (),
        }
    }
}

fn handle_data_update(
    data: &IrData,
    var_headers: &std::collections::HashMap<String, var_header::VarHeader>,
) {
    let var_details = var_headers.get("Speed").unwrap();
    println!(
        "{}: {:.2} {} (or {} {} rounded) {}",
        var_details.name(),
        data.get::<f32>(Some(var_details)).unwrap(),
        var_details.units(),
        data.get::<i32>(Some(var_details)).unwrap(),
        var_details.units(),
        var_details.description()
    );

    let track_location: serde_json::Value = data
        .get_into(var_headers.get("CarIdxTrackSurface"))
        .unwrap();

    println!("Track locations: {}", track_location);
}

fn handle_session_update(session: &String) {
    let session: session::Session = serde_yaml::from_str(session).unwrap();
    let my_index = session.driver_info.driver_car_idx;
    let my_driver = &session.driver_info.drivers[my_index as usize];

    let wind_direction = &session.weekend_info.track_wind_dir;
    let wind_speed = &session.weekend_info.track_wind_vel;

    println!("My iRating: {}", my_driver.irating);
    println!("Wind speed: {}, direction {}", wind_speed, wind_direction);
}

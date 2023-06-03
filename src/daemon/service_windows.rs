// Ping service example.
//
// You can install and uninstall this service using other example programs.
// All commands mentioned below shall be executed in Command Prompt with Administrator privileges.
//
// Service installation: `install_service.exe`
// Service uninstallation: `uninstall_service.exe`
//
// Start the service: `net start ping_service`
// Stop the service: `net stop ping_service`
//
// Ping server sends a text message to local UDP port 1234 once a second.
// You can verify that service works by running netcat, i.e: `ncat -ul 1234`.

use crate::{config::SERVICE_NAME, logger::LoggerComponent};
use std::{
    ffi::OsString,
    fs,
    io::Write,
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::mpsc,
    time::Duration,
};
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher, Result,
};

use super::run_daemon;

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

#[cfg(windows)]
pub fn init_service() -> windows_service::Result<()> {
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.

    service_dispatcher::start(SERVICE_NAME, ffi_service_main)
}

// const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
// const RECEIVER_PORT: u16 = 1234;
// const PING_MESSAGE: &str = "ping\n";

// pub fn run() -> Result<()> {

// }

// Generate the windows service boilerplate.
// The boilerplate contains the low-level service entry function (ffi_service_main) that parses
// incoming service arguments into Vec<OsString> and passes them to user defined service
// entry (my_service_main).
define_windows_service!(ffi_service_main, my_service_main);

// Service entry function which is called on background thread by the system with service
// parameters. There is no stdout or stderr at this point so make sure to configure the log
// output to file if needed.
pub fn my_service_main(_arguments: Vec<OsString>) {
    if let Err(e) = run_service() {

        // Handle the error, by logging or something.
    }
}

pub fn run_service() -> Result<()> {
    LoggerComponent::Daemon.log(&format!("INIT"), Some(true));
    // Create a channel to be able to poll a stop event from the service worker loop.
    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let clone_sender = shutdown_tx.clone();
    // Define system service event handler that will be receiving service events.
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            // Notifies a service to report its current status information to the service
            // control manager. Always return NoError even if not implemented.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

            // Handle stop
            ServiceControl::Stop => {
                clone_sender.send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }

            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler.
    // The returned status handle should be used to report service status changes to the system.
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    // Tell the system that service is running
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    // Run the daemon
    //TODO add stop signal and cmd
    //test daemon stop timeout
    //TODO remove
    // std::thread::spawn(move || {
    //     std::thread::sleep(Duration::from_secs(5 * 60));
    //     shutdown_tx.send(()).unwrap();
    // });
    println!("running");
    run_daemon(shutdown_rx);
    // let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
    // let sender_addr = SocketAddr::new(loopback_ip, 0);
    // let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
    // let msg = PING_MESSAGE.as_bytes();
    // let socket = UdpSocket::bind(sender_addr).unwrap();

    // loop {
    //     let _ = socket.send_to(msg, receiver_addr);

    //     // Poll shutdown event.
    //     match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
    //         // Break the loop either upon stop or channel disconnect
    //         Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

    //         // Continue work if no events were received within the timeout
    //         Err(mpsc::RecvTimeoutError::Timeout) => (),
    //     };
    // }

    // Tell the system that service has stopped.
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

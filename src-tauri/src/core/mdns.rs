use futures_util::{pin_mut, stream::StreamExt};
use mdns::RecordKind;
use sacn::packet::ACN_SDT_MULTICAST_PORT;
use sacn::source::SacnSource;
use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tauri::{async_runtime::Mutex, Window};

const SERVICE_NAME: &'static str = "_e131._udp.local";

pub type ServiceMap = HashMap<String, Service>;

#[derive(Serialize, Clone)]
pub struct ServiceConfig {
    pub size: u16,
    pub universe: u16,
}

static SERVICE_CONFIG: phf::Map<&'static str, ServiceConfig> = phf::phf_map! {
  "bifrost-demo._e131._udp.local" => ServiceConfig { size: 50, universe: 1 },
  "dmx-lights._e131._udp.local" => ServiceConfig { size: 12, universe: 1 },
};

#[derive(Serialize, Clone)]
pub struct Service {
    pub name: String,
    pub ip: IpAddr,
    pub port: u16,
    pub config: Option<ServiceConfig>,
}

impl Service {
    pub fn get_sacn_source(&self) -> Option<SacnSource> {
        if let Some(ref config) = self.config {
            let local_addr: SocketAddr = SocketAddr::new(
                IpAddr::V4("0.0.0.0".parse().unwrap()),
                ACN_SDT_MULTICAST_PORT + 1,
            );
            // let addr: SocketAddr = SocketAddr::new(self.ip, self.port);
            let mut src = SacnSource::with_ip("Bifrost", local_addr).unwrap();
            src.register_universe(config.universe).unwrap(); // Register with the source that will be sending on the given universe.

            return Some(src);
        }
        None
    }
}

#[tauri::command]
pub async fn discover(
    window: Window,
    services: tauri::State<'_, Arc<Mutex<ServiceMap>>>,
) -> Result<(), ()> {
    println!("starting mdns discovery");
    // Iterate through responses from each e131 device, asking for new devices every 15s
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))
        .unwrap()
        .listen();

    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        let ptr: Vec<&String> = response
            .records()
            .filter_map(|record| match &record.kind {
                RecordKind::PTR(ptr) => Some(ptr),
                _ => None,
            })
            .collect();

        println!("Found PTR records: {:?}", ptr);

        for ptr in ptr {
            if let Some((port, target)) = response
                .records()
                .filter_map(|record| {
                    if &record.name == ptr {
                        match record.kind {
                            RecordKind::SRV {
                                port, ref target, ..
                            } => Some((port, target)),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .next()
            {
                if let Some::<IpAddr>(ip) = response
                    .records()
                    .filter_map(|record| {
                        if &record.name == target {
                            match record.kind {
                                RecordKind::A(ip) => Some(ip.into()),
                                RecordKind::AAAA(ip) => Some(ip.into()),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    println!("found e131 device: {} {} at {}:{}", ptr, target, ip, port);
                    let mut services = services.lock().await;
                    services.insert(
                        target.to_string(),
                        Service {
                            name: ptr.to_string(),
                            ip: ip,
                            port: port,
                            config: SERVICE_CONFIG.get(ptr.as_str()).map(|c| c.clone()),
                        },
                    );
                }
            }
        }
        {
            let services = services.lock().await;

            window
                .emit("services", serde_json::to_value(services.clone()).unwrap())
                .unwrap();
        }
    }

    Ok(())
}

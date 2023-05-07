use mdns::RecordKind;
use sacn::packet::ACN_SDT_MULTICAST_PORT;
use sacn::source::SacnSource;
use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use std::net::SocketAddr;

pub type ServiceMap = HashMap<String, Service>;

#[derive(Serialize, Clone, Debug)]
pub struct ServiceConfig {
    pub size: u16,
    pub universe: u16,
}

static SERVICE_CONFIG: phf::Map<&'static str, ServiceConfig> = phf::phf_map! {
  "bifrost-demo._e131._udp.local" => ServiceConfig { size: 50, universe: 1 },
  "dmx-lights._e131._udp.local" => ServiceConfig { size: 12, universe: 1 },
};

#[derive(Serialize, Clone, Debug)]
pub struct Service {
    pub name: String,
    pub addr: Option<SocketAddr>,
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

fn get_ptr(response: &mdns::Response) -> Vec<&String> {
    response
        .records()
        .filter_map(|record| match &record.kind {
            RecordKind::PTR(ptr) => Some(ptr),
            _ => None,
        })
        .collect()
}
pub fn get_services(response: &mdns::Response) -> ServiceMap {
    response
        .records()
        .filter_map(|record| match &record.kind {
            RecordKind::PTR(name) => {
                let ptr = name.to_string();
                let port_target = response
                    .records()
                    .filter_map(|record| match &record.kind {
                        RecordKind::SRV { port, target, .. } if &record.name == name => {
                            Some((port, target.to_string()))
                        }
                        _ => None,
                    })
                    .next();
                if let Some((port, target)) = port_target {
                    let ip = response
                        .records()
                        .filter_map(|record| match record.kind {
                            RecordKind::A(addr) if record.name == target => Some(addr.into()),
                            RecordKind::AAAA(addr) if record.name == target => Some(addr.into()),
                            _ => None,
                        })
                        .next();
                    if let Some(ip) = ip {
                        let config = SERVICE_CONFIG.get(&ptr).cloned();
                        Some((
                            target,
                            Service {
                                name: ptr,
                                addr: Some(SocketAddr::new(ip, *port)),
                                config,
                            },
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

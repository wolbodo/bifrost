use tauri::{async_runtime::Mutex, Window,};

use serde::Serialize;
use std::sync::Arc;
use std::{time::Duration};
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{RecordKind};
use std::net::{IpAddr};
use std::collections::HashMap;

const SERVICE_NAME: &'static str = "_e131._udp.local";

pub type ServiceMap = HashMap<String, Service>;

#[derive(Serialize, Clone)]
pub struct Service {
  name: String,
  ip: IpAddr,
  port: u16,
}

#[tauri::command]
pub async fn discover(window: Window, services: tauri::State<'_, Arc<Mutex<ServiceMap>>>) -> Result<(), ()> {

  println!("starting mdns discovery");
  // Iterate through responses from each e131 device, asking for new devices every 15s
  let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15)).unwrap().listen();

  pin_mut!(stream);

  while let Some(Ok(response)) = stream.next().await {

    let ptr: Vec<&String> = response.records()
      .filter_map(|record| match &record.kind {
          RecordKind::PTR(ptr) => Some(ptr),
          _ => None,
      })
      .collect();

    println!("Found PTR records: {:?}", ptr);

    for ptr in ptr {
      if let Some((port, target)) = response.records()
        .filter_map(|record| if &record.name == ptr {
          match record.kind {
            RecordKind::SRV { port, ref target, .. } => Some((port, target)),
            _ => None,
          }
        } else {
          None
        })
        .next() {

          if let Some::<IpAddr>(ip) = response.records()
            .filter_map(|record| if &record.name == target {
              match record.kind {
                RecordKind::A(ip) => Some(ip.into()),
                RecordKind::AAAA(ip) => Some(ip.into()),
                _ => None,
              }
            } else {
              None
            })
            .next() {
              println!("found e131 device: {} {} at {}:{}", ptr, target, ip, port);
              let mut services = services.lock().await;

              services.insert(target.to_string(), Service {
                name: ptr.to_string(),
                ip: ip,
                port: port,
              });
            }
        }
    }
    {
      let services = services.lock().await;
      
      window.emit("services", serde_json::to_value(services.clone()).unwrap()).unwrap();
    }
  }

  Ok(())

}

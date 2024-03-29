/// Monitor hardware caontrol system.
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration,SystemTime};
use log::{info,warn};
use jsonrpc_core_client::local;
use jsonrpc_core::futures::future::{self, Future, FutureResult};
use jsonrpc_core::{Error, IoHandler, Result};
use jsonrpc_derive::rpc;
use lpcan::canbus::CanBus;
use std::sync::{Arc,RwLock, Mutex, RwLockReadGuard, RwLockWriteGuard};
use lazy_static::lazy_static;

use super::pump::GearPump;
use super::humidity::Humidity;
use super::pressure::Pressure;
use super::airflow::Airflow;
use super::relay::Relay;
use super::valve::Valve;
use super::sensor::Sensor;
use super::rpc::UvRpc;
use super::analog::AInput;
use super::analog::AOutput;
use super::config::UVConfig;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Lamp {
    pub uptime: SystemTime,
    pub lifetime: Duration,
    pub on: bool,
}


impl Default for Lamp {
    fn default() -> Self {
        Self {
            uptime:SystemTime::now(), 
            on: false,
            lifetime: Duration::from_secs(1),
        }
    }
}

impl Lamp {

    pub fn turn_on( &mut self) {
        self.on     = true;
        self.uptime = SystemTime::now();
    }
    pub fn turn_off(&mut self) {
        self.update_lifitime();
        self.on     = false;
    }
    pub fn update_lifitime(&mut self){
        let now     = SystemTime::now();
        match now.duration_since(self.uptime) {
            Ok(uptime) if self.on => {
                self.lifetime += uptime;
                self.uptime    = SystemTime::now();
            },
            Ok(_)  => info!("UV Lamp turn off"),
            Err(e) => warn!("UV Lamp uptime:{:}",e),
        }
    }
}


// async fn get_lamp() -> EndpointResult {
    // let id: usize = cx.param("id").client_err()?;

    // if let Some(msg) = cx.app_data().messages().get(id) {
        // Ok(response::json(msg))
    // } else {
        // Err(StatusCode::NOT_FOUND)?
    // }
// }
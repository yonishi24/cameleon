/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::sync::Mutex;

use crate::{
    imp::device::Device,
    imp::port::{Port, TlType},
    GenTlError, GenTlResult,
};

pub(crate) mod u3v;
pub(crate) mod gige;

mod u3v_genapi;
mod gige_genapi;

pub(crate) trait Interface: Port {
    fn open(&mut self) -> GenTlResult<()>;

    fn close(&mut self) -> GenTlResult<()>;

    fn update_device_list(&mut self, timeout: std::time::Duration) -> GenTlResult<bool>;

    fn interface_id(&self) -> &str;

    fn display_name(&self) -> &str;

    fn tl_type(&self) -> TlType;

    fn mac_addr(&self) -> Option<[u8; 6]>;

    fn ip_addr(&self) -> Option<std::net::Ipv4Addr>;

    fn subnet_mask(&self) -> Option<std::net::Ipv4Addr>;

    fn gateway_addr(&self) -> Option<std::net::Ipv4Addr>;

    fn devices(&self) -> Vec<&Mutex<dyn Device>>;

    fn device_by_id(&self, id: &str) -> GenTlResult<&Mutex<dyn Device>> {
        self.devices()
            .into_iter()
            .find(|dev| dev.lock().unwrap().device_id() == id)
            .ok_or_else(|| GenTlError::InvalidId(id.into()))
    }
}

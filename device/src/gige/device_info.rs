/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::fmt;

use semver::Version;

/// Device information in class-specific device descriptor.
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    /// GenCP version the device provides.
    pub gencp_version: Version,

    /// USB3-Vision version the device provides.
    pub gev_version: Version,

    /// Device GUID consists of 12 characters.
    /// First 4 characters are vendor ID and last 8 characters are unique id assigned by a vendor.
    pub guid: String,

    /// Manufacturer name of the device.
    pub vendor_name: String,

    /// Model name of the device.
    pub model_name: String,

    /// A human readable name referring to multiple models of a single manufacturer.
    pub family_name: Option<String>,

    /// Manufacturer specific device version.
    /// An application can't make any assumptions of this version.
    pub device_version: String,

    /// Manufacturer specific information.
    /// This field is optional.
    pub manufacturer_info: String,

    /// Serial number of the device.
    pub serial_number: String,

    /// User defined name.
    /// This field is optional.
    pub user_defined_name: Option<String>,

    /// Bus speed supported by the device.
    pub supported_speed: BusSpeed,
}

/// Bus speed supported by each USB device.
#[derive(Clone, Copy, Debug)]
pub enum BusSpeed {
    /// USB 1.0/Low-Speed: 1.5 Mbps
    LowSpeed,

    /// USB 1.1/Full-Speed: 12 Mbps
    FullSpeed,

    /// USB 2.0/Hi-Speed: 480 Mbps
    HighSpeed,

    /// USB 3.0/SuperSpeed: 5 Gbps
    SuperSpeed,

    /// USB 3.1/SuperSpeedPlus: 10 Gbps
    SuperSpeedPlus,
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "### Device Information ###")?;

        writeln!(f, "GenCP Version: {}", self.gencp_version)?;

        writeln!(f, "GEV Version: {}", self.gev_version)?;

        writeln!(f, "GUID: {}", self.guid)?;

        writeln!(f, "Vendor Name: {}", self.vendor_name)?;

        writeln!(f, "Model Name: {}", self.model_name)?;

        if let Some(family_name) = &self.family_name {
            writeln!(f, "Family Name: {}", family_name)
        } else {
            writeln!(f, "Family Name: N/A")
        }?;

        writeln!(f, "Manufacturer Information: {}", self.manufacturer_info)?;

        writeln!(f, "Serial Number: {}", self.serial_number)?;

        if let Some(user_defined_name) = &self.user_defined_name {
            writeln!(f, "User Defined Name: {}", user_defined_name)
        } else {
            writeln!(f, "User Defined Name: N/A")
        }?;

        write!(f, "Supported Speed: {:?}", self.supported_speed)?;

        Ok(())
    }
}

#![no_std]

use drogue_bluetooth_hci::{
    vendor,
};

use drogue_bluetooth_hci_core::commands::{
    SetAdvertisingParameters,
    SetAdvertisingData,
    SetScanResponseData,
    SetAdvertisingEnable
};

use drogue_bluetooth_hci_core::events::{
    CommandStatus,
    CommandComplete,
};
use drogue_bluetooth_hci::controller::Controller;
use drogue_bluetooth_hci::host::Host;
use drogue_bluetooth_hci::vendor::Vendor;

vendor!{
    [BlueNRG]
    commands => {
        SetAdvertisingParameters,
        SetAdvertisingData,
        SetScanResponseData,
        SetAdvertisingEnable,
    },
    events => {
        CommandStatus,
        CommandComplete,
    }
}



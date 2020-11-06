#![no_std]
#![no_main]

use rtic::app;
//use rtic::cyccnt::{U32Ext};
use rtt_logger::RTTLogger;
use rtt_target::rtt_init_print;
use rtt_target::rprintln;
use cortex_m;

use log::{info, LevelFilter};
use nb::{block, Error};

static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Debug);

use core::{
    sync::atomic::{compiler_fence, Ordering::SeqCst},
    panic::PanicInfo,
};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use cortex_m::interrupt;

    interrupt::disable();

    rprintln!("panic");
    rprintln!("{}", info);

    loop {
        compiler_fence(SeqCst);
    }
}

use stm32l4xx_hal::{
    //pac,
    //stm32l4
    hal::{
        spi,
    },
    spi::{
        Spi
    },
};

use core::fmt::Write;

//use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use stm32l4xx_hal::time::{U32Ext as HalU32Ext, Hertz, KiloHertz};
use stm32l4xx_hal::rcc::RccExt;
use stm32l4xx_hal::flash::FlashExt;
use stm32l4xx_hal::pwr::PwrExt;
//use stm32l4xx_hal::hal::blocking::spi::{Write, Transfer};
use stm32l4xx_hal::hal::digital::v2::{
    OutputPin,
    InputPin,
};
use stm32l4xx_hal::delay::Delay;
use stm32l4xx_hal::hal::blocking::delay::{DelayUs, DelayMs};
use heapless::{consts::*, spsc::Queue, i::Queue as ConstQueue, String};

use rtic::cyccnt::U32Ext as cyccntU32Ext;
use core::convert::TryInto;
//use drogue_es_wifi::network::EsWifiNetworkDriver;

use drogue_bluetooth_hci::host::Host;
use drogue_bluetooth_bluenrg::BlueNRG;

use core::str::{FromStr};
use drogue_bluetooth_ibeacon::IBeacon;

#[app(device = stm32l4xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init]
    fn init(mut ctx: init::Context) {
        rtt_init_print!( BlockIfFull, 2048);
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Info);
        let mut device: stm32l4xx_hal::pac::Peripherals = ctx.device;

        ctx.core.DCB.enable_trace();
        //// required on Cortex-M7 devices that software lock the DWT (e.g. STM32F7)
        //DWT::unlock();
        ctx.core.DWT.enable_cycle_counter();
        //let cp: cortex_m::Peripherals = ctx.core;

        //cmp.DWT.enable_cycle_counter();
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let mut pwr = device.PWR.constrain(&mut rcc.apb1r1);
        let clocks = rcc
            .cfgr
            .sysclk(80.mhz())
            .pclk1(80.mhz())
            .pclk2(80.mhz())
            .freeze(&mut flash.acr, &mut pwr);

        let mut host: Host<BlueNRG> = Host::new();
        // supports the requirements for IBeacon, so you can .start() it
        host.start();

    }

    // spare interrupt used for scheduling software tasks
    extern "C" {
        fn SPI1();
        fn SPI2();
    }
};




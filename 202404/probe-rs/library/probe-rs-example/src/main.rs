use std::time::Duration;

use probe_rs::flashing::{download_file, Format};
use probe_rs::{DebugProbeInfo, DebugProbeType, Lister, Permissions};

fn main() -> anyhow::Result<()> {
    let lister = Lister::new();
    let probes = lister.list_all();
    let my_dk_info = DebugProbeInfo::new(
        "J-Link (J-Link)",
        0x1366,
        0x1051,
        Some("001050220333".to_string()),
        DebugProbeType::JLink,
        None,
    );

    if let Some(my_dk) = probes.iter().find(|probe| probe == &&my_dk_info) {
        println!("Found my DK: {:#?}", my_dk);
        let probe: probe_rs::Probe = my_dk.open(&lister)?;
        let mut session = probe.attach("nRF52840_xxAA", Permissions::default())?;
        println!("now flashing...");
        download_file(
            &mut session,
            "/home/tomoyuki/repos/nrf52840-dk/target/thumbv7em-none-eabihf/debug/examples/blinky",
            Format::Elf,
        )?;

        println!("now resetting...");
        let mut core = session.core(0)?;
        core.halt(Duration::from_millis(100))?;
        core.reset()?;
        core.run()?;
        println!("done!");
    }

    Ok(())
}

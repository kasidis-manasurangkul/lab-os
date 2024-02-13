fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    println!("UEFI Path: {}", uefi_path);
    println!("BIOS Path: {}", bios_path);

    // choose whether to start the UEFI or BIOS image
    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");

    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive").arg(format!("format=raw,file={}", uefi_path));
        cmd.arg("-serial").arg("stdio");
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={}", bios_path));
        cmd.arg("-serial").arg("stdio");
    }

    println!("Running command: {:?}", cmd);

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to start QEMU: {}", e);
            return;
        }
    };

    if let Err(e) = child.wait() {
        eprintln!("QEMU execution failed: {}", e);
    }
}

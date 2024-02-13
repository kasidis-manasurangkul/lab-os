// #![no_std] // don't link the Rust standard library
// #![no_main] // disable all Rust-level entry points

// use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
// use core::fmt::Write;
// use bootloader_api::info::{FrameBuffer, PixelFormat};
// use kernel::serial;
// use noto_sans_mono_bitmap::{FontWeight, get_raster, RasterHeight, RasterizedChar};



// #[panic_handler]
// fn panic(info: &core::panic::PanicInfo) -> ! {
//     let _ = writeln!(serial(), "PANIC: {info}");
//     loop {}
// }
// const BOOTLOADER_CONFIG: BootloaderConfig = {
//     let mut config = BootloaderConfig::new_default();
//     config.kernel_stack_size = 100 * 1024; // 100 KiB kernel stack size
//     config
// };
// entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

// fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
//     writeln!(serial(), "Entered kernel with boot info: {boot_info:?}").unwrap();

//     let vga_buffer = boot_info.framebuffer.as_mut().unwrap();
//     // vga_buffer.buffer_mut().fill(0);

//     let bitmap_char = get_raster('O', FontWeight::Regular, RasterHeight::Size16).unwrap();
//     let next = write_rendered_char(vga_buffer, 0, 512, bitmap_char);
//     write_rendered_char(vga_buffer, next.0, next.1, get_raster('S', FontWeight::Regular, RasterHeight::Size16).unwrap());

//     writeln!(serial(), "Entering kernel wait loop...").unwrap();

//     loop {}
// }

// fn write_rendered_char(buffer:&mut FrameBuffer, x_pos:usize, y_pos:usize, rendered_char: RasterizedChar) -> (usize, usize) {
//     for (y, row) in rendered_char.raster().iter().enumerate() {
//         for (x, byte) in row.iter().enumerate() {
//             write_pixel(buffer, x_pos + x, y_pos + y, *byte);
//         }
//     }
//     (x_pos + rendered_char.width(), y_pos)
// }

// fn write_pixel(buffer:&mut FrameBuffer, x: usize, y: usize, intensity: u8) {
//     let mut info = buffer.info();
//     let pixel_offset = y * usize::from(info.stride) + x;
//     let color = match info.pixel_format {
//         PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
//         PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
//         other => {
//             info.pixel_format = PixelFormat::Rgb;
//             panic!("pixel format {:?} not supported in logger", other)
//         }
//     };
//     let bytes_per_pixel = info.bytes_per_pixel;
//     let byte_offset = pixel_offset * usize::from(bytes_per_pixel);

//     buffer.buffer_mut()[byte_offset..(byte_offset + usize::from(bytes_per_pixel))]
//         .copy_from_slice(&color[..usize::from(bytes_per_pixel)]);
// }


#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use core::fmt::Write;
use bootloader_api::info::{FrameBuffer, PixelFormat};
use kernel::serial;
use noto_sans_mono_bitmap::{FontWeight, get_raster, RasterHeight, RasterizedChar};


// Add this enum for directional input
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(serial(), "PANIC: {info}");
    loop {}
}

const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB kernel stack size
    config
};
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    writeln!(serial(), "Entered kernel with boot info: {boot_info:?}").unwrap();

    let vga_buffer = boot_info.framebuffer.as_mut().unwrap();
    vga_buffer.buffer_mut().fill(0);

    let hello_world = "Hello, World!";
    let mut x_pos = 0;
    let mut y_pos = 512;

    loop {
        // Clear the framebuffer
        vga_buffer.buffer_mut().fill(0);

        // Render "Hello, World!" at the current position
        let mut current_x = x_pos;
        for c in hello_world.chars() {
            let bitmap_char = get_raster(c, FontWeight::Regular, RasterHeight::Size16).unwrap();
            let next = write_rendered_char(vga_buffer, current_x, y_pos, bitmap_char);
            current_x = next.0 + 1;  // Adding space between characters
        }

        // Handle input (this function needs to be implemented)
        match get_input() {
            Some(Direction::Up) => if y_pos > 0 { y_pos -= 10; },
            Some(Direction::Down) => y_pos += 10,
            Some(Direction::Left) => if x_pos > 0 { x_pos -= 10; },
            Some(Direction::Right) => x_pos += 10,
            None => (),
        }
    }
}

fn write_rendered_char(buffer: &mut FrameBuffer, x_pos: usize, y_pos: usize, rendered_char: RasterizedChar) -> (usize, usize) {
    for (y, row) in rendered_char.raster().iter().enumerate() {
        for (x, byte) in row.iter().enumerate() {
            write_pixel(buffer, x_pos + x, y_pos + y, *byte);
        }
    }
    (x_pos + rendered_char.width(), y_pos)
}

fn write_pixel(buffer: &mut FrameBuffer, x: usize, y: usize, intensity: u8) {
    let mut info = buffer.info();
    let pixel_offset = y * usize::from(info.stride) + x;
    let color = match info.pixel_format {
        PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
        PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
        other => {
            info.pixel_format = PixelFormat::Rgb;
            panic!("pixel format {:?} not supported in logger", other)
        }
    };
    let bytes_per_pixel = info.bytes_per_pixel;
    let byte_offset = pixel_offset * usize::from(bytes_per_pixel);

    buffer.buffer_mut()[byte_offset..(byte_offset + usize::from(bytes_per_pixel))]
        .copy_from_slice(&color[..usize::from(bytes_per_pixel)]);
}

use x86_64::instructions::port::Port;

fn get_input() -> Option<Direction> {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    match scancode {
        0x48 => Some(Direction::Up),    // Arrow Up
        0x50 => Some(Direction::Down),  // Arrow Down
        0x4B => Some(Direction::Left),  // Arrow Left
        0x4D => Some(Direction::Right), // Arrow Right
        _ => None,                      // This handles all other cases
    }
}
fn no_op() {}

#[no_mangle]
#[used]
pub static mut ABORT_HANDLER: fn() = no_op;

// Raise the whole `.data` segment alignment without adding useful live data.
#[repr(align(4096))]
#[allow(dead_code)]
struct SegmentAlign([u8; 1]);

#[link_section = ".data"]
#[used]
static mut SEGMENT_ALIGN: SegmentAlign = SegmentAlign([0x33]);

#[used]
static mut MAIN_DATA: u8 = 0x11;
#[used]
static mut SPLIT_A_DATA: u8 = 0x22;
#[used]
static mut SPLIT_B_DATA: u8 = 0x44;
#[used]
static mut SPLIT_C_DATA: u8 = 0x66;

#[no_mangle]
pub extern "C" fn read_main() -> u8 {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(MAIN_DATA)) }
}

#[no_mangle]
pub extern "C" fn __wasm_split_00split_a00_export_00000000000000000000000000000000() -> u8 {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(SPLIT_A_DATA)) }
}

#[no_mangle]
pub extern "C" fn __wasm_split_00split_b00_export_00000000000000000000000000000000() -> u8 {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(SPLIT_B_DATA)) }
}

#[no_mangle]
pub extern "C" fn __wasm_split_00split_c00_export_00000000000000000000000000000000() -> u8 {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(SPLIT_C_DATA)) }
}

#[link(wasm_import_module = "__wasm_split_placeholder__")]
extern "C" {
    fn __wasm_split_00split_a00_import_00000000000000000000000000000000() -> u8;
    fn __wasm_split_00split_b00_import_00000000000000000000000000000000() -> u8;
    fn __wasm_split_00split_c00_import_00000000000000000000000000000000() -> u8;
}

#[no_mangle]
pub extern "C" fn entry() -> u8 {
    unsafe {
        __wasm_split_00split_a00_import_00000000000000000000000000000000()
            ^ __wasm_split_00split_b00_import_00000000000000000000000000000000()
            ^ __wasm_split_00split_c00_import_00000000000000000000000000000000()
    }
}

#[link_section = "__wasm_split_unstable"]
#[used]
static WASM_SPLIT_MARKER: [u8; 3] = [1, 1, 1];

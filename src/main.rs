mod common;
mod device;
mod host;

fn main() {
    #[cfg(feature = "device")]
    device::run();
    #[cfg(feature = "host")]
    host::run();
}

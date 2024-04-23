use tracing::*;
use twba_common::init_tracing;

pub fn main() {
    let _guard = init_tracing("common");

    info!("Hello, world! info");
    error!("Hello, world! error");
    warn!("Hello, world! warn");
    debug!("Hello, world! debug");
    trace!("Hello, world! trace");
}

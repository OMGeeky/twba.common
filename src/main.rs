use twba_common::prelude::*;

pub fn main() {
    let _guard = init_tracing("twba_common");

    trace!("Hello, world! trace");
    debug!("Hello, world! debug");
    info!("Hello, world! info");
    warn!("Hello, world! warn");
    error!("Hello, world! error");
}

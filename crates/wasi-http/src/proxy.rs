use crate::{bindings, WasiHttpView};
use wasmtime_wasi::preview2;

wasmtime::component::bindgen!({
    world: "wasi:http/proxy",
    tracing: true,
    async: true,
    with: {
        "wasi:cli/stderr": preview2::bindings::cli::stderr,
        "wasi:cli/stdin": preview2::bindings::cli::stdin,
        "wasi:cli/stdout": preview2::bindings::cli::stdout,
        "wasi:clocks/monotonic-clock": preview2::bindings::clocks::monotonic_clock,
        "wasi:clocks/timezone": preview2::bindings::clocks::timezone,
        "wasi:clocks/wall-clock": preview2::bindings::clocks::wall_clock,
        "wasi:http/incoming-handler": bindings::http::incoming_handler,
        "wasi:http/outgoing-handler": bindings::http::outgoing_handler,
        "wasi:http/types": bindings::http::types,
        "wasi:io/streams": preview2::bindings::io::streams,
        "wasi:poll/poll": preview2::bindings::poll::poll,
        "wasi:random/random": preview2::bindings::random::random,
    },
});

pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()>
where
    T: WasiHttpView + bindings::http::types::Host,
{
    bindings::http::incoming_handler::add_to_linker(l, |t| t)?;
    bindings::http::outgoing_handler::add_to_linker(l, |t| t)?;
    bindings::http::types::add_to_linker(l, |t| t)?;
    Ok(())
}

#[cfg(feature = "sync")]
pub mod sync {
    use crate::{bindings, WasiHttpView};
    use wasmtime_wasi::preview2;

    wasmtime::component::bindgen!({
        world: "wasi:http/proxy",
        tracing: true,
        async: false,
        with: {
            "wasi:cli/stderr": preview2::bindings::cli::stderr,
            "wasi:cli/stdin": preview2::bindings::cli::stdin,
            "wasi:cli/stdout": preview2::bindings::cli::stdout,
            "wasi:clocks/monotonic-clock": preview2::bindings::clocks::monotonic_clock,
            "wasi:clocks/timezone": preview2::bindings::clocks::timezone,
            "wasi:clocks/wall-clock": preview2::bindings::clocks::wall_clock,
            "wasi:http/incoming-handler": bindings::sync::http::incoming_handler,
            "wasi:http/outgoing-handler": bindings::sync::http::outgoing_handler,
            "wasi:http/types": bindings::sync::http::types,
            "wasi:io/streams": preview2::bindings::sync_io::io::streams,
            "wasi:poll/poll": preview2::bindings::sync_io::poll::poll,
            "wasi:random/random": preview2::bindings::random::random,
        },
    });

    pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()>
    where
        T: WasiHttpView + bindings::sync::http::types::Host,
    {
        bindings::sync::http::incoming_handler::add_to_linker(l, |t| t)?;
        bindings::sync::http::outgoing_handler::add_to_linker(l, |t| t)?;
        bindings::sync::http::types::add_to_linker(l, |t| t)?;
        Ok(())
    }
}

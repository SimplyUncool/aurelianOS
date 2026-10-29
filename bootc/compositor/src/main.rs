// SPDX-License-Identifier: Apache-2.0

mod backend;
mod state;

use smithay::{
    backend::session::libseat::LibSeatSession,
    reexports::{
        calloop::{EventLoop, Interest, Mode, PostAction, generic::Generic},
        wayland_server::Display,
    },
    wayland::compositor::CompositorState,
};

use crate::{
    backend::udev::Backend,
    state::{State, add_wayland_socket},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let mut event_loop: EventLoop<'static, State> = EventLoop::try_new()?;
    let display = Display::<State>::new()?;
    let display_handle = display.handle();

    let (session, session_notifier) = LibSeatSession::new()?;
    let (backend, drm_notifier) = Backend::initialize(session, &display_handle)?;
    let compositor_state = CompositorState::new::<State>(&display_handle);
    let state = State {
        display_handle: display_handle.clone(),
        compositor_state,
        backend,
    };

    add_wayland_socket(&event_loop)?;
    event_loop.handle().insert_source(
        Generic::new(display, Interest::READ, Mode::Level),
        |_, display, state| {
            // Safety: the calloop source owns this Display for the lifetime of the event loop.
            unsafe {
                display.get_mut().dispatch_clients(state)?;
            }
            Ok(PostAction::Continue)
        },
    )?;
    event_loop
        .handle()
        .insert_source(drm_notifier, |event, _, state| {
            state.backend.on_drm_event(event);
        })?;
    event_loop
        .handle()
        .insert_source(session_notifier, |event, _, state| {
            state.backend.on_session_event(event);
        })?;

    let mut state = state;
    tracing::info!("aurelianOS compositor ready; dispatching Wayland clients");
    loop {
        event_loop.dispatch(None, &mut state)?;
        state.display_handle.flush_clients()?;
    }
}

// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use smithay::{
    reexports::{
        calloop::EventLoop,
        wayland_server::{DisplayHandle, backend::ClientData},
    },
    wayland::{
        compositor::{CompositorClientState, CompositorHandler, CompositorState},
        socket::ListeningSocketSource,
    },
};

use crate::backend::udev::Backend;

pub struct ClientState {
    compositor_state: CompositorClientState,
}

impl ClientData for ClientState {}

pub struct State {
    pub display_handle: DisplayHandle,
    pub compositor_state: CompositorState,
    pub backend: Backend,
}

impl CompositorHandler for State {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(
        &self,
        client: &'a smithay::reexports::wayland_server::Client,
    ) -> &'a CompositorClientState {
        &client
            .get_data::<ClientState>()
            .expect("missing client state")
            .compositor_state
    }

    fn commit(
        &mut self,
        _surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        tracing::debug!("Wayland surface committed");
    }
}

impl smithay::wayland::output::OutputHandler for State {}

smithay::delegate_compositor!(State);
smithay::delegate_output!(State);

pub fn add_wayland_socket(
    event_loop: &EventLoop<'static, State>,
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = ListeningSocketSource::new_auto()?;
    event_loop
        .handle()
        .insert_source(socket, |stream, _, state| {
            if let Err(error) = state.display_handle.insert_client(
                stream,
                Arc::new(ClientState {
                    compositor_state: CompositorClientState::default(),
                }),
            ) {
                tracing::error!(?error, "failed to add Wayland client");
            }
        })?;
    Ok(())
}

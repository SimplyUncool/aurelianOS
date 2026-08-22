use std::sync::Arc;

use smithay::{
    reexports::{
        calloop::EventLoop,
        wayland_server::{
            backend::ClientData,
            Display,
        },
    },
    wayland::{
        compositor::{
            CompositorClientState,
            CompositorHandler,
            CompositorState,
        },
        socket::ListeningSocketSource,
    },
};

struct ClientState {
    compositor_state: CompositorClientState,
}

impl ClientData for ClientState {}

struct State {
    display: Display<State>,
    compositor_state: CompositorState,
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
        println!("surface committed");
    }
}

smithay::delegate_compositor!(State);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop: EventLoop<'static, State> = EventLoop::try_new()?;

    let display = Display::<State>::new()?;
    let display_handle = display.handle();

    let compositor_state = CompositorState::new::<State>(&display_handle);

    let mut state = State {
        display,
        compositor_state,
    };

    let socket = ListeningSocketSource::new_auto()?;

    event_loop.handle().insert_source(socket, |stream, _, state| {
        state
            .display
            .handle()
            .insert_client(
                stream,
                Arc::new(ClientState {
                    compositor_state: CompositorClientState::default(),
                }),
            )
            .expect("failed to insert Wayland client");
    })?;

    println!("aurelianOS Wayland compositor starting...");

    loop {
        event_loop.dispatch(None, &mut state)?;
        state.display.flush_clients()?;
    }
}

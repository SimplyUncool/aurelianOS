// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use smithay::{
    backend::{
        allocator::gbm::{GbmAllocator, GbmBufferFlags, GbmDevice},
        drm::{
            DrmDevice, DrmDeviceFd, DrmEvent, DrmNode,
            compositor::{FrameFlags, PrimaryPlaneElement},
            exporter::gbm::GbmFramebufferExporter,
            output::{DrmOutput, DrmOutputManager},
        },
        egl::{EGLDisplay, context::EGLContext},
        renderer::{
            Color32F,
            element::{Id, Kind, solid::SolidColorRenderElement},
            gles::GlesRenderer,
            utils::CommitCounter,
        },
        session::{Session, libseat::LibSeatSession},
        udev::UdevBackend,
    },
    output::{Mode as WlMode, Output, PhysicalProperties, Subpixel},
    reexports::{drm::control::ModeTypeFlags, rustix::fs::OFlags, wayland_server::DisplayHandle},
    utils::{DeviceFd, Rectangle},
};
use smithay_drm_extras::drm_scanner::{DrmScanEvent, DrmScanner};

type Allocator = GbmAllocator<DrmDeviceFd>;
type Exporter = GbmFramebufferExporter<DrmDeviceFd>;
type OutputType = DrmOutput<Allocator, Exporter, (), DrmDeviceFd>;
type OutputManager = DrmOutputManager<Allocator, Exporter, (), DrmDeviceFd>;

pub struct Backend {
    _session: Option<LibSeatSession>,
    node: DrmNode,
    renderer: GlesRenderer,
    output_manager: OutputManager,
    output: OutputType,
    crtc: smithay::reexports::drm::control::crtc::Handle,
    frame_size: (i32, i32),
    frame_queued: bool,
}

impl Backend {
    pub fn initialize(
        mut session: LibSeatSession,
        display: &DisplayHandle,
    ) -> Result<(Self, smithay::backend::drm::DrmDeviceNotifier), Box<dyn std::error::Error>> {
        let seat = session.seat();
        tracing::info!(%seat, "initializing udev DRM backend");
        let udev = UdevBackend::new(&seat)?;
        let devices = udev
            .device_list()
            .map(|(id, path)| (id, path.to_path_buf()))
            .collect::<Vec<_>>();
        if devices.is_empty() {
            return Err("udev found no DRM devices on the active seat".into());
        }

        for (device_id, path) in devices {
            tracing::info!(?device_id, path = %path.display(), "trying DRM device");
            match Self::try_device(&mut session, path.as_path(), display) {
                Ok((mut backend, notifier)) => {
                    tracing::info!(?device_id, "selected DRM device");
                    backend._session = Some(session);
                    return Ok((backend, notifier));
                }
                Err(error) => {
                    tracing::warn!(?error, path = %path.display(), "DRM device unavailable for output rendering")
                }
            }
        }

        Err("no DRM device had a connected output with a usable mode and renderer".into())
    }

    fn try_device(
        session: &mut LibSeatSession,
        path: &Path,
        display: &DisplayHandle,
    ) -> Result<(Self, smithay::backend::drm::DrmDeviceNotifier), Box<dyn std::error::Error>> {
        let fd = session.open(
            path,
            OFlags::RDWR | OFlags::CLOEXEC | OFlags::NOCTTY | OFlags::NONBLOCK,
        )?;
        let fd = DrmDeviceFd::new(DeviceFd::from(fd));
        let node = DrmNode::from_file(&fd)?;
        tracing::info!(?node, "opened DRM device through libseat");
        let (drm, notifier) = DrmDevice::new(fd.clone(), true)
            .map_err(|error| format!("failed to initialize DRM/KMS device: {error}"))?;

        let gbm = GbmDevice::new(fd.clone())
            .map_err(|error| format!("failed to initialize GBM device: {error}"))?;
        let egl_display = unsafe { EGLDisplay::new(gbm.clone()) }
            .map_err(|error| format!("failed to initialize EGL display: {error}"))?;
        let context = EGLContext::new(&egl_display)
            .map_err(|error| format!("failed to create EGL context: {error}"))?;
        let mut renderer = unsafe { GlesRenderer::new(context) }
            .map_err(|error| format!("failed to initialize GLES renderer: {error}"))?;
        let renderer_formats = renderer
            .egl_context()
            .dmabuf_render_formats()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        tracing::info!(
            renderer = ?renderer,
            formats = renderer_formats.len(),
            "initialized EGL/GLES renderer"
        );

        let allocator = GbmAllocator::new(
            gbm.clone(),
            GbmBufferFlags::RENDERING | GbmBufferFlags::SCANOUT,
        );
        let exporter = GbmFramebufferExporter::new(gbm.clone(), Some(node));
        let color_formats = [
            smithay::backend::allocator::Fourcc::Abgr8888,
            smithay::backend::allocator::Fourcc::Argb8888,
        ];
        let mut output_manager = DrmOutputManager::new(
            drm,
            allocator,
            exporter,
            Some(gbm),
            color_formats,
            renderer_formats,
        );

        let mut scanner = DrmScanner::<smithay_drm_extras::drm_scanner::SimpleCrtcMapper>::new();
        let scan = scanner.scan_connectors(output_manager.device())?;
        for event in scan {
            if let DrmScanEvent::Connected {
                connector,
                crtc: Some(crtc),
            } = event
            {
                if connector.modes().is_empty() {
                    tracing::warn!(connector = ?connector.interface(), "connected DRM connector has no modes");
                    continue;
                }
                let mode = connector
                    .modes()
                    .iter()
                    .find(|mode| mode.mode_type().contains(ModeTypeFlags::PREFERRED))
                    .copied()
                    .unwrap_or(connector.modes()[0]);
                let connector_name = format!(
                    "{}-{}",
                    connector.interface().as_str(),
                    connector.interface_id()
                );
                tracing::info!(%connector_name, crtc = ?crtc, width = mode.size().0, height = mode.size().1, refresh_hz = mode.vrefresh(), "selected connected connector mode");

                let wayland_output = Output::new(
                    connector_name.clone(),
                    PhysicalProperties {
                        size: connector
                            .size()
                            .map(|(w, h)| (w as i32, h as i32).into())
                            .unwrap_or_default(),
                        subpixel: Subpixel::Unknown,
                        make: "Unknown".into(),
                        model: "DRM output".into(),
                    },
                );
                let wl_mode = WlMode::from(mode);
                wayland_output.set_preferred(wl_mode);
                wayland_output.change_current_state(Some(wl_mode), None, None, Some((0, 0).into()));
                let planes = output_manager.device().planes(&crtc)?;
                let sync_elements = smithay::backend::drm::output::DrmOutputRenderElements::<
                    GlesRenderer,
                    SolidColorRenderElement,
                >::new();
                let output = output_manager
                    .initialize_output::<_, SolidColorRenderElement>(
                        crtc,
                        mode,
                        &[connector.handle()],
                        &wayland_output,
                        Some(planes),
                        &mut renderer,
                        &sync_elements,
                    )
                    .map_err(|error| format!("failed to initialize DRM output: {error}"))?;
                wayland_output.create_global::<crate::state::State>(display);
                let frame_size = (mode.size().0.into(), mode.size().1.into());
                let mut backend = Self {
                    _session: None,
                    node,
                    renderer,
                    output_manager,
                    output,
                    crtc,
                    frame_size,
                    frame_queued: false,
                };
                backend
                    .render_test_frame(frame_size.0, frame_size.1)
                    .map_err(|error| {
                        format!("failed to render and queue initial frame: {error}")
                    })?;
                tracing::info!(%connector_name, "DRM output initialized and first frame queued");
                return Ok((backend, notifier));
            }
        }
        Err("DRM device has no connected connector with an available CRTC".into())
    }

    fn render_test_frame(
        &mut self,
        width: i32,
        height: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let inset = (width.min(height) / 6).max(1);
        let geometry = Rectangle::new(
            (inset, inset).into(),
            ((width - 2 * inset).max(1), (height - 2 * inset).max(1)).into(),
        );
        let element = SolidColorRenderElement::new(
            Id::new(),
            geometry,
            CommitCounter::default(),
            Color32F::new(0.32, 0.58, 0.98, 1.0),
            Kind::Unspecified,
        );
        let elements = [element];
        let rendered = self.output.render_frame(
            &mut self.renderer,
            &elements,
            Color32F::new(0.035, 0.025, 0.12, 1.0),
            FrameFlags::empty(),
        )?;
        if rendered.needs_sync() {
            if let PrimaryPlaneElement::Swapchain(element) = &rendered.primary_element {
                element.sync.wait()?;
            }
        }
        drop(rendered);
        self.output.queue_frame(())?;
        self.frame_queued = true;
        tracing::info!("static test scene rendered and submitted to KMS");
        Ok(())
    }

    pub fn on_drm_event(&mut self, event: DrmEvent) {
        match event {
            DrmEvent::VBlank(crtc) if crtc == self.crtc && self.frame_queued => {
                match self.output.frame_submitted() {
                    Ok(_) => {
                        tracing::info!(device = ?self.node, ?crtc, "initial page-flip completed; frame event received")
                    }
                    Err(error) => {
                        tracing::error!(?error, "failed to complete DRM frame submission")
                    }
                }
                self.frame_queued = false;
            }
            DrmEvent::VBlank(crtc) => tracing::debug!(?crtc, "unmatched DRM vblank"),
            DrmEvent::Error(error) => tracing::error!(?error, "DRM event error"),
        }
    }

    pub fn on_session_event(&mut self, event: smithay::backend::session::Event) {
        match event {
            smithay::backend::session::Event::PauseSession => {
                self.output_manager.pause();
                tracing::info!("paused DRM output for inactive libseat session");
            }
            smithay::backend::session::Event::ActivateSession => {
                match self.output_manager.activate(false) {
                    Ok(()) => {
                        tracing::info!("reactivated DRM output after libseat session resumed");
                        if let Err(error) =
                            self.render_test_frame(self.frame_size.0, self.frame_size.1)
                        {
                            tracing::error!(
                                ?error,
                                "failed to redraw test scene after session resume"
                            );
                        }
                    }
                    Err(error) => tracing::error!(?error, "failed to reactivate DRM output"),
                }
            }
        }
    }
}

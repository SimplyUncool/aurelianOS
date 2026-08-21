# Decision 0002: Wayland Compositor

## Status

Accepted

## Decision

aurelianOS will use **Smithay** as the foundation for its Wayland compositor.

aurelianOS will implement its own compositor behavior and window-management policy on top of Smithay rather than adopting an existing desktop compositor such as KWin or Mutter.

## Requirements

The compositor must support the goals established for aurelianOS:

* Wayland-first desktop architecture
* Reliable multi-monitor support
* Fractional scaling
* HDR and VRR
* Modern GPU rendering
* Keyboard, mouse, touchpad, and other input devices
* XWayland for compatibility with X11 applications
* Good gaming behavior
* Screen capture and recording
* Hardware hotplugging
* Suspend and resume
* A foundation suitable for the aurelianOS desktop shell
* Advanced users must retain access to the underlying Linux environment

## Alternatives Considered

### KWin

KWin provides a mature, feature-rich compositor and already handles much of the difficult desktop integration work.

It was not selected because aurelianOS would inherit KDE's compositor architecture and a significant amount of behavior that we ultimately want to control ourselves.

### Mutter

Mutter is similarly mature and provides the foundation for GNOME's desktop experience.

It was not selected because aurelianOS is not intended to reproduce GNOME's workflow or depend on GNOME's shell architecture.

### wlroots

wlroots provides reusable infrastructure for Wayland compositor development and has been used by numerous compositors.

It remains a viable alternative, but Smithay's Rust-based architecture and the amount of system-level Wayland, DRM, input, session, and XWayland infrastructure it provides better fit the direction of aurelianOS.

### Custom compositor from scratch

This would provide maximum control but would require reimplementing a substantial amount of already-solved Wayland and graphics infrastructure.

The development and maintenance cost is not justified.

## Why Smithay

Smithay describes itself as a library providing building blocks for Wayland compositors rather than a complete desktop compositor. It handles much of the low-level Wayland and system interaction while leaving window-management and drawing policy to the compositor developer.

Smithay 0.7 provides support for components including DRM, GBM, libinput, libseat, udev, Vulkan, OpenGL/EGL, multi-renderer support, Wayland protocol handling, and XWayland.

This matches the aurelianOS requirement for a custom desktop experience without requiring the project to independently implement the entire low-level Wayland stack.

Smithay is also MIT-licensed, which makes its license compatible with the commercial licensing strategy being considered for aurelianOS.

## Trade-offs

Choosing Smithay means aurelianOS is taking responsibility for significant compositor development.

Smithay does **not** provide the complete window-management and desktop behavior required by a finished operating system. Those components will have to be implemented by aurelianOS.

This increases development and maintenance requirements compared with adopting KWin or Mutter.

In exchange, aurelianOS gains substantially greater control over:

* Window management
* Desktop behavior
* Rendering policy
* Input behavior
* System integration
* Future desktop features
* Integration with the aurelianOS shell

## Consequences

The compositor will be developed as an aurelianOS component built around Smithay.

The initial compositor should prioritize:

1. Booting into a Wayland session
2. DRM/KMS display output
3. Keyboard and pointer input
4. Basic window management
5. XWayland support
6. Multi-monitor support
7. Basic rendering
8. Application launching
9. Proper session termination

Advanced functionality such as HDR, VRR, fractional scaling, screen capture, and advanced window management will be implemented progressively rather than blocking the initial prototype.

## Future Review

This decision can be revisited if Smithay's development direction, hardware compatibility, required protocol support, or maintenance burden becomes incompatible with aurelianOS.

A future decision should replace this one rather than silently changing the architecture.

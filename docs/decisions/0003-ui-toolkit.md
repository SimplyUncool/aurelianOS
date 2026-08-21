# Decision 0003: UI Toolkit

## Status

Accepted

## Decision

aurelianOS will use **Slint** as the UI toolkit for the aurelianOS desktop shell and graphical system interfaces.

The aurelianOS shell will be implemented in Rust and use Slint for its user interface layer.

## Requirements

The UI toolkit must support:

* Rust
* Native Linux desktop applications
* GPU-accelerated rendering
* Wayland-based environments
* High-DPI and fractional scaling
* Animations and transitions
* Consistent application-wide theming
* Accessibility
* Low resource usage
* A declarative UI model
* Long-term maintainability
* Commercial distribution without requiring aurelianOS to become open source

## Alternatives Considered

### GTK

GTK is a mature Linux GUI toolkit with extensive ecosystem support.

It was not selected because aurelianOS is intended to have a highly controlled and custom visual system rather than closely following the conventions of an existing Linux desktop environment.

### Qt

Qt provides an extremely mature and feature-rich UI framework with strong desktop support.

It was not selected because its commercial licensing model introduces additional licensing and cost considerations that are undesirable for the initial aurelianOS development model.

### Iced

Iced provides a Rust-native declarative GUI architecture and is particularly relevant to Wayland/Rust desktop development.

It was considered, but Slint provides a more established declarative UI language and a stronger separation between UI design and application logic.

### Custom UI renderer

A custom renderer would provide maximum control over the aurelianOS interface.

It was rejected because implementing and maintaining a complete UI toolkit would substantially increase development complexity without providing enough benefit at this stage.

## Why Slint

Slint is a declarative GUI toolkit with official Rust support and Linux desktop support. It is designed to allow the UI description to remain separate from application logic.

Slint also provides GPU-accelerated rendering and is designed for relatively low resource usage.

The declarative approach is well suited to aurelianOS because the project intends to maintain a consistent visual language across the desktop shell, settings, system dialogs, and other first-party interfaces.

Slint's royalty-free desktop license permits proprietary desktop applications without requiring the application's source code to be released under the GPL, provided the required attribution conditions are met.

This is compatible with the current commercial licensing direction of aurelianOS.

## Attribution

aurelianOS will provide the required Slint attribution through the system's About interface.

The attribution mechanism should be implemented as part of the standard:

`Settings → About aurelianOS → Legal & Licenses`

interface.

## Trade-offs

Using Slint introduces dependency on an external UI framework and therefore on its development direction, API stability, and licensing terms.

Slint has also been actively improving its desktop capabilities, meaning some desktop-oriented functionality may continue to evolve.

These trade-offs are preferable to maintaining a complete custom UI framework during the early development of aurelianOS.

## Consequences

The aurelianOS graphical stack will be structured approximately as:

```
Linux
  ↓
Wayland
  ↓
Smithay
  ↓
aurelianOS compositor
  ↓
aurelianOS shell
  ↓
Slint UI
```

The compositor and UI toolkit remain separate components.

Smithay is responsible for compositor infrastructure and Wayland integration, while Slint is responsible for rendering the graphical interfaces used by the aurelianOS shell.

## Future Review

This decision may be revisited if Slint's technical capabilities, licensing, performance, accessibility, maintenance, or long-term compatibility become unsuitable for aurelianOS.

A future decision should replace this decision rather than silently changing the architecture.

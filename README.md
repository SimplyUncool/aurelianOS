# aurelianOS

**A Linux desktop operating system built around simplicity, control, performance, and a cohesive user experience.**

aurelianOS is an experimental OS project for x86_64 PCs. The goal is to build a complete desktop environment that feels intentional from the system level up rather than assembling an existing Linux desktop and reskinning it.

## Status

**Early development / experimental**

The project is actively being rebuilt around a bootc-based system image, a custom Wayland compositor, and a custom desktop shell.

Current work includes:

* A bootable Linux system for x86_64 PCs
* A Fedora bootc-based system image
* A Rust compositor built with Smithay 0.7
* A functioning Wayland server and socket
* Wayland protocol testing with `wayland-info`
* A Slint-based desktop shell prototype
* A fullscreen, resolution-independent desktop layout
* A translucent glass-style desktop interface
* A floating application dock
* An application launcher
* Tabler SVG icons
* Bundled Inter typography
* A custom 4K desktop wallpaper
* An original aurelianOS logo and visual identity

The compositor and shell are still being developed independently before being integrated into a complete desktop session.

## Vision

aurelianOS is intended to make Linux approachable without hiding what is underneath it.

For everyday users, the system should provide a polished graphical experience with sensible defaults and minimal configuration.

For advanced users, the underlying Linux environment should remain accessible, including the terminal, filesystem, services, development tools, and system configuration.

The long-term goal is a desktop that feels cohesive at every level:

* System installation
* Login and startup
* Desktop shell
* Window management
* Applications
* Settings
* Notifications
* Updates and recovery
* Hardware configuration
* Gaming and Windows application compatibility

## Architecture

The project is being developed as several cooperating layers:

```text
                    aurelianOS
                         │
        ┌────────────────┼────────────────┐
        │                │                │
      bootc          compositor         shell
        │                │                │
   system image      Rust + Smithay   Slint + assets
                         │
                      Wayland
```

### Current components

**System**

The bootable system is being built around Fedora bootc.

**Compositor**

The compositor is written in Rust using Smithay and is responsible for the Wayland protocol, surfaces, input, output management, and eventually the complete rendering pipeline.

**Shell**

The current desktop shell is being prototyped in Slint. It provides the visual layer of aurelianOS, including the desktop, dock, launcher, typography, icons, and visual design system.

## Visual Design

The current interface is based around a restrained glass aesthetic:

* Large, unobstructed wallpaper areas
* Translucent surfaces
* Rounded floating controls
* Minimal persistent UI
* Consistent iconography
* Inter typography
* Blue and violet visual accents
* Subtle interaction states

The long-term design includes real compositor-level effects such as backdrop blur, rather than relying solely on transparent UI surfaces.

## Project Structure

```text
aurelianOS/
├── assets/          # Wallpapers, fonts, icons and other assets
├── bootc/           # Bootable system and current compositor/shell work
│   ├── compositor/  # Rust + Smithay compositor
│   ├── shell/       # Slint desktop shell
│   └── ...
├── buildroot/       # Earlier system build work
├── compatibility/   # Compatibility work
├── configs/         # System configuration
├── desktop/         # Desktop-related project files
├── docs/            # Documentation
├── installer/       # Installer work
├── scripts/         # Development and system scripts
├── system/          # System components
└── tests/           # Tests
```

Some directories represent earlier stages of development and are being replaced or reorganized as the project evolves.

## Development

aurelianOS is currently developed and tested primarily on x86_64 systems, with QEMU used heavily during development.

The compositor and shell are standalone Rust projects under `bootc/` and can be developed independently while the surrounding OS infrastructure is built.

## Roadmap

### Foundation

* [x] Bootable Linux system
* [x] bootc-based system development
* [x] Initial Wayland compositor
* [x] Wayland client connectivity
* [x] Initial desktop shell

### Compositor

* [ ] Proper output and renderer pipeline
* [ ] Surface rendering
* [ ] Input management
* [ ] Window management
* [ ] Multiple outputs
* [ ] GPU-accelerated effects
* [ ] Backdrop blur and other glass effects

### Desktop

* [ ] Finalize visual design
* [ ] Real application launcher
* [ ] Window decorations
* [ ] Notifications
* [ ] System status menus
* [ ] Settings application
* [ ] File manager integration
* [ ] Session management

### System

* [ ] Installer
* [ ] Hardware configuration
* [ ] Update and rollback workflow
* [ ] Recovery environment
* [ ] Application management
* [ ] Gaming support
* [ ] Windows application compatibility

## Contributing

aurelianOS is currently developed primarily as a solo project, but contributions, testing, bug reports, technical discussion, and ideas are welcome.

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Supporting the Project

aurelianOS is developed independently. If you want to support development, donations are appreciated and help fund hardware, testing, hosting, and other project costs.

**Bitcoin Address:** bc1q8l5tcp6rx6q06qj323dqt4q3ywsh7x6wus2jjn  
**Litecoin Address:** ltc1qg3rmw5zh4k47wqm30utsv7hqaqalsk3e5uhaw7  
**BNB Smart Chain Address:** 0x2321cBfFA5CaA335F10C71Eb078D61E662E201Be  
**USDT (BEP-20) Address:** 0x2321cBfFA5CaA335F10C71Eb078D61E662E201Be  
**USDC (Solana) Address:** F9fdD2r9PatQds68wHjfSCachHndt91wefrxD5WPFkJs  

## Licensing

aurelianOS source code is licensed under the **Apache License 2.0** unless otherwise specified.

Third-party software, fonts, icons, artwork, and other external materials retain their respective licenses.

The **aurelianOS** name, logo, and official branding are not granted for use under the Apache License. See [TRADEMARKS.md](TRADEMARKS.md).

## Disclaimer

aurelianOS is experimental software.

Do not use development builds on systems containing data you cannot afford to lose. Features, interfaces, architecture, and project direction may change substantially during development.

---

**aurelianOS — Linux, rebuilt around the desktop.**

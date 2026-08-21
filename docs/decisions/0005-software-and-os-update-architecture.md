```md
# Decision 0005: Software and OS Update Architecture

## Status

Accepted

## Decision

aurelianOS will use an **atomic base operating system with Flatpak applications**.

The base operating system and desktop applications will be treated as separate layers:

- The base OS will be distributed and updated as a complete atomic system image.
- Desktop applications will be distributed through Flatpak.
- aurelianOS will eventually provide a unified graphical software-management interface over these systems.

aurelianOS will not initially create its own general-purpose package format or package manager.

## Requirements

The software architecture must provide:

- Atomic operating-system updates
- Safe rollback to a previous OS version
- Verification of system updates
- Separation between the base OS and applications
- Application sandboxing
- Installation and removal of graphical applications without rebuilding the OS
- Independent application updates
- Reproducible system images
- A straightforward graphical software-management experience
- A usable command-line environment for advanced users
- A path toward reliable offline recovery

## Alternatives Considered

### Traditional Mutable Distribution

A conventional package-managed root filesystem was considered.

This approach is well established, but updating large numbers of system packages individually increases the number of possible partial-update states and makes reliable rollback more difficult.

It also conflicts with the goal of making the base operating system a predictable, reproducible system image.

### Nix / NixOS-style Architecture

Nix demonstrates strong reproducibility, atomic operations, generations, and rollback.

However, its package model and declarative configuration system introduce substantial additional complexity for users and developers.

aurelianOS does not currently require the full Nix model to achieve atomic system updates.

### aurelianOS-native Package Manager

A native package manager was considered.

It was rejected for the initial architecture because implementing package dependency resolution, installation tracking, file ownership, upgrades, removals, signatures, repositories, rollback, and security would create a large maintenance burden.

The project should first use established technologies and concentrate development effort on the aurelianOS user experience and operating-system architecture.

### Buildroot as the Long-Term Package System

Buildroot will continue to be used during the current prototype stage.

However, Buildroot is primarily designed to build complete Linux systems for embedded systems rather than operate as a conventional desktop distribution package manager. Its documentation explicitly discusses why it does not generate conventional binary packages and why package-management functionality is substantially more complicated than simply tracking installed files. :contentReference[oaicite:0]{index=0}

Therefore, Buildroot is not considered a sufficient long-term software-management architecture by itself.

## Why Atomic Updates

An atomic base OS allows an entire system revision to be prepared separately from the currently running system.

The new revision can then become active as a single deployment, reducing the risk of leaving the machine in a partially updated state.

Fedora Atomic Desktops demonstrate this model in production: system updates take effect together after reboot, and a previous system version is retained for rollback. :contentReference[oaicite:1]{index=1}

This matches the aurelianOS goal of making system updates safer and easier to recover from.

## Why Flatpak

Desktop applications will be separated from the base operating system and distributed through Flatpak.

This provides:

- Application isolation
- Independent application updates
- Permission controls
- Separation from the immutable system image
- Access to both open-source and proprietary applications

Fedora's Atomic desktops use this same separation, with graphical applications delivered as Flatpaks while the base system is updated atomically. :contentReference[oaicite:2]{index=2}

## Proposed Architecture

    aurelianOS
        │
        ├── Base OS
        │     └── Atomic system image
        │
        ├── System updates
        │     └── Download → Verify → Deploy → Reboot
        │
        ├── Desktop applications
        │     └── Flatpak
        │
        ├── Development environment
        │     └── Isolated development tools
        │
        └── aurelianOS Software
              └── Unified graphical management interface

## Build System Implication

Buildroot remains the current development and prototyping build system.

However, the project will not assume that Buildroot is the permanent foundation of aurelianOS.

Buildroot's documented purpose is building complete Linux systems, particularly for embedded systems. :contentReference[oaicite:3]{index=3}

The long-term base-image technology will therefore be evaluated separately.

Possible future architectures may include an OSTree/bootc-style system-image model or another technology capable of producing and deploying atomic desktop operating-system images.

Fedora's current Atomic Desktop architecture demonstrates the viability of combining image-based operating-system updates with Flatpak applications, and current Fedora development is moving further toward image-builder and bootc-based workflows. :contentReference[oaicite:4]{index=4}

## User Experience

The user should not need to understand the underlying technologies.

The eventual aurelianOS Software interface should provide a single place to:

- Browse applications
- Install applications
- Remove applications
- Update applications
- View application permissions
- Check for system updates
- Start system updates
- View update history
- Recover from a failed system update

The underlying implementation may use multiple technologies, but the user experience should remain consistent.

## Advanced Mode

Advanced users will retain access to the underlying Linux environment.

The atomic architecture should not prevent users from using:

- The terminal
- Shell tools
- Development environments
- Containers
- System diagnostics
- Linux configuration tools

Normal mode should hide unnecessary complexity rather than remove advanced functionality.

## Trade-offs

An atomic system introduces additional storage requirements because previous system revisions may need to be retained for rollback.

It also requires a more sophisticated image-generation and deployment system than a traditional mutable root filesystem.

Flatpak applications additionally depend on runtimes and sandbox permissions, which can introduce compatibility considerations for applications requiring unusual system integration.

These trade-offs are considered acceptable because reliability, rollback, application isolation, and a predictable base system are core aurelianOS goals.

## Consequences

The software architecture will be developed around the separation between:

1. The operating system
2. Desktop applications
3. Development environments

The project will not build a custom package manager merely to replace Buildroot's lack of target-side package management.

Instead, future aurelianOS development will focus on providing a polished software-management experience over established underlying technologies.

## Future Review

This decision may be revisited when the project reaches the point where the long-term base operating-system architecture must be selected.

The next architectural decision should evaluate **Buildroot versus alternative image-building and atomic-OS foundations**.

That decision will determine how the current Buildroot prototype evolves into a distributable aurelianOS system.
```

````md
# Decision 0004: Init System

## Status

Accepted

## Decision

aurelianOS will use **systemd** as its init system and system service manager.

## Requirements

The init and service-management system must support:

- Reliable system initialization
- Service dependency management
- Parallel service startup
- Service supervision and restart
- User sessions
- D-Bus integration
- Device management
- Logging
- Graphical-session startup
- Networking and other system services
- Integration with a full desktop Linux environment
- Long-term maintainability

## Alternatives Considered

### BusyBox init

BusyBox init is Buildroot's default and is appropriate for many embedded systems.

It was not selected because aurelianOS is intended to be a full desktop operating system with interconnected services, graphical sessions, user sessions, and substantial runtime management requirements.

### OpenRC

OpenRC provides a lightweight service-management system with dependency-based startup.

It was not selected because systemd provides broader integration with the desktop Linux ecosystem and provides functionality we expect to need as a complete desktop operating system.

### SysVinit

SysVinit is mature and simple but relies on a traditional initialization and service-management model.

It was not selected because it does not provide the integrated service, session, device, logging, and activation infrastructure desired for aurelianOS.

## Why systemd

Buildroot describes systemd as a newer-generation init system that provides substantially more than traditional init systems, including parallelization, socket and D-Bus activation, on-demand daemon startup, and Linux cgroup-based process tracking. Buildroot specifically notes that systemd is useful for more complex systems involving D-Bus and interconnected services. :contentReference[oaicite:0]{index=0}

This makes systemd a better fit for aurelianOS than Buildroot's default BusyBox init.

Buildroot provides dedicated systemd integration, including systemd service units, systemd-based device management through udev, and configuration for the system's default target. :contentReference[oaicite:1]{index=1}

## Desktop Integration

systemd will be responsible for the system-level service environment.

The aurelianOS graphical session will be started and managed through systemd rather than through a collection of custom boot scripts.

This provides a consistent mechanism for managing:

- System services
- User sessions
- Graphical sessions
- Device-related services
- Networking
- Background daemons
- Service dependencies

D-Bus integration is also important because Buildroot's systemd integration requires a D-Bus daemon, providing the interprocess communication infrastructure used by many desktop Linux components. :contentReference[oaicite:2]{index=2}

## Trade-offs

systemd is substantially larger and has more dependencies than simpler init systems.

Buildroot notes that systemd brings dependencies including D-Bus and udev and requires a more capable toolchain configuration. Current Buildroot integration requires, among other things, glibc, dynamic linking, threading support, and sufficiently recent kernel headers and compilers. :contentReference[oaicite:3]{index=3}

These additional requirements are acceptable because aurelianOS targets modern desktop hardware rather than minimal embedded systems.

## Buildroot Configuration

The aurelianOS Buildroot configuration will select:

```text
BR2_INIT_SYSTEMD=y
````

The exact systemd services and default target will be configured as the graphical desktop architecture develops.

## Consequences

The base system architecture will now be:

```
Linux kernel
    ↓
systemd
    ↓
system services
    ↓
graphical session
    ↓
Smithay compositor
    ↓
aurelianOS shell
    ↓
Slint UI
```

This also means future aurelianOS services should preferably integrate with systemd using native service units rather than custom boot scripts.

## Future Review

This decision may be revisited if systemd's resource requirements, architecture, maintenance burden, or integration with the aurelianOS desktop become unsuitable.

A future decision should replace this decision rather than silently changing the init architecture.

```
```


# Decision 0006: Long-Term OS Foundation

## Status

Accepted

## Decision

aurelianOS will use **bootc and OCI images** as the long-term foundation for building, distributing, installing, and updating the operating system.

Buildroot will remain the current prototype build system during the transition but will not be treated as the final aurelianOS system architecture.

## Requirements

The long-term OS foundation must support:

- Atomic operating-system updates
- Rollback to previous system deployments
- Reproducible system images
- Versioned OS releases
- Cryptographic verification of system images
- OCI-compatible image distribution
- Kernel and userspace versioning as one system revision
- Persistent machine configuration across updates
- Separation of immutable system files from persistent state
- VM-based development and testing
- Installation to physical hardware
- Recovery from failed updates
- Integration with systemd
- Integration with the aurelianOS update system

## Alternatives Considered

### Buildroot

Buildroot is currently used to prototype aurelianOS.

It is effective at generating complete Linux systems, including the kernel, root filesystem, and boot components.

However, Buildroot is primarily designed as a Linux system builder rather than a general-purpose desktop distribution foundation. Its update and package-management model does not provide the architecture required by aurelianOS's long-term desktop goals. ([buildroot.org](https://buildroot.org/downloads/manual/manual.pdf))

Buildroot will therefore remain useful during early development but will eventually be replaced as the primary OS build system.

### Yocto / OpenEmbedded

Yocto and OpenEmbedded provide a powerful framework for building customized Linux distributions and support complex package and hardware configurations.

They were considered seriously because they are more distribution-oriented than Buildroot.

However, the additional build-system complexity is not currently justified when aurelianOS's desired architecture is based around complete immutable system images rather than traditional package-based system updates.

### OSTree

OSTree provides atomic filesystem deployments and rollback and is an important technology in the immutable Linux ecosystem.

It remains relevant to the aurelianOS architecture, but OSTree alone does not define the complete image-building and distribution workflow required by the project.

bootc provides a higher-level image-based workflow built around OCI images while using the underlying atomic deployment model.

### Traditional Mutable Distribution

A traditional mutable root filesystem with package-level system updates was rejected because it makes complete-system rollback and reproducible system revisions more difficult.

It also conflicts with the atomic system architecture established in Decision 0005.

## Why bootc

bootc is designed around bootable OCI container images.

A bootable image contains the operating system userspace, Linux kernel, and system manager. The image can be built using standard OCI-compatible container tooling and stored in a normal container registry. :contentReference[oaicite:1]{index=1}

bootc can apply a new image to an installed system and switch the system to the new deployment after reboot. The previous operating-system deployment can remain available through the bootloader, allowing rollback if the new deployment fails. :contentReference[oaicite:2]{index=2}

This directly supports the atomic update architecture established in Decision 0005.

## System State

The immutable operating-system image will contain the base aurelianOS system.

Machine-specific and persistent state will remain outside the immutable image.

The architecture will primarily separate:

    /usr
        Immutable operating-system userspace

    /boot
        Kernel and boot components associated with the deployment

    /etc
        Persistent system configuration

    /var
        Persistent variable system data

This allows the operating system itself to be replaced without unnecessarily destroying machine configuration or persistent data. :contentReference[oaicite:3]{index=3}

## Image Architecture

The long-term build pipeline will follow approximately:

    aurelianOS source
        ↓
    OCI image definition
        ↓
    Build
        ↓
    Test
        ↓
    Sign
        ↓
    OCI registry
        ↓
    aurelianOS updater
        ↓
    bootc
        ↓
    Atomic deployment
        ↓
    Reboot

The exact CI, registry, signing, and release infrastructure will be determined by later architectural decisions.

## Update Model

System updates will replace the complete base operating-system deployment rather than individually modifying system packages.

The expected update flow is:

    Check for update
        ↓
    Download image
        ↓
    Verify image
        ↓
    Stage deployment
        ↓
    Reboot
        ↓
    Boot new deployment
        ↓
    Verify successful startup

If the new deployment is unusable, the previous deployment can be selected for rollback through the bootloader. :contentReference[oaicite:4]{index=4}

## Applications

Desktop applications remain separate from the base OS.

As established by Decision 0005, graphical applications will primarily use Flatpak.

The base OS image therefore contains the operating-system infrastructure while Flatpak provides the application layer.

## Development Strategy

The existing Buildroot implementation will not be discarded immediately.

Buildroot will continue to be used while the bootc-based system is developed and tested.

The transition will occur incrementally:

    Buildroot prototype
        ↓
    bootc prototype
        ↓
    bootc + aurelianOS userspace
        ↓
    Smithay compositor
        ↓
    Slint shell
        ↓
    Physical hardware testing
        ↓
    Production aurelianOS image

The Buildroot prototype should remain available until the bootc system can independently boot and provide the required development environment.

## Trade-offs

bootc introduces additional architectural complexity compared with generating a conventional root filesystem.

The project will also become dependent on the bootable-container ecosystem, OCI image tooling, bootloader integration, and the capabilities of the underlying filesystem and update infrastructure.

The image-based model also requires sufficient storage for multiple deployments and requires careful handling of persistent state.

These trade-offs are accepted because atomic updates, rollback, reproducibility, and a clean separation between the operating system and applications are central aurelianOS requirements.

## Consequences

The long-term aurelianOS architecture will be image-based rather than package-based.

The base operating system will be treated as a versioned artifact that can be built, tested, distributed, installed, and rolled back as a complete unit.

This also allows the same OS image model to be used across virtual machines and physical hardware.

## Future Review

This decision may be revisited if bootc's capabilities, hardware compatibility, ecosystem maturity, or maintenance requirements become unsuitable for aurelianOS.

A future decision should replace this decision rather than silently changing the OS foundation.

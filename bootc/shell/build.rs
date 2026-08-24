// SPDX-License-Identifier: Apache-2.0

fn main() {
    slint_build::compile("ui/main.slint").unwrap();

    println!("cargo:rerun-if-changed=ui/main.slint");
    println!("cargo:rerun-if-changed=ui/theme.slint");
    println!("cargo:rerun-if-changed=ui/panel.slint");
    println!("cargo:rerun-if-changed=ui/dock.slint");
    println!("cargo:rerun-if-changed=ui/launcher.slint");

    println!("cargo:rerun-if-changed=../../../assets/wallpaper.jpg");
    println!("cargo:rerun-if-changed=../../../assets/icons");

    println!("cargo:rerun-if-changed=../../../assets/fonts/Inter-Regular.ttf");
    println!("cargo:rerun-if-changed=../../../assets/fonts/Inter-Medium.ttf");
    println!("cargo:rerun-if-changed=../../../assets/fonts/Inter-SemiBold.ttf");
}

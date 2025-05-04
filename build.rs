use bindgen::Builder as BindgenBuilder;

use std::{
    env::{set_var, var},
    path::Path,
};

const INCLUDE_COMMON: [&str; 13] = [
    "attribute.c",
    "attrlist.c",
    "attrstr.c",
    "areaevents.c",
    "control.c",
    "debug.c",
    "matrix.c",
    "opentype.c",
    "shouldquit.c",
    "tablemodel.c",
    "tablevalue.c",
    "userbugs.c",
    "utf.c",
];
const INCLUDE_WINDOWS: [&str; 61] = [
    "alloc.cpp",
    "area.cpp",
    "areadraw.cpp",
    "areaevents.cpp",
    "areascroll.cpp",
    "areautil.cpp",
    "attrstr.cpp",
    "box.cpp",
    "button.cpp",
    "checkbox.cpp",
    "colorbutton.cpp",
    "colordialog.cpp",
    "combobox.cpp",
    "container.cpp",
    "control.cpp",
    "d2dscratch.cpp",
    "datetimepicker.cpp",
    "debug.cpp",
    "draw.cpp",
    "drawmatrix.cpp",
    "drawpath.cpp",
    "drawtext.cpp",
    "dwrite.cpp",
    "editablecombo.cpp",
    "entry.cpp",
    "events.cpp",
    "fontbutton.cpp",
    "fontdialog.cpp",
    "fontmatch.cpp",
    "form.cpp",
    "graphemes.cpp",
    "grid.cpp",
    "group.cpp",
    "image.cpp",
    "init.cpp",
    "label.cpp",
    "main.cpp",
    "menu.cpp",
    "multilineentry.cpp",
    "opentype.cpp",
    "parent.cpp",
    "progressbar.cpp",
    "radiobuttons.cpp",
    "separator.cpp",
    "sizing.cpp",
    "slider.cpp",
    "spinbox.cpp",
    "stddialogs.cpp",
    "tab.cpp",
    "table.cpp",
    "tabledispinfo.cpp",
    "tabledraw.cpp",
    "tableediting.cpp",
    "tablemetrics.cpp",
    "tabpage.cpp",
    "text.cpp",
    "utf16.cpp",
    "utilwin.cpp",
    "window.cpp",
    "winpublic.cpp",
    "winutil.cpp",
];
const INCLUDE_UNIX: [&str; 44] = [
    "alloc.c",
    "area.c",
    "attrstr.c",
    "box.c",
    "button.c",
    "cellrendererbutton.c",
    "checkbox.c",
    "child.c",
    "colorbutton.c",
    "combobox.c",
    "control.c",
    "datetimepicker.c",
    "debug.c",
    "draw.c",
    "drawmatrix.c",
    "drawpath.c",
    "drawtext.c",
    "editablecombo.c",
    "entry.c",
    "fontbutton.c",
    "fontmatch.c",
    "form.c",
    "future.c",
    "graphemes.c",
    "grid.c",
    "group.c",
    "image.c",
    "label.c",
    "main.c",
    "menu.c",
    "multilineentry.c",
    "opentype.c",
    "progressbar.c",
    "radiobuttons.c",
    "separator.c",
    "slider.c",
    "spinbox.c",
    "stddialogs.c",
    "tab.c",
    "table.c",
    "tablemodel.c",
    "text.c",
    "util.c",
    "window.c",
];
const INCLUDE_DARWIN: [&str; 50] = [
    "aat.m",
    "alloc.m",
    "area.m",
    "areaevents.m",
    "attrstr.m",
    "autolayout.m",
    "box.m",
    "button.m",
    "checkbox.m",
    "colorbutton.m",
    "combobox.m",
    "control.m",
    "datetimepicker.m",
    "debug.m",
    "draw.m",
    "drawtext.m",
    "editablecombo.m",
    "entry.m",
    "event.m",
    "fontbutton.m",
    "fontmatch.m",
    "fonttraits.m",
    "fontvariation.m",
    "form.m",
    "future.m",
    "graphemes.m",
    "grid.m",
    "group.m",
    "image.m",
    "label.m",
    "main.m",
    "menu.m",
    "multilineentry.m",
    "nstextfield.m",
    "opentype.m",
    "progressbar.m",
    "radiobuttons.m",
    "scrollview.m",
    "separator.m",
    "slider.m",
    "spinbox.m",
    "stddialogs.m",
    "tab.m",
    "table.m",
    "tablecolumn.m",
    "text.m",
    "undocumented.m",
    "util.m",
    "window.m",
    "winmoveresize.m",
];

fn main() -> anyhow::Result<()> {
    // Determine build platform
    let target_os = var("CARGO_CFG_TARGET_OS").unwrap();
    let target_triple = var("TARGET").unwrap();
    let apple = target_triple.contains("apple");
    let unix = cfg!(target_family = "unix") && !apple;
    let out_dir = var("OUT_DIR")?;

    // Generate libui bindings on the fly
    let bindings = BindgenBuilder::default()
        .header("src/ui.h")
        .opaque_type("max_align_t") // For some reason this ends up too large
        //.rustified_enum(".*")
        .clang_args(["-target", &target_triple])
        .trust_clang_mangling(false) // clang sometimes wants to treat these functions as C++
        .generate()?;

    let out_path = Path::new(&out_dir);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    let mut base_config = cc::Build::new();
    let src_base = Path::new("src");

    // libui might emit lots of warning we can do nothing about here
    base_config.warnings(false);

    // Add source files that are common to all platforms
    base_config.include(src_base.join("common"));

    for filename in INCLUDE_COMMON.iter() {
        let path = src_base.join("common").join(filename);
        base_config.file(&path);
        println!("cargo:rerun-if-changed={}", path.display());
    }

    if target_os == "windows" {
        base_config.cpp(true);
        base_config.include(src_base.join("windows"));

        for filename in INCLUDE_WINDOWS.iter() {
            let path = src_base.join("windows").join(filename);
            base_config.file(&path);
            println!("cargo:rerun-if-changed={}", path.display());
        }

        // See https://github.com/nabijaczleweli/rust-embed-resource/issues/11
        let target = var("TARGET").unwrap();
        if let Some(tool) = cc::windows_registry::find_tool(target.as_str(), "cl.exe") {
            for (key, value) in tool.env() {
                unsafe { set_var(key, value) };
            }
        }
        let _ = embed_resource::compile(
            src_base.join("windows").join("resources.rc"),
            embed_resource::NONE,
        );

        link("user32", false);
        link("kernel32", false);
        link("gdi32", false);
        link("comctl32", false);
        link("uxtheme", false);
        link("msimg32", false);
        link("comdlg32", false);
        link("d2d1", false);
        link("dwrite", false);
        link("ole32", false);
        link("oleaut32", false);
        link("oleacc", false);
        link("uuid", false);
        link("windowscodecs", false);
    } else if unix {
        base_config.include(src_base.join("unix"));

        let pkg_cfg = pkg_config::Config::new().probe("gtk+-3.0").unwrap();
        for inc in pkg_cfg.include_paths {
            base_config.include(inc);
        }

        for filename in INCLUDE_UNIX.iter() {
            let path = src_base.join("unix").join(filename);
            base_config.file(&path);
            println!("cargo:rerun-if-changed={}", path.display());
        }
    } else if apple {
        base_config.include(src_base.join("darwin"));

        // https://github.com/sbmpost/AutoRaise/issues/69
        // https://youtrack.jetbrains.com/issue/KT-48807
        // "In Xcode 13 RC localizedAttributedStringForKey:value:table: method got NS_FORMAT_ARGUMENT(1) macro attribute.
        // Previously, this attribute was applicable only to functions that return a C string, CFString or NSString.
        // Recently, Apple added support in Clang for another type, NSAttributedString. Clang that ships with Kotlin/Native
        // does not have this patch and fails to process localizedAttributedStringForKey:value:table: declaration.
        // What this workaround does is it makes NS_FORMAT_ARGUMENT(1) a no-op."
        base_config.flag("-DNS_FORMAT_ARGUMENT(A)=");

        // libui-ng uses this flag. I found it to cause a linker error on MaCOS 11.6.
        // Undefined symbol "___isPlatformVersionAtLeast", caused by the @available attribute when mixing XCode/CLTools versions.
        // I'll leave it here in case its different for someone.
        //base_config.flag("-mmacosx-version-min=10.8");

        for filename in INCLUDE_DARWIN.iter() {
            let path = src_base.join("darwin").join(filename);
            base_config.file(&path);
            println!("cargo:rerun-if-changed={}", path.display());
        }
        println!("cargo:rustc-link-lib=framework=AppKit");
    } else {
        panic!("unrecognized platform! cannot build libui from source");
    }

    // Link everything together into `libui.a`.  This will get linked
    // together because of the `links="ui"` flag in the `Cargo.toml` file,
    // and because the `.compile()` function emits
    // `cargo:rustc-link-lib=static=ui`.
    base_config.compile("libui.a");

    Ok(())
}

/// Tell cargo to link the given library, and optionally to bundle it in.
pub fn link(name: &str, bundled: bool) {
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        println!("cargo:rustc-link-lib=dylib={}", name);
        if bundled && target.get(3) == Some(&"gnu") {
            let dir = var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo:rustc-link-search=native={}/{}", dir, target[0]);
        }
    } else {
        println!("cargo:rustc-link-lib=dylib={}", name);
    }
}

/// Add the given framework to the linker path
pub fn link_framework(name: &str) {
    println!("cargo:rustc-link-lib=framework={}", name);
}

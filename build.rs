use {
    cc::Build,
    std::{
        env::{set_var, var},
        path::{Path, PathBuf},
    },
};

//noinspection SpellCheckingInspection
const LIB_WINDOWS: [&str; 14] = [
    "user32",
    "kernel32",
    "gdi32",
    "comctl32",
    "uxtheme",
    "msimg32",
    "comdlg32",
    "d2d1",
    "dwrite",
    "ole32",
    "oleaut32",
    "oleacc",
    "uuid",
    "windowscodecs",
];

const SRC_COMMON: [&str; 13] = [
    "attribute.c",
    "attr_list.c",
    "attr_str.c",
    "area_events.c",
    "control.c",
    "debug.c",
    "matrix.c",
    "opentype.c",
    "should_quit.c",
    "tablemodel.c",
    "tablevalue.c",
    "user_bugs.c",
    "utf.c",
];
const SRC_WINDOWS: [&str; 61] = [
    "alloc.cpp",
    "area.cpp",
    "area_draw.cpp",
    "area_events.cpp",
    "area_scroll.cpp",
    "area_util.cpp",
    "attr_str.cpp",
    "box.cpp",
    "button.cpp",
    "checkbox.cpp",
    "color_button.cpp",
    "color_dialog.cpp",
    "combobox.cpp",
    "container.cpp",
    "control.cpp",
    "d2d_scratch.cpp",
    "datetimepicker.cpp",
    "debug.cpp",
    "draw.cpp",
    "drawmatrix.cpp",
    "draw_path.cpp",
    "drawtext.cpp",
    "d_write.cpp",
    "editable_combo.cpp",
    "entry.cpp",
    "events.cpp",
    "font_button.cpp",
    "font_dialog.cpp",
    "font_match.cpp",
    "form.cpp",
    "graphemes.cpp",
    "grid.cpp",
    "group.cpp",
    "image.cpp",
    "init.cpp",
    "label.cpp",
    "main.cpp",
    "menu.cpp",
    "multi_line_entry.cpp",
    "opentype.cpp",
    "parent.cpp",
    "progressbar.cpp",
    "radiobuttons.cpp",
    "separator.cpp",
    "sizing.cpp",
    "slider.cpp",
    "spinbox.cpp",
    "std_dialogs.cpp",
    "tab.cpp",
    "table.cpp",
    "table_disp_info.cpp",
    "tabledraw.cpp",
    "table_editing.cpp",
    "table_metrics.cpp",
    "tab_page.cpp",
    "text.cpp",
    "utf16.cpp",
    "util_win.cpp",
    "window.cpp",
    "win_public.cpp",
    "win_util.cpp",
];
const SRC_UNIX: [&str; 44] = [
    "alloc.c",
    "area.c",
    "attr_str.c",
    "box.c",
    "button.c",
    "cell_renderer_button.c",
    "checkbox.c",
    "child.c",
    "color_button.c",
    "combobox.c",
    "control.c",
    "datetimepicker.c",
    "debug.c",
    "draw.c",
    "drawmatrix.c",
    "draw_path.c",
    "drawtext.c",
    "editable_combo.c",
    "entry.c",
    "font_button.c",
    "font_match.c",
    "form.c",
    "future.c",
    "graphemes.c",
    "grid.c",
    "group.c",
    "image.c",
    "label.c",
    "main.c",
    "menu.c",
    "multi_line_entry.c",
    "opentype.c",
    "progressbar.c",
    "radiobuttons.c",
    "separator.c",
    "slider.c",
    "spinbox.c",
    "std_dialogs.c",
    "tab.c",
    "table.c",
    "tablemodel.c",
    "text.c",
    "util.c",
    "window.c",
];
const SRC_DARWIN: [&str; 50] = [
    "aat.m",
    "alloc.m",
    "area.m",
    "area_events.m",
    "attr_str.m",
    "autolayout.m",
    "box.m",
    "button.m",
    "checkbox.m",
    "color_button.m",
    "combobox.m",
    "control.m",
    "datetimepicker.m",
    "debug.m",
    "draw.m",
    "drawtext.m",
    "editable_combo.m",
    "entry.m",
    "event.m",
    "font_button.m",
    "font_match.m",
    "font_traits.m",
    "font_variation.m",
    "form.m",
    "future.m",
    "graphemes.m",
    "grid.m",
    "group.m",
    "image.m",
    "label.m",
    "main.m",
    "menu.m",
    "multi_line_entry.m",
    "ns_text_field.m",
    "opentype.m",
    "progressbar.m",
    "radiobuttons.m",
    "scroll_view.m",
    "separator.m",
    "slider.m",
    "spinbox.m",
    "std_dialogs.m",
    "tab.m",
    "table.m",
    "table_column.m",
    "text.m",
    "undocumented.m",
    "util.m",
    "window.m",
    "win_move_resize.m",
];

fn main() -> anyhow::Result<()> {
    // Determine build platform
    let target_os = var("CARGO_CFG_TARGET_OS")?;
    let target_triple = var("TARGET")?;
    let apple = target_triple.contains("apple");
    let unix = cfg!(target_family = "unix") && !apple;
    let out_dir = var("OUT_DIR")?;
    let mut src_base = Path::new("src").join("raw");

    // Generate system-ui bindings on the fly
    let bindings = bindgen::Builder::default()
        .header(src_base.join("ui.h").to_string_lossy())
        .opaque_type("max_align_t") // For some reason this ends up too large
        .clang_args(["-target", &target_triple])
        .trust_clang_mangling(false) // clang sometimes wants to treat these functions as C++
        .generate()?;

    let out_path = Path::new(&out_dir);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    let mut base_config = Build::new();

    // system-ui might emit lots of warning we can do nothing about here
    base_config.warnings(false);

    // Add source files that are common to all platforms
    base_config.include(src_base.join("common"));

    for filename in SRC_COMMON.iter() {
        let path = src_base.join("common").join(filename);
        base_config.file(&path);
        println!("cargo:rerun-if-changed={}", path.display());
    }

    if target_os == "windows" {
        config_windows(&mut base_config, &mut src_base)?;
    } else if unix {
        config_unix(&mut base_config, &mut src_base)?;
    } else if apple {
        config_apple(&mut base_config, &mut src_base)?;
    } else {
        panic!("unrecognized platform! cannot build system-ui from source");
    }

    base_config.compile("ui.a");

    Ok(())
}

fn config_apple(base_config: &mut Build, src_path: &mut PathBuf) -> anyhow::Result<()> {
    base_config.include(src_path.join("darwin"));

    // https://github.com/sbmpost/AutoRaise/issues/69
    // https://youtrack.jetbrains.com/issue/KT-48807
    // "In Xcode 13 RC localizedAttributedStringForKey:value:table: method got NS_FORMAT_ARGUMENT(1) macro attribute.
    // Previously, this attribute was applicable only to functions that return a C string, CFString or NSString.
    // Recently, Apple added support in Clang for another type, NSAttributedString. Clang that ships with Kotlin/Native
    // does not have this patch and fails to process localizedAttributedStringForKey:value:table: declaration.
    // What this workaround does is it makes NS_FORMAT_ARGUMENT(1) a no-op."
    base_config.flag("-DNS_FORMAT_ARGUMENT(A)=");

    for filename in SRC_DARWIN.iter() {
        let path = src_path.join("darwin").join(filename);
        base_config.file(&path);
        println!("cargo:rerun-if-changed={}", path.display());
    }
    println!("cargo:rustc-link-lib=framework=AppKit");

    Ok(())
}

fn config_windows(base_config: &mut Build, src_path: &mut PathBuf) -> anyhow::Result<()> {
    base_config.cpp(true);
    base_config.include(src_path.join("windows"));

    for filename in SRC_WINDOWS.iter() {
        let path = src_path.join("windows").join(filename);
        base_config.file(&path);
        println!("cargo:rerun-if-changed={}", path.display());
    }

    // See https://github.com/nabijaczleweli/rust-embed-resource/issues/11
    let target = var("TARGET")?;
    if let Some(tool) = cc::windows_registry::find_tool(target.as_str(), "cl.exe") {
        for (key, value) in tool.env() {
            unsafe { set_var(key, value) };
        }
    }
    let _ = embed_resource::compile(
        src_path.join("windows").join("resources.rc"),
        embed_resource::NONE,
    );

    for lib in LIB_WINDOWS.iter() {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    Ok(())
}

fn config_unix(base_config: &mut Build, src_path: &mut PathBuf) -> anyhow::Result<()> {
    base_config.include(src_path.join("unix"));

    let pkg_cfg = pkg_config::Config::new().probe("gtk+-3.0")?;
    for inc in pkg_cfg.include_paths {
        base_config.include(inc);
    }

    for filename in SRC_UNIX.iter() {
        let path = src_path.join("unix").join(filename);
        base_config.file(&path);
        println!("cargo:rerun-if-changed={}", path.display());
    }

    Ok(())
}

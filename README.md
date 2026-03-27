# egui-cjk-font

[![Crates.io](https://img.shields.io/crates/v/egui-cjk-font)](https://crates.io/crates/egui-cjk-font)
[![License](https://img.shields.io/crates/l/egui-cjk-font)](https://crates.io/crates/egui-cjk-font)

A lightweight Rust crate that automatically loads system CJK fonts for egui/eframe applications.

## Features

- **Two APIs**: `load_cjk_font` for simple setup, `merge_cjk_font` to preserve custom fonts
- **Cross-platform**: Windows, macOS, Linux support
- **Smart detection**: POSIX / BCP-47 locale parsing (`zh_CN.UTF-8`, `zh_TW@calendar`, `zh-Hans-CN`, etc.)
- **Native fonts first**: Prioritizes platform UI fonts for best visual consistency
- **Safe**: Idempotent calls, `merge_cjk_font` never removes user-defined fonts

## Installation

```toml
# Cargo.toml
[dependencies]
egui-cjk-font = "0.2"
```

## Quick Start

### Simple API (recommended for most apps)

```rust
use {
    eframe::{CreationContext, egui},
    egui_cjk_font::load_cjk_font,
};

struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.heading("你好，世界！ Hello world! こんにちは 안녕하세요");
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My App",
        options,
        Box::new(|cc| {
            load_cjk_font(&cc.egui_ctx);
            Ok(Box::new(MyApp))
        }),
    )
}
```

### Merge API (preserves custom font configuration)

If your app already configures fonts, use `merge_cjk_font` instead:

```rust
use {
    eframe::egui,
    egui_cjk_font::merge_cjk_font,
};

struct MyApp {
    cjk_loaded: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if !self.cjk_loaded {
            merge_cjk_font(ui.ctx());
            self.cjk_loaded = true;
        }
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.heading("你好，世界！");
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My App",
        options,
        Box::new(|cc| {
            // Custom font setup here
            cc.egui_ctx.set_fonts(egui::FontDefinitions::default());
            Ok(Box::new(MyApp { cjk_loaded: false }))
        }),
    )
}
```

> **Note**: `merge_cjk_font` must be called after the first `Context::run()` (e.g., in `App::ui`).

## Supported Fonts

### Windows

- Microsoft YaHei, Microsoft JhengHei
- Yu Gothic, Meiryo
- Malgun Gothic

### macOS

- PingFang SC, PingFang TC, PingFang HK
- Hiragino Sans
- Apple SD Gothic Neo

### Linux

- Noto Sans CJK
- Source Han Sans
- WenQuanYi Micro Hei
- Sarasa Gothic SC
- Droid Sans Fallback

## Locale Support

| Locale Pattern            | Script | Region     | Font Priority       |
| ------------------------- | ------ | ---------- | ------------------- |
| `zh`, `zh-CN`, `zh-SG`    | -      | CN, SG     | Simplified Chinese  |
| `zh-Hans`, `zh-Hans-CN`   | Hans   | -          | Simplified Chinese  |
| `zh-TW`, `zh-HK`, `zh-MO` | -      | TW, HK, MO | Traditional Chinese |
| `zh-Hant`, `zh-Hant-TW`   | Hant   | -          | Traditional Chinese |
| `ja`, `ja-JP`             | -      | JP         | Japanese            |
| `ko`, `ko-KR`             | -      | KR         | Korean              |

## How It Works

1. Parse system locale into language/script/region components
2. If not CJK (zh/ja/ko), do nothing
3. Select font priority list based on locale
4. Try each font family using font-kit's best-match
5. Apply font:
   - `load_cjk_font`: Creates a new `FontDefinitions`, inserts the CJK font at the front of `Proportional`, and appends it to `Monospace` (replaces existing config)
   - `merge_cjk_font`: Inserts the CJK font at the front of `Proportional`, and appends it to `Monospace` in the existing configuration (preserves custom fonts)

## Notes

- CJK fonts are only loaded when the system locale is a CJK language
- `load_cjk_font`: Simple but replaces existing font configuration; suitable for apps without custom fonts
- `merge_cjk_font`: Preserves existing font configuration, but must be called after the first `Context::run()`
- This crate emits no font-loading logs; if font lookup fails, existing font configuration is silently retained

## Testing

```bash
cargo test
```

## License

MIT OR Apache-2.0

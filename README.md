# egui-cjk-font

[![Crates.io](https://img.shields.io/crates/v/egui-cjk-font)](https://crates.io/crates/egui-cjk-font)
[![License](https://img.shields.io/crates/l/egui-cjk-font)](https://crates.io/crates/egui-cjk-font)

A lightweight Rust crate that automatically loads system CJK fonts for egui/eframe applications.

## Features

- **Two APIs**: [`load_cjk_font`] for simple setup, [`merge_cjk_font`] to preserve custom fonts
- **Cross-platform**: Windows, macOS, Linux support
- **Smart detection**: BCP-47 locale parsing (zh-Hans-CN, zh_Hant_TW, etc.)
- **Native fonts first**: Prioritizes platform UI fonts for best visual consistency
- **Safe**: Idempotent calls, `merge_cjk_font` never removes user-defined fonts

## Installation

```toml
# Cargo.toml
[dependencies]
egui-cjk-font = "0.1"
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
    eframe::{CreationContext, egui},
    egui_cjk_font::merge_cjk_font,
};

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        merge_cjk_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
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
            cc.egui_ctx.set_fonts(my_custom_fonts());
            Ok(Box::new(MyApp))
        }),
    )
}
```

> **Note**: `merge_cjk_font` must be called after the first `Context::run()` (e.g., in `App::update`).

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

| Locale Pattern | Script | Region | Font Priority |
|----------------|--------|--------|---------------|
| `zh`, `zh-CN`, `zh-SG` | - | CN, SG | Simplified Chinese |
| `zh-Hans`, `zh-Hans-CN` | Hans | - | Simplified Chinese |
| `zh-TW`, `zh-HK`, `zh-MO` | - | TW, HK, MO | Traditional Chinese |
| `zh-Hant`, `zh-Hant-TW` | Hant | - | Traditional Chinese |
| `ja`, `ja-JP` | - | JP | Japanese |
| `ko`, `ko-KR` | - | KR | Korean |

## How It Works

1. Parse system locale into language/script/region components
2. If not CJK (zh/ja/ko), do nothing
3. Select font priority list based on locale
4. Try each font family using font-kit's best-match
5. Apply font:
   - `load_cjk_font`: Creates a new `FontDefinitions` with CJK font at highest priority (replaces existing config)
   - `merge_cjk_font`: Inserts CJK font into existing configuration (preserves custom fonts)

## Testing

```bash
cargo test -p egui-cjk-font
```

## License

MIT OR Apache-2.0

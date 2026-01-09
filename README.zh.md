# egui-cjk-font

[![Crates.io](https://img.shields.io/crates/v/egui-cjk-font)](https://crates.io/crates/egui-cjk-font)
[![License](https://img.shields.io/crates/l/egui-cjk-font)](https://crates.io/crates/egui-cjk-font)

一个轻量级 Rust 包，为 egui/eframe 应用自动加载系统 CJK 字体。

## 功能特性

- **两种 API**：[`load_cjk_font`] 简单易用，[`merge_cjk_font`] 保留自定义字体配置
- **跨平台支持**: Windows、macOS、Linux
- **自动检测**: 检测系统语言区域，仅在需要时加载 CJK 字体
- **智能回退**: 优先使用平台原生 UI 字体，多重备选方案
- **安全**: 幂等调用，`merge_cjk_font` 不会移除用户定义的字体

## 安装

```toml
# Cargo.toml
[dependencies]
egui-cjk-font = "0.1"
```

## 快速开始

### 简单 API（推荐大多数应用使用）

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

### 合并 API（保留自定义字体配置）

如果你的应用已经配置了字体，请使用 `merge_cjk_font`：

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
            // 在此处进行自定义字体配置
            cc.egui_ctx.set_fonts(my_custom_fonts());
            Ok(Box::new(MyApp))
        }),
    )
}
```

> **注意**：`merge_cjk_font` 必须在第一次 `Context::run()` 之后调用（例如在 `App::update` 中）。

## 支持的字体

### Windows
- Microsoft YaHei（简体中文）
- Microsoft JhengHei（繁體中文）
- Yu Gothic（日语）
- Meiryo（日语）
- Malgun Gothic（韩语）

### macOS
- PingFang SC/TC/HK
- Hiragino Sans（日语）
- Apple SD Gothic Neo（韩语）

### Linux
- Noto Sans CJK（简体中文/繁體中文/日语/韩语）
- Source Han Sans
- WenQuanYi Micro Hei
- Sarasa Gothic SC
- Droid Sans Fallback

## 语言区域支持

| 语言区域 | 文字 | 区域 |
|----------|------|------|
| `zh`, `zh-CN`, `zh-SG`, `zh-hans` | 简体中文 | 简体 |
| `zh-TW`, `zh-HK`, `zh-MO`, `zh-hant` | 繁體中文 | 繁体 |
| `ja`, `ja-JP` | 日本語 | 日语 |
| `ko`, `ko-KR` | 한국어 | 韩语 |

## 工作原理

1. 通过 `sys-locale` 检测系统语言区域
2. 如果是 CJK 语言（zh/ja/ko），按优先级搜索字体：
   - 平台原生 UI 字体（最佳视觉效果）
   - 高质量开源备选字体
   - 系统无衬线字体作为最后手段
3. 应用字体：
   - `load_cjk_font`：创建新的 `FontDefinitions`，CJK 字体优先级最高（会替换现有配置）
   - `merge_cjk_font`：将 CJK 字体插入现有配置（保留自定义字体）

## 注意事项

- 仅在系统语言区域为 CJK 语言时才加载 CJK 字体
- `load_cjk_font`：简单但会替换现有的字体配置，适合没有自定义字体配置的应用
- `merge_cjk_font`：保留现有字体配置，但必须在首次 `Context::run()` 后调用
- 字体加载失败时会静默回退（使用现有字体或默认配置）
- 如需调试，可在 stderr 查看字体加载日志

## 许可证

MIT OR Apache-2.0

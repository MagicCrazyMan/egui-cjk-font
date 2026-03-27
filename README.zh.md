# egui-cjk-font

[![Crates.io](https://img.shields.io/crates/v/egui-cjk-font)](https://crates.io/crates/egui-cjk-font)
[![License](https://img.shields.io/crates/l/egui-cjk-font)](https://crates.io/crates/egui-cjk-font)

一个轻量级 Rust 包，为 egui/eframe 应用自动加载系统 CJK 字体。

## 功能特性

- **两种 API**：`load_cjk_font` 简单易用，`merge_cjk_font` 保留自定义字体配置
- **跨平台支持**: Windows、macOS、Linux
- **自动检测**: 解析 POSIX / BCP-47 语言区域（`zh_CN.UTF-8`、`zh_TW@calendar`、`zh-Hans-CN` 等），仅在需要时加载 CJK 字体
- **智能回退**: 优先使用平台原生 UI 字体，再回退到通用 CJK 字体族
- **安全**: 幂等调用，`merge_cjk_font` 不会移除用户定义的字体

## 安装

```toml
# Cargo.toml
[dependencies]
egui-cjk-font = "0.2"
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

### 合并 API（保留自定义字体配置）

如果你的应用已经配置了字体，请使用 `merge_cjk_font`：

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
            // 在此处进行自定义字体配置
            cc.egui_ctx.set_fonts(egui::FontDefinitions::default());
            Ok(Box::new(MyApp { cjk_loaded: false }))
        }),
    )
}
```

> **注意**：`merge_cjk_font` 必须在第一次 `Context::run()` 之后调用（例如在 `App::ui` 中）。

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

| 语言区域模式              | 脚本 | 区域       | 字体优先级 |
| ------------------------- | ---- | ---------- | ---------- |
| `zh`, `zh-CN`, `zh-SG`    | -    | CN, SG     | 简体中文   |
| `zh-Hans`, `zh-Hans-CN`   | Hans | -          | 简体中文   |
| `zh-TW`, `zh-HK`, `zh-MO` | -    | TW, HK, MO | 繁體中文   |
| `zh-Hant`, `zh-Hant-TW`   | Hant | -          | 繁體中文   |
| `ja`, `ja-JP`             | -    | JP         | 日语       |
| `ko`, `ko-KR`             | -    | KR         | 韩语       |

## 工作原理

1. 解析系统语言区域为 language / script / region 组件
2. 如果不是 CJK 语言（zh/ja/ko），直接跳过
3. 根据 locale 选择字体优先级列表
4. 使用 `font-kit` 的 best-match 逐个尝试字体族
5. 应用字体：
   - `load_cjk_font`：创建新的 `FontDefinitions`，将 CJK 字体插入 `Proportional` 首位，并追加到 `Monospace` 末尾（会替换现有配置）
   - `merge_cjk_font`：将 CJK 字体插入现有配置的 `Proportional` 首位，并追加到 `Monospace` 末尾（保留自定义字体）

## 注意事项

- 仅在系统语言区域为 CJK 语言时才加载 CJK 字体
- `load_cjk_font`：简单但会替换现有的字体配置，适合没有自定义字体配置的应用
- `merge_cjk_font`：保留现有字体配置，但必须在首次 `Context::run()` 后调用
- 当前 crate 不输出字体加载日志；字体查找失败时会静默保持现有字体配置

## 测试

```bash
cargo test
```

## 许可证

MIT OR Apache-2.0

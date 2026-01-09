//! egui-cjk-font
//!
//! A lightweight utility crate that loads a suitable system CJK font
//! for egui / eframe applications on Windows, macOS, and Linux.
//!
//! ## Design goals
//!
//! - Automatically selects native platform UI fonts
//! - Activates only for CJK locales (zh / ja / ko)
//! - Avoids egui initialization-order pitfalls
//!
//! ## Two usage modes
//!
//! This crate provides two APIs with different guarantees:
//!
//! - [`load_cjk_font`]: Safe to call at any time (including before `Context::run()`), but **does
//!   not merge** with existing font configuration.
//!
//! - [`merge_cjk_font`]: Merges a CJK font into the existing egui font configuration, but **must be
//!   called after the first `Context::run()`**.
//!
//! ## Important notes
//!
//! Due to egui API constraints, it is not possible to safely inspect
//! existing font configuration before the first frame is rendered.
//! As a result, merging fonts before `Context::run()` is fundamentally
//! unsupported by egui.
//!
//! Choose the API that best matches your application's initialization flow.

use {
    egui::{Context, FontData, FontDefinitions, FontFamily},
    font_kit::{
        family_name::FamilyName,
        handle::Handle,
        properties::Properties,
        source::SystemSource,
    },
    std::{fs::File, io::Read, sync::Arc},
    sys_locale::get_locale,
};

const CJK_FONT_NAME: &str = "egui_cjk_font";

/// Parsed BCP-47–like locale info
#[derive(Debug, Clone)]
struct LocaleInfo {
    lang: String,
    script: Option<String>,
    region: Option<String>,
}

fn current_locale() -> LocaleInfo {
    parse_locale(&get_locale().unwrap_or_else(|| "en-us".to_string()))
}

/// Parse locale string into language / script / region.
///
/// Examples:
/// - zh_CN.UTF-8 -> zh / None / cn
/// - zh-Hans-CN  -> zh / hans / cn
/// - ja_JP      -> ja / None / jp
fn parse_locale(raw: &str) -> LocaleInfo {
    let raw = raw
        .split_once('.')
        .map(|(l, _)| l)
        .unwrap_or(raw)
        .to_lowercase();

    let parts: Vec<&str> = raw.split(&['-', '_'][..]).collect();

    match parts.len() {
        1 => LocaleInfo {
            lang: parts[0].to_string(),
            script: None,
            region: None,
        },
        2 => LocaleInfo {
            lang: parts[0].to_string(),
            script: None,
            region: Some(parts[1].to_string()),
        },
        _ => LocaleInfo {
            lang: parts[0].to_string(),
            script: Some(parts[1].to_string()),
            region: Some(parts[2].to_string()),
        },
    }
}

fn is_cjk(locale: &LocaleInfo) -> bool { matches!(locale.lang.as_str(), "zh" | "ja" | "ko") }

/// Load a suitable system CJK font into the egui context.
///
/// This function is safe to call before or after `egui::Context::run()`.
/// It does nothing for non-CJK locales.
///
/// The font is installed using a fresh `FontDefinitions` based on egui
/// defaults, with the CJK font inserted at high priority. Existing font
/// configuration is not inspected or merged.
///
/// If your application customizes fonts manually, consider using
/// [`merge_cjk_font`] instead.
pub fn load_cjk_font(ctx: &Context) {
    let locale = current_locale();
    if !is_cjk(&locale) {
        return;
    }

    if let Some(font_data) = find_cjk_font(&locale) {
        apply_font(ctx, font_data);
    }
}

/// Merge a suitable system CJK font into the existing egui font configuration.
///
/// ## Contract
///
/// This function must be called **after the first `egui::Context::run()`**.
/// Calling it earlier violates egui's initialization contract and may panic.
///
/// ## Behavior
///
/// - Reads existing font definitions from egui
/// - Inserts a CJK font without removing user-defined fonts
/// - Safe to call multiple times (idempotent)
///
/// ## Typical usage
///
/// Call this from `App::update` during the first frame, or any time after
/// egui has started rendering.
pub fn merge_cjk_font(ctx: &Context) {
    let locale = current_locale();
    if !is_cjk(&locale) {
        return;
    }

    let Some(font_data) = find_cjk_font(&locale) else {
        return;
    };

    ctx.fonts(|f| {
        let mut defs = f.definitions().clone();

        if defs.font_data.contains_key(CJK_FONT_NAME) {
            return;
        }

        defs.font_data.insert(
            CJK_FONT_NAME.to_string(),
            Arc::new(FontData::from_owned(font_data)),
        );

        if let Some(family) = defs.families.get_mut(&FontFamily::Proportional) {
            family.insert(0, CJK_FONT_NAME.to_string());
        }

        if let Some(family) = defs.families.get_mut(&FontFamily::Monospace) {
            family.push(CJK_FONT_NAME.to_string());
        }

        ctx.set_fonts(defs);
    });
}

/// Find the most suitable CJK font based on locale.
pub(crate) fn find_cjk_font(locale: &LocaleInfo) -> Option<Vec<u8>> {
    let source = SystemSource::new();

    let priorities: &[&str] = match locale.lang.as_str() {
        "zh" => match (locale.script.as_deref(), locale.region.as_deref()) {
            (Some("hant"), _) | (_, Some("tw" | "hk" | "mo")) => &[
                "Microsoft JhengHei",
                "PingFang TC",
                "PingFang HK",
                "Noto Sans CJK TC",
                "Source Han Sans TC",
            ],
            _ => &[
                "Microsoft YaHei",
                "PingFang SC",
                "Noto Sans CJK SC",
                "Source Han Sans SC",
            ],
        },
        "ja" => &["Yu Gothic", "Meiryo", "Hiragino Sans", "Noto Sans CJK JP"],
        "ko" => &["Malgun Gothic", "Apple SD Gothic Neo", "Noto Sans CJK KR"],
        _ => &[],
    };

    for name in priorities {
        if let Some(data) = load_by_family_name(&source, name) {
            return Some(data);
        }
    }

    for name in [
        "Noto Sans CJK",
        "Source Han Sans",
        "WenQuanYi Micro Hei",
        "Sarasa Gothic SC",
        "Droid Sans Fallback",
    ] {
        if let Some(data) = load_by_family_name(&source, name) {
            return Some(data);
        }
    }

    None
}

pub(crate) fn load_by_family_name(source: &SystemSource, name: &str) -> Option<Vec<u8>> {
    source
        .select_best_match(&[FamilyName::Title(name.to_string())], &Properties::new())
        .ok()
        .and_then(load_handle)
}

pub(crate) fn load_handle(handle: Handle) -> Option<Vec<u8>> {
    match handle {
        Handle::Memory { bytes, .. } => Some(bytes.to_vec()),
        Handle::Path { path, .. } => {
            let mut buf = Vec::new();
            File::open(path).ok()?.read_to_end(&mut buf).ok()?;
            Some(buf)
        }
    }
}

pub(crate) fn apply_font(ctx: &Context, font_data: Vec<u8>) {
    if font_data.is_empty() {
        return;
    }

    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        CJK_FONT_NAME.to_string(),
        Arc::new(FontData::from_owned(font_data)),
    );

    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.insert(0, CJK_FONT_NAME.to_string());
    }

    if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
        family.push(CJK_FONT_NAME.to_string());
    }

    ctx.set_fonts(fonts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_locale_variants() {
        let l = parse_locale("zh-Hans-CN.UTF-8");
        assert_eq!(l.lang, "zh");
        assert_eq!(l.script.as_deref(), Some("hans"));
        assert_eq!(l.region.as_deref(), Some("cn"));

        let l = parse_locale("ja_JP");
        assert_eq!(l.lang, "ja");
        assert_eq!(l.script.as_deref(), None);
        assert_eq!(l.region.as_deref(), Some("jp"));

        let l = parse_locale("zh_TW");
        assert_eq!(l.lang, "zh");
        assert_eq!(l.script.as_deref(), None);
        assert_eq!(l.region.as_deref(), Some("tw"));
    }

    #[test]
    fn non_cjk_locale_detection() {
        let l = parse_locale("en-US");
        assert!(!is_cjk(&l));
    }

    #[test]
    fn apply_font_no_panic_with_empty_data() { apply_font(&Context::default(), Vec::new()) }

    #[test]
    fn is_cjk_detection() {
        assert!(is_cjk(&parse_locale("zh-CN")));
        assert!(is_cjk(&parse_locale("ja-JP")));
        assert!(is_cjk(&parse_locale("ko-KR")));
        assert!(!is_cjk(&parse_locale("en-US")));
        assert!(!is_cjk(&parse_locale("de-DE")));
    }
}

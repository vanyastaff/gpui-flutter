# Иконки (Lucide) и Accessibility в gpui-flutter (Updated)

Дата: 2026-01-19

---

## 1. Система иконок (Lucide) - Использование готовых крейтов

### Доступные Rust крейты для Lucide

#### Рекомендуемые крейты:

**1. [lucide-icons](https://crates.io/crates/lucide-icons)** (Рекомендуется)
- ✅ Самый популярный (47,354+ загрузок)
- ✅ Актуальная версия 0.555.0
- ✅ Автоматически обновляется при релизах Lucide
- ✅ Просто Rust enum definitions
- ✅ MIT/ISC лицензия

**2. [RustForWeb/lucide](https://github.com/RustForWeb/lucide)**
- ✅ Поддержка Dioxus, Leptos, Yew
- ✅ Компоненты для web frameworks
- ⚠️ Может быть избыточным для GPUI

**3. [lucide-svg-rs](https://github.com/crabtools-rs/lucide-svg-rs)**
- ✅ CLI инструмент для скачивания SVG
- ✅ Offline доступ к SVG файлам
- ⚠️ Больше подходит для dev tools

### Интеграция с GPUI

#### Cargo.toml

```toml
[dependencies]
lucide-icons = "0.555" # или последняя версия
gpui = "0.1"
```

#### Wrapper для GPUI

```rust
// crates/ui/src/icons/mod.rs

use gpui::*;
pub use lucide_icons::Icon as LucideIcon; // Re-export

#[derive(IntoElement, Clone)]
pub struct Icon {
    icon: LucideIcon,
    size: IconSize,
    color: Option<Hsla>,
    stroke_width: f32,
    
    // Accessibility
    label: Option<SharedString>,
    decorative: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    Xs,  // 12px
    Sm,  // 16px
    Md,  // 20px
    Lg,  // 24px (default)
    Xl,  // 32px
    Xxl, // 48px
}

impl IconSize {
    pub fn to_pixels(&self) -> Pixels {
        match self {
            IconSize::Xs => px(12.),
            IconSize::Sm => px(16.),
            IconSize::Md => px(20.),
            IconSize::Lg => px(24.),
            IconSize::Xl => px(32.),
            IconSize::Xxl => px(48.),
        }
    }
}

impl Icon {
    pub fn new(icon: LucideIcon) -> Self {
        Self {
            icon,
            size: IconSize::Lg,
            color: None,
            stroke_width: 2.0,
            label: None,
            decorative: false,
        }
    }
    
    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
    
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }
    
    // Accessibility
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self.decorative = false;
        self
    }
    
    pub fn decorative(mut self) -> Self {
        self.decorative = true;
        self.label = None;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        let color = self.color.unwrap_or(theme.colors.foreground);
        let size = self.size.to_pixels();
        
        // Получить SVG path data из lucide-icons
        let svg_data = self.icon.svg_path_data();
        
        // Рендерить SVG в GPUI
        svg()
            .size(size)
            .path(svg_data)
            .text_color(color)
            .flex_none() // prevent stretching
            // Accessibility
            .when(!self.decorative, |svg| {
                svg.aria_label(
                    self.label.unwrap_or_else(|| {
                        // Default: название иконки
                        self.icon.name().replace('-', " ").into()
                    })
                )
            })
            .when(self.decorative, |svg| {
                svg.aria_hidden(true)
            })
    }
}
```

### Использование с lucide-icons крейтом

```rust
use gpui_flutter::prelude::*;
use lucide_icons::Icon as LucideIcon;

// Все иконки доступны как варианты enum
Icon::new(LucideIcon::Search)
    .size(IconSize::Md)
    .label("Search")

Icon::new(LucideIcon::Heart)
    .color(theme.colors.destructive)
    .decorative()

Icon::new(LucideIcon::Settings)
    .size(IconSize::Lg)
    .stroke_width(3.0)

// В компонентах
Button::new("delete")
    .leading_icon(Icon::new(LucideIcon::Trash).decorative())
    .label("Delete")
    .destructive()

TextField::new("email")
    .label("Email")
    .leading_icon(Icon::new(LucideIcon::Mail).decorative())
    .placeholder("Enter your email")
```

### Доступные иконки из lucide-icons крейта

Крейт предоставляет все 1000+ иконок Lucide:

```rust
// Базовые
LucideIcon::X
LucideIcon::Check
LucideIcon::ChevronDown
LucideIcon::ChevronUp
LucideIcon::ChevronLeft
LucideIcon::ChevronRight

// Интерфейс
LucideIcon::Search
LucideIcon::Menu
LucideIcon::Settings
LucideIcon::User
LucideIcon::Home
LucideIcon::Bell
LucideIcon::Mail
LucideIcon::Phone

// Действия
LucideIcon::Plus
LucideIcon::Minus
LucideIcon::Edit
LucideIcon::Trash
LucideIcon::Copy
LucideIcon::Download
LucideIcon::Upload
LucideIcon::Share
LucideIcon::Save

// Статусы
LucideIcon::Info
LucideIcon::AlertCircle
LucideIcon::AlertTriangle
LucideIcon::CheckCircle
LucideIcon::XCircle
LucideIcon::HelpCircle

// Медиа
LucideIcon::Play
LucideIcon::Pause
LucideIcon::Stop
LucideIcon::SkipForward
LucideIcon::SkipBack
LucideIcon::Volume
LucideIcon::VolumeX

// Файлы
LucideIcon::File
LucideIcon::FileText
LucideIcon::Folder
LucideIcon::FolderOpen
LucideIcon::Image
LucideIcon::Video

// Навигация
LucideIcon::ArrowLeft
LucideIcon::ArrowRight
LucideIcon::ArrowUp
LucideIcon::ArrowDown
LucideIcon::ExternalLink

// Соцсети
LucideIcon::Github
LucideIcon::Twitter
LucideIcon::Facebook
LucideIcon::Linkedin
LucideIcon::Youtube

// ... и многие другие
```

### Кастомизация иконок

Если нужны дополнительные кастомные иконки:

```rust
// crates/ui/src/icons/custom.rs

pub enum CustomIcon {
    MyCustomIcon,
    AnotherIcon,
}

impl CustomIcon {
    pub fn svg_data(&self) -> &'static str {
        match self {
            CustomIcon::MyCustomIcon => include_str!("../assets/icons/custom/my-icon.svg"),
            CustomIcon::AnotherIcon => include_str!("../assets/icons/custom/another.svg"),
        }
    }
}

// Unified Icon enum
pub enum IconVariant {
    Lucide(LucideIcon),
    Custom(CustomIcon),
}

impl Icon {
    pub fn lucide(icon: LucideIcon) -> Self {
        Icon::new(IconVariant::Lucide(icon))
    }
    
    pub fn custom(icon: CustomIcon) -> Self {
        Icon::new(IconVariant::Custom(icon))
    }
}
```

---

## 2. Accessibility (без изменений)

### Принципы ARIA

**Основное правило:** "No ARIA is better than bad ARIA"

1. **Semantic HTML First** - используйте семантические элементы
2. **ARIA только когда необходимо** - добавляйте ARIA только если HTML не может решить задачу
3. **Не дублируйте роли** - не добавляйте `role="button"` к `<button>`
4. **Keyboard accessibility** - все интерактивные элементы доступны с клавиатуры

### Accessibility Context

```rust
// crates/ui/src/accessibility/mod.rs

use gpui::*;

#[derive(Clone, Default)]
pub struct AccessibilityProps {
    pub role: Option<AriaRole>,
    pub label: Option<SharedString>,
    pub description: Option<SharedString>,
    pub hidden: bool,
    pub live: Option<AriaLive>,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub checked: Option<AriaChecked>,
    pub disabled: bool,
    pub required: bool,
    pub invalid: bool,
    pub readonly: bool,
    pub autocomplete: Option<AriaAutocomplete>,
    pub haspopup: Option<AriaHasPopup>,
    pub controls: Option<ElementId>,
    pub labelledby: Option<ElementId>,
    pub describedby: Option<ElementId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaRole {
    Button,
    Link,
    Checkbox,
    Radio,
    Textbox,
    Combobox,
    Listbox,
    Option,
    Menu,
    MenuItem,
    MenuBar,
    Tab,
    TabList,
    TabPanel,
    Dialog,
    AlertDialog,
    Alert,
    Status,
    Navigation,
    Main,
    Complementary,
    Banner,
    ContentInfo,
    Search,
    Form,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaLive {
    Off,
    Polite,
    Assertive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaChecked {
    True,
    False,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaAutocomplete {
    Inline,
    List,
    Both,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaHasPopup {
    True,
    Menu,
    Listbox,
    Tree,
    Grid,
    Dialog,
}

// Trait для применения accessibility
pub trait Accessible: Styled + Sized {
    fn accessibility(self, props: AccessibilityProps) -> Self;
    
    fn aria_role(self, role: AriaRole) -> Self {
        let mut props = AccessibilityProps::default();
        props.role = Some(role);
        self.accessibility(props)
    }
    
    fn aria_label(self, label: impl Into<SharedString>) -> Self {
        let mut props = AccessibilityProps::default();
        props.label = Some(label.into());
        self.accessibility(props)
    }
    
    fn aria_hidden(self, hidden: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.hidden = hidden;
        self.accessibility(props)
    }
    
    fn aria_live(self, live: AriaLive) -> Self {
        let mut props = AccessibilityProps::default();
        props.live = Some(live);
        self.accessibility(props)
    }
    
    fn aria_expanded(self, expanded: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.expanded = Some(expanded);
        self.accessibility(props)
    }
    
    fn aria_selected(self, selected: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.selected = Some(selected);
        self.accessibility(props)
    }
    
    fn aria_checked(self, checked: AriaChecked) -> Self {
        let mut props = AccessibilityProps::default();
        props.checked = Some(checked);
        self.accessibility(props)
    }
    
    fn aria_disabled(self, disabled: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.disabled = disabled;
        self.accessibility(props)
    }
    
    fn aria_required(self, required: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.required = required;
        self.accessibility(props)
    }
    
    fn aria_invalid(self, invalid: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.invalid = invalid;
        self.accessibility(props)
    }
    
    fn aria_controls(self, id: impl Into<ElementId>) -> Self {
        let mut props = AccessibilityProps::default();
        props.controls = Some(id.into());
        self.accessibility(props)
    }
    
    fn aria_labelledby(self, id: impl Into<ElementId>) -> Self {
        let mut props = AccessibilityProps::default();
        props.labelledby = Some(id.into());
        self.accessibility(props)
    }
    
    fn aria_describedby(self, id: impl Into<ElementId>) -> Self {
        let mut props = AccessibilityProps::default();
        props.describedby = Some(id.into());
        self.accessibility(props)
    }
}
```

### Примеры компонентов с встроенным accessibility

#### Button

```rust
impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .id(self.id)
            // Accessibility (автоматически)
            .aria_role(AriaRole::Button)
            .aria_label(self.aria_label.unwrap_or(self.label.clone()))
            .aria_disabled(self.disabled)
            .when_some(self.pressed, |div, pressed| {
                div.aria_pressed(pressed)
            })
            // Keyboard support
            .on_key_down(Key::Enter, move |event, cx| {
                if !self.disabled {
                    if let Some(handler) = &self.on_click {
                        handler(event, cx);
                    }
                }
            })
            .on_key_down(Key::Space, move |event, cx| {
                if !self.disabled {
                    if let Some(handler) = &self.on_click {
                        handler(event, cx);
                    }
                }
            })
            // Focus management
            .focusable()
            .when(self.focused, |div| {
                div.outline(theme.colors.ring)
                   .outline_width(px(2.))
                   .outline_offset(px(2.))
            })
            // ... styling
            .child(self.label)
    }
}
```

#### TextField

```rust
impl RenderOnce for TextField {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        let input_id = self.id.clone();
        let label_id = ElementId::from(format!("{}-label", input_id));
        let helper_id = ElementId::from(format!("{}-helper", input_id));
        let error_id = ElementId::from(format!("{}-error", input_id));
        
        Column::new()
            .gap(theme.spacing.xs)
            // Label
            .when_some(self.label.clone(), |col, label_text| {
                col.child(
                    div()
                        .id(label_id.clone())
                        .child(label_text)
                        .when(self.required, |div| {
                            div.child(Text::new("*").color(theme.colors.destructive))
                        })
                )
            })
            // Input
            .child(
                div()
                    .id(input_id)
                    .aria_role(AriaRole::Textbox)
                    .aria_labelledby(label_id)
                    .when(self.helper_text.is_some(), |div| {
                        div.aria_describedby(helper_id.clone())
                    })
                    .when(self.error_text.is_some(), |div| {
                        div.aria_describedby(error_id.clone())
                            .aria_invalid(true)
                    })
                    .aria_required(self.required)
                    .aria_disabled(self.disabled)
                    .aria_readonly(self.readonly)
                    .focusable()
                    // ... implementation
            )
            // Helper text
            .when_some(self.helper_text, |col, helper| {
                col.child(
                    div()
                        .id(helper_id)
                        .text_size(theme.typography.size_xs)
                        .text_color(theme.colors.muted_foreground)
                        .child(helper)
                )
            })
            // Error text
            .when_some(self.error_text, |col, error| {
                col.child(
                    div()
                        .id(error_id)
                        .aria_live(AriaLive::Polite)
                        .text_size(theme.typography.size_xs)
                        .text_color(theme.colors.destructive)
                        .child(error)
                )
            })
    }
}
```

---

## 3. Практические примеры

### Button с иконкой

```rust
use lucide_icons::Icon as LucideIcon;

Button::new("save")
    .label("Save Changes")
    .leading_icon(
        Icon::new(LucideIcon::Save)
            .decorative() // иконка декоративная, label уже есть
    )
    .variant(ButtonVariant::Primary)
    .on_click(|_, cx| {
        // Save logic
    })
```

### IconButton (только иконка)

```rust
IconButton::new("delete")
    .icon(Icon::new(LucideIcon::Trash))
    .label("Delete item") // ВАЖНО: для screen readers
    .variant(IconButtonVariant::Destructive)
    .on_click(|_, cx| {
        // Delete logic
    })
```

### Search Field

```rust
TextField::new("search")
    .placeholder("Search...")
    .leading_icon(
        Icon::new(LucideIcon::Search)
            .decorative()
    )
    .trailing_icon(
        Icon::new(LucideIcon::X)
            .label("Clear search")
    )
    .on_change(|value, cx| {
        // Search logic
    })
```

### Alert with icon

```rust
Alert::new()
    .variant(AlertVariant::Warning)
    .icon(Icon::new(LucideIcon::AlertTriangle).decorative())
    .title("Warning")
    .description("This action cannot be undone.")
```

### Navigation menu

```rust
Column::new()
    .gap(4.)
    .child(
        Button::new("home")
            .label("Home")
            .leading_icon(Icon::new(LucideIcon::Home).decorative())
            .variant(ButtonVariant::Ghost)
    )
    .child(
        Button::new("search")
            .label("Search")
            .leading_icon(Icon::new(LucideIcon::Search).decorative())
            .variant(ButtonVariant::Ghost)
    )
    .child(
        Button::new("settings")
            .label("Settings")
            .leading_icon(Icon::new(LucideIcon::Settings).decorative())
            .variant(ButtonVariant::Ghost)
    )
```

---

## 4. Обновленная структура проекта

```
gpui-flutter/
├── Cargo.toml
├── crates/
│   ├── ui/
│   │   ├── Cargo.toml  # lucide-icons = "0.555"
│   │   ├── src/
│   │   │   ├── icons/
│   │   │   │   ├── mod.rs           # Icon wrapper component
│   │   │   │   └── custom.rs        # Кастомные иконки (опционально)
│   │   │   ├── accessibility/
│   │   │   │   ├── mod.rs           # Accessibility traits & types
│   │   │   │   ├── keyboard.rs      # Keyboard navigation
│   │   │   │   └── focus.rs         # Focus management
│   │   │   ├── theme/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── registry.rs
│   │   │   │   └── colors.rs
│   │   │   ├── components/
│   │   │   │   ├── button.rs
│   │   │   │   ├── text_field.rs
│   │   │   │   ├── card.rs
│   │   │   │   └── ...
│   │   │   └── lib.rs
│   │   └── themes/
│   │       ├── default-light.json
│   │       └── default-dark.json
│   └── examples/
│       └── gallery/
│           ├── src/
│           │   └── main.rs          # Story gallery для компонентов
│           └── Cargo.toml
└── docs/
    └── research/
        ├── gpui-component-architecture.md
        ├── flutter-style-api.md
        └── icons-and-accessibility-updated.md
```

---

## Источники

### Lucide Icons Rust Crates
- [lucide-icons на crates.io](https://crates.io/crates/lucide-icons) - Рекомендуемый крейт
- [lucide-icons-rs на GitHub](https://github.com/WhySoBad/lucide-icons-rs)
- [RustForWeb/lucide](https://github.com/RustForWeb/lucide) - Для web frameworks
- [lucide-svg-rs](https://github.com/crabtools-rs/lucide-svg-rs) - CLI tool
- [lucide-rust](https://github.com/codegress-com/lucide-rust) - Multi-framework support

### Accessibility
- [ARIA - Accessibility | MDN](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA)
- [WAI-ARIA Overview | W3C](https://www.w3.org/WAI/standards-guidelines/aria/)
- [ARIA Authoring Practices Guide | W3C](https://www.w3.org/WAI/ARIA/apg/)
- [Modern Frontend Accessibility 2026](https://medium.com/design-bootcamp/modern-frontend-accessibility-a-2026-developers-guide-b2de10d01d22)

---

**Конец документа**

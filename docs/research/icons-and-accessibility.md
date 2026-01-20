# Иконки (Lucide) и Accessibility в gpui-flutter

Дата: 2026-01-19

---

## 1. Система иконок (Lucide)

### Обзор Lucide

**Lucide** - это beautiful & consistent icon toolkit, форк Feather Icons с открытым исходным кодом.

**Преимущества Lucide:**
- 🎨 Консистентный визуальный стиль (24x24px grid)
- 📦 Маленький размер бандла
- 🔧 Полностью кастомизируемые (stroke-width, color, size)
- ✅ Более 1000+ иконок
- 🚀 Отличная производительность

**Официальный сайт:** https://lucide.dev

### Архитектура интеграции

#### Вариант 1: SVG как строки (Рекомендуется для Rust)

Встраивать SVG данные напрямую в Rust код:

```rust
// crates/ui/src/icons/lucide.rs

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub type IconSvg = &'static str;

// Генерируется автоматически из lucide icons
static ICONS: Lazy<HashMap<&'static str, IconSvg>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Базовые иконки
    map.insert("x", include_str!("../assets/icons/lucide/x.svg"));
    map.insert("check", include_str!("../assets/icons/lucide/check.svg"));
    map.insert("chevron-down", include_str!("../assets/icons/lucide/chevron-down.svg"));
    map.insert("search", include_str!("../assets/icons/lucide/search.svg"));
    map.insert("menu", include_str!("../assets/icons/lucide/menu.svg"));
    map.insert("settings", include_str!("../assets/icons/lucide/settings.svg"));
    map.insert("user", include_str!("../assets/icons/lucide/user.svg"));
    map.insert("home", include_str!("../assets/icons/lucide/home.svg"));
    map.insert("plus", include_str!("../assets/icons/lucide/plus.svg"));
    map.insert("trash", include_str!("../assets/icons/lucide/trash.svg"));
    map.insert("edit", include_str!("../assets/icons/lucide/edit.svg"));
    map.insert("mail", include_str!("../assets/icons/lucide/mail.svg"));
    // ... добавить остальные по мере необходимости
    
    map
});

pub fn get_icon_svg(name: &str) -> Option<IconSvg> {
    ICONS.get(name).copied()
}

// Enum для типобезопасности
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LucideIcon {
    X,
    Check,
    ChevronDown,
    ChevronUp,
    ChevronLeft,
    ChevronRight,
    Search,
    Menu,
    Settings,
    User,
    Home,
    Plus,
    Minus,
    Trash,
    Edit,
    Copy,
    Download,
    Upload,
    Mail,
    Phone,
    Calendar,
    Clock,
    Star,
    Heart,
    Info,
    AlertCircle,
    CheckCircle,
    XCircle,
    HelpCircle,
    // ... расширять по мере необходимости
}

impl LucideIcon {
    pub fn as_str(&self) -> &'static str {
        match self {
            LucideIcon::X => "x",
            LucideIcon::Check => "check",
            LucideIcon::ChevronDown => "chevron-down",
            LucideIcon::ChevronUp => "chevron-up",
            LucideIcon::ChevronLeft => "chevron-left",
            LucideIcon::ChevronRight => "chevron-right",
            LucideIcon::Search => "search",
            LucideIcon::Menu => "menu",
            LucideIcon::Settings => "settings",
            LucideIcon::User => "user",
            LucideIcon::Home => "home",
            LucideIcon::Plus => "plus",
            LucideIcon::Minus => "minus",
            LucideIcon::Trash => "trash",
            LucideIcon::Edit => "edit",
            LucideIcon::Copy => "copy",
            LucideIcon::Download => "download",
            LucideIcon::Upload => "upload",
            LucideIcon::Mail => "mail",
            LucideIcon::Phone => "phone",
            LucideIcon::Calendar => "calendar",
            LucideIcon::Clock => "clock",
            LucideIcon::Star => "star",
            LucideIcon::Heart => "heart",
            LucideIcon::Info => "info",
            LucideIcon::AlertCircle => "alert-circle",
            LucideIcon::CheckCircle => "check-circle",
            LucideIcon::XCircle => "x-circle",
            LucideIcon::HelpCircle => "help-circle",
        }
    }
    
    pub fn svg(&self) -> Option<IconSvg> {
        get_icon_svg(self.as_str())
    }
}
```

#### Компонент Icon

```rust
// crates/ui/src/components/icon.rs

use gpui::*;
use crate::icons::LucideIcon;
use crate::prelude::*;

#[derive(IntoElement)]
pub struct Icon {
    icon: LucideIcon,
    size: IconSize,
    color: Option<Hsla>,
    stroke_width: f32,
    
    // Accessibility
    label: Option<SharedString>,
    decorative: bool, // если true, то aria-hidden="true"
}

#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    Xs,  // 12px
    Sm,  // 16px
    Md,  // 20px
    Lg,  // 24px (default Lucide size)
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
            stroke_width: 2.0, // default Lucide stroke width
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
        
        // Для GPUI нужно рендерить SVG через gpui::svg или использовать img
        // Вариант 1: Если GPUI поддерживает inline SVG
        svg()
            .size(size)
            .path(self.icon.svg().unwrap_or(""))
            .text_color(color)
            // Accessibility attributes
            .when(!self.decorative, |svg| {
                svg.aria_label(self.label.unwrap_or_else(|| {
                    // Default label - название иконки
                    self.icon.as_str().replace('-', " ").into()
                }))
            })
            .when(self.decorative, |svg| {
                svg.aria_hidden(true)
            })
    }
}
```

#### Build-time генерация

Создать build script для автоматической генерации иконок:

```rust
// build.rs

use std::fs;
use std::path::Path;

fn main() {
    // Скачать lucide icons при сборке
    download_lucide_icons();
    
    // Генерировать Rust код с enum и mappings
    generate_icon_enum();
}

fn download_lucide_icons() {
    // Можно использовать git submodule или скачивать напрямую
    // https://github.com/lucide-icons/lucide/tree/main/icons
}

fn generate_icon_enum() {
    let icons_dir = Path::new("assets/icons/lucide");
    let output_path = Path::new("src/icons/generated.rs");
    
    // Читать все SVG файлы
    // Генерировать enum варианты
    // Генерировать match statements
    
    // Псевдокод:
    // for each svg in icons_dir {
    //     enum_variants.push(to_pascal_case(filename));
    //     match_arms.push(mapping);
    // }
}
```

### Использование Icon

```rust
// Декоративная иконка (не нужна для screen readers)
Icon::new(LucideIcon::Star)
    .decorative()

// Иконка с семантическим значением
Icon::new(LucideIcon::Search)
    .label("Search")
    .size(IconSize::Md)

// Кастомные стили
Icon::new(LucideIcon::Heart)
    .color(theme.colors.destructive)
    .size(IconSize::Xl)
    .stroke_width(3.0)

// В кнопке
Button::new("delete")
    .leading_icon(Icon::new(LucideIcon::Trash).decorative())
    .label("Delete")
    .variant(ButtonVariant::Destructive)
```

---

## 2. Accessibility (Доступность)

### Принципы ARIA

**Основное правило:** "No ARIA is better than bad ARIA"

1. **Semantic HTML First** - используйте семантические HTML элементы вместо ARIA, когда возможно
2. **ARIA только когда необходимо** - добавляйте ARIA только если HTML не может решить задачу
3. **Не дублируйте роли** - не добавляйте `role="button"` к `<button>`
4. **Keyboard accessibility** - все интерактивные элементы должны быть доступны с клавиатуры

### Архитектура Accessibility в GPUI

#### Accessibility Context

```rust
// crates/ui/src/accessibility/mod.rs

use gpui::*;

#[derive(Clone)]
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

impl Default for AccessibilityProps {
    fn default() -> Self {
        Self {
            role: None,
            label: None,
            description: None,
            hidden: false,
            live: None,
            expanded: None,
            selected: None,
            checked: None,
            disabled: false,
            required: false,
            invalid: false,
            readonly: false,
            autocomplete: None,
            haspopup: None,
            controls: None,
            labelledby: None,
            describedby: None,
        }
    }
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
    // ... и другие WAI-ARIA роли
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

// Trait для применения accessibility к элементам
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

// Реализация для Div и других элементов
impl Accessible for Div {
    fn accessibility(self, props: AccessibilityProps) -> Self {
        // GPUI имеет свою систему accessibility
        // Нужно адаптировать к GPUI API
        let mut element = self;
        
        if let Some(role) = props.role {
            // element = element.role(role);
        }
        
        if let Some(label) = props.label {
            // element = element.aria_label(label);
        }
        
        if props.hidden {
            // element = element.aria_hidden(true);
        }
        
        // ... остальные атрибуты
        
        element
    }
}
```

### Accessibility в компонентах

#### Button с полной accessibility

```rust
impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .id(self.id)
            // Семантика - не нужна role="button" для кнопки
            // но если это div, то нужно:
            .aria_role(AriaRole::Button)
            .aria_label(self.aria_label.unwrap_or(self.label.clone()))
            .aria_disabled(self.disabled)
            .when(self.pressed.is_some(), |div| {
                div.aria_pressed(self.pressed.unwrap())
            })
            // Keyboard support
            .on_key_down(Key::Enter, |_, cx| {
                // Trigger click
            })
            .on_key_down(Key::Space, |_, cx| {
                // Trigger click
            })
            // Focus management
            .focusable()
            .on_focus(|event, cx| {
                // Show focus ring
            })
            .on_blur(|event, cx| {
                // Hide focus ring
            })
            // Styling
            .when(self.has_focus, |div| {
                div.outline(theme.colors.ring)
                   .outline_width(px(2.))
                   .outline_offset(px(2.))
            })
            // ... остальная стилизация
            .child(self.label)
    }
}
```

#### TextField с accessibility

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
            .when_some(self.label.clone(), |col, label| {
                col.child(
                    div()
                        .id(label_id.clone())
                        .text_size(theme.typography.size_sm)
                        .text_color(theme.colors.foreground)
                        .child(label)
                        .when(self.required, |div| {
                            div.child(
                                Text::new("*")
                                    .color(theme.colors.destructive)
                                    .ml(px(2.))
                            )
                        })
                )
            })
            // Input field
            .child(
                div()
                    .id(input_id.clone())
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
                    .when_some(self.placeholder.clone(), |div, placeholder| {
                        div.placeholder(placeholder)
                    })
                    // Keyboard support
                    .focusable()
                    .on_key_down(Key::Enter, |event, cx| {
                        // Submit handler
                    })
                    // ... остальная логика input
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

#### Dialog с accessibility

```rust
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    title: Option<SharedString>,
    description: Option<SharedString>,
    content: Option<AnyElement>,
    actions: Vec<AnyElement>,
    open: bool,
    on_close: Option<Box<dyn Fn(&mut WindowContext)>>,
}

impl RenderOnce for Dialog {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        let title_id = ElementId::from(format!("{}-title", self.id));
        let desc_id = ElementId::from(format!("{}-description", self.id));
        
        // Overlay backdrop
        div()
            .when(self.open, |div| {
                div.absolute()
                    .inset_0()
                    .bg(hsla(0., 0., 0., 0.5))
                    .z_index(999)
                    // Accessibility
                    .aria_hidden(true) // backdrop декоративный
                    // Click outside to close
                    .on_click(|_, cx| {
                        if let Some(on_close) = self.on_close {
                            on_close(cx);
                        }
                    })
                    // Dialog container
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size_full()
                            .child(
                                // Dialog panel
                                div()
                                    .id(self.id.clone())
                                    .aria_role(AriaRole::Dialog)
                                    .aria_labelledby(title_id.clone())
                                    .when(self.description.is_some(), |div| {
                                        div.aria_describedby(desc_id.clone())
                                    })
                                    .aria_modal(true)
                                    // Focus trap
                                    .focusable()
                                    .on_key_down(Key::Escape, |_, cx| {
                                        // Close on Escape
                                        if let Some(on_close) = self.on_close {
                                            on_close(cx);
                                        }
                                    })
                                    // Styling
                                    .bg(theme.colors.background)
                                    .rounded(theme.radius.lg)
                                    .shadow_lg()
                                    .max_w(px(500.))
                                    .p(theme.spacing.lg)
                                    // Content
                                    .child(
                                        Column::new()
                                            .gap(theme.spacing.md)
                                            // Title
                                            .when_some(self.title, |col, title| {
                                                col.child(
                                                    div()
                                                        .id(title_id)
                                                        .text_size(theme.typography.size_xl)
                                                        .font_weight(theme.typography.weight_semibold)
                                                        .child(title)
                                                )
                                            })
                                            // Description
                                            .when_some(self.description, |col, desc| {
                                                col.child(
                                                    div()
                                                        .id(desc_id)
                                                        .text_size(theme.typography.size_sm)
                                                        .text_color(theme.colors.muted_foreground)
                                                        .child(desc)
                                                )
                                            })
                                            // Content
                                            .when_some(self.content, |col, content| {
                                                col.child(content)
                                            })
                                            // Actions
                                            .when(!self.actions.is_empty(), |col| {
                                                col.child(
                                                    Row::new()
                                                        .gap(theme.spacing.sm)
                                                        .justify_end()
                                                        .children(self.actions)
                                                )
                                            })
                                    )
                            )
                    )
            })
    }
}
```

### Keyboard Navigation

Все компоненты должны поддерживать клавиатурную навигацию:

```rust
pub trait KeyboardNavigable {
    // Tab/Shift+Tab - переход между элементами (handled by GPUI)
    
    // Enter/Space - активация
    fn on_activate(&self, cx: &mut WindowContext);
    
    // Arrow keys - навигация в списках/меню
    fn on_arrow_up(&self, cx: &mut WindowContext) {}
    fn on_arrow_down(&self, cx: &mut WindowContext) {}
    fn on_arrow_left(&self, cx: &mut WindowContext) {}
    fn on_arrow_right(&self, cx: &mut WindowContext) {}
    
    // Escape - закрытие/отмена
    fn on_escape(&self, cx: &mut WindowContext) {}
    
    // Home/End - начало/конец списка
    fn on_home(&self, cx: &mut WindowContext) {}
    fn on_end(&self, cx: &mut WindowContext) {}
}
```

### Focus Management

```rust
pub struct FocusManager {
    focus_stack: Vec<ElementId>,
    focus_trap_enabled: bool,
}

impl FocusManager {
    // Установить фокус на элемент
    pub fn focus(&mut self, id: ElementId, cx: &mut WindowContext) {
        cx.focus(&id);
        self.focus_stack.push(id);
    }
    
    // Вернуть фокус к предыдущему элементу
    pub fn restore_focus(&mut self, cx: &mut WindowContext) {
        if let Some(id) = self.focus_stack.pop() {
            cx.focus(&id);
        }
    }
    
    // Focus trap для модальных окон
    pub fn enable_focus_trap(&mut self, container: ElementId) {
        self.focus_trap_enabled = true;
        // Ограничить Tab навигацию только внутри container
    }
    
    pub fn disable_focus_trap(&mut self) {
        self.focus_trap_enabled = false;
    }
}
```

---

## 3. Лучшие практики

### Иконки

✅ **DO:**
- Всегда указывайте `label()` для иконок с семантическим значением
- Используйте `decorative()` для чисто визуальных иконок
- Кастомизируйте размер и цвет через API
- Используйте типобезопасный `LucideIcon` enum

❌ **DON'T:**
- Не используйте иконки без accessibility labels (если они не decorative)
- Не хардкодите SVG строки напрямую в компонентах
- Не забывайте про контрастность цвета иконки

### Accessibility

✅ **DO:**
- Используйте семантические роли только когда HTML не подходит
- Всегда связывайте labels с inputs через `aria-labelledby`
- Используйте `aria-live` для динамического контента
- Поддерживайте полную клавиатурную навигацию
- Тестируйте со screen readers

❌ **DON'T:**
- Не дублируйте ARIA роли для семантических элементов
- Не скрывайте фокусируемые элементы с `aria-hidden="true"`
- Не забывайте про focus indicators
- Не используйте `div` вместо `button` без ARIA

### Пример правильного компонента

```rust
Button::new("submit")
    .label("Submit Form")
    .leading_icon(Icon::new(LucideIcon::Check).decorative())
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Lg)
    .disabled(form_invalid)
    // Accessibility автоматически:
    // - role="button" (если нужно)
    // - aria-label="Submit Form"
    // - aria-disabled="true" (если disabled)
    // - keyboard support (Enter, Space)
    // - focus management
    .on_click(|_, cx| {
        // Submit logic
    })
```

---

## Источники

### Lucide Icons
- [Lucide Official Site](https://lucide.dev)
- [Lucide Icons on GitHub](https://github.com/lucide-icons/lucide)
- [Lucide Static Package](https://lucide.dev/guide/packages/lucide-static)
- [lucide-react-sprite](https://github.com/homielab/lucide-react-sprite)
- [Best React Icon Libraries for 2026](https://mighil.com/best-react-icon-libraries)

### ARIA & Accessibility
- [ARIA - Accessibility | MDN](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA)
- [WAI-ARIA Overview | W3C](https://www.w3.org/WAI/standards-guidelines/aria/)
- [ARIA Authoring Practices Guide | W3C](https://www.w3.org/WAI/ARIA/apg/)
- [Modern Frontend Accessibility: A 2026 Developer's Guide](https://medium.com/design-bootcamp/modern-frontend-accessibility-a-2026-developers-guide-b2de10d01d22)
- [ARIA Accessibility Best Practices](https://www.accessibilitychecker.org/blog/aria-accessibility/)
- [ARIA and HTML | web.dev](https://web.dev/learn/accessibility/aria-html)
- [WebAIM: Introduction to ARIA](https://webaim.org/techniques/aria/)

---

**Конец документа**

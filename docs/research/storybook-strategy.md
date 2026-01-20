# Storybook стратегия для gpui-flutter

Дата: 2026-01-19

---

## Обзор

Для разработки и демонстрации компонентов библиотеки нужен инструмент визуального просмотра (аналог Storybook для React/Vue).

---

## Существующие решения

### gpui-storybook

**Репозиторий**: https://github.com/stayhydated/gpui-storybook
**Crate**: https://crates.io/crates/gpui-storybook (v0.5.0)
**Лицензия**: MIT / Apache-2.0

#### Возможности

- ✅ Story trait для создания stories компонентов
- ✅ Attribute macros `#[story]` и `#[story_init]`
- ✅ Gallery UI с навигацией по компонентам
- ✅ i18n поддержка (смена языков)
- ✅ Embedded assets
- ✅ Автоматическая регистрация stories
- ✅ Интеграция с gpui-component

#### API

```rust
use gpui::*;
use gpui_storybook::*;

#[story]
struct ButtonStory;

impl Story for ButtonStory {
    fn name(&self) -> &str {
        "Button"
    }
    
    fn category(&self) -> &str {
        "Buttons"
    }
    
    fn render(&self, cx: &mut WindowContext) -> impl IntoElement {
        Button::new("demo").label("Click me!")
    }
}

#[story_init]
fn init_stories(cx: &mut AppContext) {
    gpui_flutter::init(cx);
}

fn main() {
    App::new().run(|cx| {
        gpui_storybook::init(cx);
        gpui_storybook::create_new_window(cx);
    });
}
```

---

## Рекомендация: Гибридный подход

Использовать **gpui-storybook** для быстрого старта, но с возможностью кастомизации.

### Структура проекта

```
gpui-flutter/
├── Cargo.toml
├── crates/
│   ├── ui/                    # Основная библиотека компонентов
│   │   ├── src/
│   │   │   ├── components/
│   │   │   ├── theme/
│   │   │   ├── icons/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   └── gallery/               # Gallery/Storybook приложение
│       ├── src/
│       │   ├── main.rs
│       │   ├── stories/
│       │   │   ├── mod.rs
│       │   │   ├── layout/
│       │   │   │   ├── container.rs
│       │   │   │   ├── row.rs
│       │   │   │   └── column.rs
│       │   │   ├── buttons/
│       │   │   │   ├── button.rs
│       │   │   │   └── icon_button.rs
│       │   │   ├── inputs/
│       │   │   │   ├── text_field.rs
│       │   │   │   ├── checkbox.rs
│       │   │   │   └── switch.rs
│       │   │   ├── cards/
│       │   │   │   └── card.rs
│       │   │   └── icons/
│       │   │       └── icon.rs
│       │   └── custom/         # Кастомный UI (опционально)
│       │       ├── sidebar.rs
│       │       └── theme_switcher.rs
│       └── Cargo.toml
│
├── examples/                   # Примеры использования
│   ├── basic_app/
│   ├── form_example/
│   └── dashboard/
│
└── docs/
    └── research/
```

### Cargo.toml для gallery

```toml
[package]
name = "gpui-flutter-gallery"
version = "0.1.0"
edition = "2021"

[dependencies]
gpui = "0.2"
gpui-flutter = { path = "../ui" }
gpui-storybook = "0.5"
lucide-icons = "0.555"

[features]
default = ["all-stories"]
all-stories = []

# Отдельные фичи для категорий (для ускорения компиляции в разработке)
layout = []
buttons = []
inputs = []
cards = []
```

### main.rs

```rust
use gpui::*;
use gpui_storybook::*;

// Импорт всех stories
mod stories;

#[story_init]
fn init_stories(cx: &mut AppContext) {
    // Инициализация библиотеки
    gpui_flutter::init(cx);
    
    // Загрузка дефолтной темы
    gpui_flutter::theme::load_default_themes(cx);
}

fn main() {
    App::new().run(|cx| {
        // Настройка логгера
        env_logger::init();
        
        // Инициализация storybook
        gpui_storybook::init(cx);
        
        // Создание окна gallery
        gpui_storybook::create_new_window(
            cx,
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(100.), px(100.)),
                    size: size(px(1400.), px(900.)),
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("gpui-flutter Gallery".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        );
    });
}
```

### Пример Story - Button

```rust
// crates/gallery/src/stories/buttons/button.rs

use gpui::*;
use gpui_storybook::*;
use gpui_flutter::prelude::*;
use lucide_icons::Icon as LucideIcon;

#[story]
pub struct ButtonStory;

impl Story for ButtonStory {
    fn name(&self) -> &str {
        "Button"
    }
    
    fn category(&self) -> &str {
        "Buttons"
    }
    
    fn description(&self) -> Option<&str> {
        Some("Buttons allow users to take actions with a single tap")
    }
    
    fn render(&self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        Column::new()
            .gap(32.)
            .p(24.)
            .size_full()
            .overflow_y_scroll()
            // Variants
            .child(story_section(
                "Variants",
                "Different button styles for different contexts",
                Column::new()
                    .gap(16.)
                    .child(
                        Row::new()
                            .gap(8.)
                            .child(Button::new("default").label("Default").variant(ButtonVariant::Default))
                            .child(Button::new("outline").label("Outline").variant(ButtonVariant::Outline))
                            .child(Button::new("secondary").label("Secondary").variant(ButtonVariant::Secondary))
                            .child(Button::new("ghost").label("Ghost").variant(ButtonVariant::Ghost))
                            .child(Button::new("destructive").label("Destructive").variant(ButtonVariant::Destructive))
                            .child(Button::new("link").label("Link").variant(ButtonVariant::Link))
                    )
            ))
            // Sizes
            .child(story_section(
                "Sizes",
                "Buttons come in different sizes",
                Row::new()
                    .gap(8.)
                    .items_center()
                    .child(Button::new("sm").label("Small").size(ButtonSize::Sm))
                    .child(Button::new("md").label("Medium").size(ButtonSize::Md))
                    .child(Button::new("lg").label("Large").size(ButtonSize::Lg))
            ))
            // With Icons
            .child(story_section(
                "With Icons",
                "Buttons can have leading or trailing icons",
                Column::new()
                    .gap(8.)
                    .child(
                        Row::new()
                            .gap(8.)
                            .child(
                                Button::new("icon-left")
                                    .label("Send Email")
                                    .leading_icon(Icon::new(LucideIcon::Mail).decorative())
                            )
                            .child(
                                Button::new("icon-right")
                                    .label("Next")
                                    .trailing_icon(Icon::new(LucideIcon::ArrowRight).decorative())
                            )
                    )
            ))
            // States
            .child(story_section(
                "States",
                "Different button states",
                Row::new()
                    .gap(8.)
                    .child(Button::new("normal").label("Normal"))
                    .child(Button::new("disabled").label("Disabled").disabled(true))
                    .child(Button::new("loading").label("Loading").loading(true))
            ))
            // Interactive Example
            .child(story_section(
                "Interactive",
                "Click the button to see interaction",
                InteractiveButtonDemo::new()
            ))
    }
}

// Helper для создания секций
fn story_section(
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    content: impl IntoElement,
) -> impl IntoElement {
    Column::new()
        .gap(8.)
        .child(
            Column::new()
                .gap(4.)
                .child(
                    Text::new(title)
                        .size(18.)
                        .weight(600)
                )
                .child(
                    Text::new(description)
                        .size(14.)
                        .color(cx.theme().colors.muted_foreground)
                )
        )
        .child(
            Container::new()
                .p(16.)
                .border_1()
                .border_color(cx.theme().colors.border)
                .rounded(8.)
                .child(content)
        )
}

// Интерактивный пример
#[derive(IntoElement)]
struct InteractiveButtonDemo {
    click_count: usize,
}

impl InteractiveButtonDemo {
    fn new() -> Self {
        Self { click_count: 0 }
    }
}

impl RenderOnce for InteractiveButtonDemo {
    fn render(mut self, cx: &mut WindowContext) -> impl IntoElement {
        Column::new()
            .gap(16.)
            .child(
                Text::new(format!("Clicked {} times", self.click_count))
                    .size(16.)
            )
            .child(
                Button::new("counter")
                    .label("Click Me!")
                    .on_click(move |_, cx| {
                        self.click_count += 1;
                        cx.refresh();
                    })
            )
    }
}
```

### Пример Story - TextField

```rust
// crates/gallery/src/stories/inputs/text_field.rs

use gpui::*;
use gpui_storybook::*;
use gpui_flutter::prelude::*;
use lucide_icons::Icon as LucideIcon;

#[story]
pub struct TextFieldStory;

impl Story for TextFieldStory {
    fn name(&self) -> &str {
        "TextField"
    }
    
    fn category(&self) -> &str {
        "Inputs"
    }
    
    fn description(&self) -> Option<&str> {
        Some("Text fields let users enter and edit text")
    }
    
    fn render(&self, cx: &mut WindowContext) -> impl IntoElement {
        Column::new()
            .gap(32.)
            .p(24.)
            .size_full()
            .max_w(px(800.))
            .overflow_y_scroll()
            // Basic
            .child(story_section(
                "Basic",
                "Simple text input",
                TextField::new("basic")
                    .placeholder("Enter text...")
            ))
            // With Label
            .child(story_section(
                "With Label",
                "Text field with label and helper text",
                TextField::new("labeled")
                    .label("Email")
                    .placeholder("you@example.com")
                    .helper_text("We'll never share your email with anyone else")
            ))
            // With Icons
            .child(story_section(
                "With Icons",
                "Text fields can have leading and trailing icons",
                Column::new()
                    .gap(16.)
                    .child(
                        TextField::new("search")
                            .label("Search")
                            .placeholder("Search...")
                            .leading_icon(Icon::new(LucideIcon::Search).decorative())
                    )
                    .child(
                        TextField::new("email")
                            .label("Email")
                            .placeholder("Enter your email")
                            .leading_icon(Icon::new(LucideIcon::Mail).decorative())
                            .trailing_icon(Icon::new(LucideIcon::Check).decorative())
                    )
            ))
            // Variants
            .child(story_section(
                "Variants",
                "Different visual styles",
                Column::new()
                    .gap(16.)
                    .child(
                        TextField::new("outlined")
                            .label("Outlined")
                            .variant(TextFieldVariant::Outlined)
                            .placeholder("Outlined variant")
                    )
                    .child(
                        TextField::new("filled")
                            .label("Filled")
                            .variant(TextFieldVariant::Filled)
                            .placeholder("Filled variant")
                    )
                    .child(
                        TextField::new("underlined")
                            .label("Underlined")
                            .variant(TextFieldVariant::Underlined)
                            .placeholder("Underlined variant")
                    )
            ))
            // States
            .child(story_section(
                "States",
                "Different input states",
                Column::new()
                    .gap(16.)
                    .child(
                        TextField::new("disabled")
                            .label("Disabled")
                            .placeholder("Disabled input")
                            .disabled(true)
                    )
                    .child(
                        TextField::new("readonly")
                            .label("Readonly")
                            .value("Read-only value")
                            .readonly(true)
                    )
                    .child(
                        TextField::new("error")
                            .label("With Error")
                            .placeholder("Enter valid email")
                            .error("This field is required")
                    )
                    .child(
                        TextField::new("required")
                            .label("Required Field")
                            .placeholder("Required...")
                            .required(true)
                    )
            ))
            // Sizes
            .child(story_section(
                "Sizes",
                "Different input sizes",
                Column::new()
                    .gap(16.)
                    .child(
                        TextField::new("sm")
                            .label("Small")
                            .size(Size::Sm)
                            .placeholder("Small input")
                    )
                    .child(
                        TextField::new("md")
                            .label("Medium")
                            .size(Size::Md)
                            .placeholder("Medium input")
                    )
                    .child(
                        TextField::new("lg")
                            .label("Large")
                            .size(Size::Lg)
                            .placeholder("Large input")
                    )
            ))
    }
}
```

### stories/mod.rs

```rust
// crates/gallery/src/stories/mod.rs

// Layout components
#[cfg(feature = "layout")]
pub mod layout {
    pub mod container;
    pub mod row;
    pub mod column;
}

// Button components
#[cfg(feature = "buttons")]
pub mod buttons {
    pub mod button;
    pub mod icon_button;
}

// Input components
#[cfg(feature = "inputs")]
pub mod inputs {
    pub mod text_field;
    pub mod checkbox;
    pub mod switch;
    pub mod radio;
}

// Card components
#[cfg(feature = "cards")]
pub mod cards {
    pub mod card;
}

// Icons
pub mod icons;

// Theme demo
pub mod theme;
```

---

## Кастомизация Gallery UI (Опционально)

Если нужен более кастомизированный UI, можно расширить стандартный Gallery:

```rust
// crates/gallery/src/custom/theme_switcher.rs

use gpui::*;
use gpui_flutter::prelude::*;

#[derive(IntoElement)]
pub struct ThemeSwitcher {
    current_theme: String,
}

impl ThemeSwitcher {
    pub fn new(current_theme: String) -> Self {
        Self { current_theme }
    }
}

impl RenderOnce for ThemeSwitcher {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        Row::new()
            .gap(8.)
            .child(Text::new("Theme:"))
            .child(
                DropdownButton::new("theme-select")
                    .value(self.current_theme.clone())
                    .items(vec![
                        "Default Light".to_string(),
                        "Default Dark".to_string(),
                        "Catppuccin".to_string(),
                        "Nord".to_string(),
                    ])
                    .on_change(|theme, cx| {
                        gpui_flutter::theme::set_active_theme(&theme, cx);
                    })
            )
    }
}
```

---

## Workflow разработки

### 1. Запуск Gallery

```bash
# Запустить gallery со всеми компонентами
cargo run -p gpui-flutter-gallery

# Запустить только конкретную категорию (быстрее компиляция)
cargo run -p gpui-flutter-gallery --features layout
cargo run -p gpui-flutter-gallery --features buttons
```

### 2. Hot Reload (опционально)

Для быстрой итерации можно использовать `cargo-watch`:

```bash
cargo install cargo-watch

# Watch и перезапуск при изменениях
cargo watch -x 'run -p gpui-flutter-gallery'
```

### 3. Workflow создания нового компонента

```bash
# 1. Создать компонент
touch crates/ui/src/components/my_component.rs

# 2. Создать story
touch crates/gallery/src/stories/category/my_component.rs

# 3. Добавить в mod.rs
# echo 'pub mod my_component;' >> crates/gallery/src/stories/category/mod.rs

# 4. Запустить gallery и проверить
cargo run -p gpui-flutter-gallery
```

---

## Преимущества подхода

### ✅ Использование gpui-storybook:
- Готовый UI из коробки
- Автоматическая регистрация stories через macros
- Навигация по категориям
- i18n поддержка
- Не нужно писать UI с нуля

### ✅ Гибкость:
- Можно кастомизировать при необходимости
- Feature flags для быстрой компиляции
- Отдельный crate не засоряет основную библиотеку
- Легко добавлять новые stories

### ✅ Developer Experience:
- Быстрая итерация
- Визуальный просмотр всех компонентов
- Тестирование вариантов и состояний
- Документация через примеры

---

## Альтернатива: Простой Gallery без зависимостей

Если gpui-storybook кажется избыточным, можно создать простой gallery:

```rust
// Минимальный gallery без зависимостей
struct SimpleGallery {
    stories: Vec<Box<dyn StoryItem>>,
    selected: usize,
}

trait StoryItem {
    fn name(&self) -> &str;
    fn render(&self, cx: &mut WindowContext) -> impl IntoElement;
}

// Использование
impl Render for SimpleGallery {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        Row::new()
            .size_full()
            // Sidebar
            .child(
                Column::new()
                    .w(px(250.))
                    .bg(cx.theme().colors.muted)
                    .children(
                        self.stories.iter().enumerate().map(|(i, story)| {
                            Button::new(format!("story-{}", i))
                                .label(story.name())
                                .ghost()
                                .selected(i == self.selected)
                                .on_click(move |_, cx| {
                                    self.selected = i;
                                    cx.refresh();
                                })
                        })
                    )
            )
            // Content
            .child(
                Container::new()
                    .flex_1()
                    .p(24.)
                    .child(self.stories[self.selected].render(cx))
            )
    }
}
```

---

## Рекомендация

**Для gpui-flutter:** Использовать **gpui-storybook** как основу.

**Причины:**
1. ✅ Экономит время - не нужно писать UI gallery с нуля
2. ✅ Интегрируется с gpui-component (если решим его использовать)
3. ✅ Macros упрощают добавление новых stories
4. ✅ i18n из коробки (может пригодиться в будущем)
5. ✅ Активно поддерживается

**Следующие шаги:**
1. Создать `crates/gallery` с gpui-storybook
2. Добавить первые stories для базовых компонентов
3. По мере разработки компонентов добавлять stories
4. При необходимости кастомизировать UI

---

## Ссылки

- [gpui-storybook на GitHub](https://github.com/stayhydated/gpui-storybook)
- [gpui-storybook на crates.io](https://crates.io/crates/gpui-storybook)
- [gpui-component story example](https://github.com/longbridge/gpui-component/tree/main/crates/story)

---

**Конец документа**

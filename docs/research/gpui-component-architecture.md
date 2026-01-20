# GPUI Component Architecture Research
## Исследование реализации тем и компонентов в GPUI проектах

Дата: 2026-01-19

---

## Обзор

Это исследование анализирует, как темы и компоненты реализованы в существующих GPUI-проектах, с целью создания библиотеки компонентов в стиле Flutter с дизайном shadcn/ui.

---

## 1. Анализ существующих GPUI проектов

### 1.1 longbridge/gpui-component

**Репозиторий**: https://github.com/longbridge/gpui-component

#### Система тем

**Архитектура**:
- Темы определяются в JSON файлах (`.theme-schema.json`)
- `ThemeRegistry` управляет загрузкой и переключением тем
- `Theme` структура содержит активную тему с `ThemeColor`
- Глобальный доступ через `Theme::global()` и `cx.theme()`

**Структура темы**:
```rust
ThemeSet {
    name: String,
    author: String,
    url: String,
    themes: Vec<ThemeConfig>
}

ThemeConfig {
    name: String,
    mode: "light" | "dark",
    font: { size, family },
    radius: { sm, md, lg },
    colors: ThemeConfigColors {
        accent: { background, foreground },
        background,
        foreground,
        border,
        primary,
        secondary,
        muted,
        destructive,
        // ... и многие другие
    }
}
```

**Цвета**:
- Использует формат `Hsla` (Hue, Saturation, Lightness, Alpha)
- Семантические названия: `background`, `foreground`, `primary`, `accent`
- Пары цветов: `background`/`foreground`, `primary`/`primary-foreground`

**Применение в компонентах**:
```rust
// Доступ к теме в компоненте
let theme = cx.theme();
theme.list_active
theme.accent
theme.border
```

#### Паттерны компонентов

**Два основных паттерна**:

1. **Stateless `RenderOnce`** - для простых компонентов:
```rust
#[derive(IntoElement)]
struct Button {
    label: String,
    variant: ButtonVariant,
    size: Size,
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .bg(cx.theme().primary)
            .child(self.label)
    }
}
```

2. **Stateful `Render`** - для компонентов с состоянием:
```rust
struct Input {
    state: Entity<InputState>,
}

impl Render for Input {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        // ...
    }
}
```

**Fluent API для стилизации**:
```rust
Button::new("ok")
    .primary()
    .label("Click Me")
    .on_click(|_, _, _| println!("Clicked!"))
```

**Трейты для поведения компонентов**:
- `Sizable` - размеры (xs, sm, md, lg)
- `Selectable` - выбираемость
- `Disableable` - отключаемость
- `Collapsible` - сворачиваемость

**StyledExt трейт**:
```rust
div()
    .h_flex()           // horizontal flex
    .v_flex()           // vertical flex
    .gap_2()            // spacing
    .p_3()              // padding
    .rounded_md()       // border radius
    .bg(color)          // background
```

**Size enum**:
```rust
enum Size {
    XSmall,
    Small,
    Medium,
    Large,
    Size(Pixels), // custom
}

// Методы для компонентов
size.input_px()
size.input_py()
size.table_row_height()
```

**Композиция компонентов** (Compound Components):
```rust
Root::new(view, window, cx)
    .child(
        Sidebar::new(Side::Left)
            .header(SidebarHeader::new().child("App"))
            .child(
                SidebarGroup::new("Navigation")
                    .child(SidebarMenu::new()
                        .child(SidebarMenuItem::new("Dashboard")
                            .icon(IconName::LayoutDashboard)
                        )
                    )
            )
            .footer(SidebarFooter::new())
    )
```

---

### 1.2 Zed Editor (zed-industries/zed)

**Репозиторий**: https://github.com/zed-industries/zed

#### Система тем

**Архитектура**:
- `ThemeFamily` - семейство тем (light + dark варианты)
- `ThemeRegistry` - глобальный синглтон для управления темами
- `GlobalTheme` - текущая активная тема
- `ThemeSettings` - настройки темы с переопределениями

**Определение тем**:
- JSON файлы в `assets/themes/`
- Схема: `https://zed.dev/schema/themes/v0.2.0.json`
- Структура: `name`, `appearance`, `style`

**Доступ к теме**:
```rust
// Через ActiveTheme trait
let theme = cx.theme();

// Категории цветов
theme.colors()      // UI цвета
theme.syntax()      // подсветка синтаксиса
theme.accents()     // акцентные цвета
theme.status()      // статусные цвета

// Использование
cx.theme().colors().border_selected
theme.syntax_color(name)
```

**Переопределения**:
- Через `theme_overrides` в `settings.json`
- Позволяет кастомизировать отдельные цвета без создания новой темы

#### Компоненты

**Основной паттерн**:
- `Component` trait с методами: `id()`, `scope()`, `status()`, `name()`, `description()`, `preview()`
- `RenderOnce` для превращения данных в элементы
- `#[derive(IntoElement)]` для автоматической реализации

**Пример Avatar**:
```rust
#[derive(IntoElement, Documented, RegisterComponent)]
struct Avatar {
    image: ImageSource,
    size: Size,
    border_color: Option<Color>,
    indicator: Option<Indicator>,
}

impl Avatar {
    fn new(image: ImageSource) -> Self { ... }
    fn grayscale(mut self) -> Self { ... }
    fn border_color(mut self, color: Color) -> Self { ... }
    fn size(mut self, size: Size) -> Self { ... }
}
```

**Композиция**:
- `Div` как основной строительный блок
- Композиция через `child()` методы
- Примеры: `SplitButton` = `ButtonLike` + `IconButton`

---

### 1.3 Другие проекты из awesome-gpui

#### Loungy (app launcher)
- Использует **Catppuccin** тему (готовая дизайн-система)
- Модульная архитектура с Cargo feature flags
- GPU-ускоренный рендеринг

#### Hummingbird (music player)
- Упоминает "Theming with hot reload"
- Детали реализации требуют изучения исходников

---

## 2. Анализ shadcn/ui дизайн-системы

**Официальный сайт**: https://ui.shadcn.com/

### 2.1 Архитектура shadcn/ui

#### Двухслойная архитектура

1. **Поведенческий слой** (Radix UI):
   - Headless компоненты
   - Accessibility (WAI-ARIA)
   - Клавиатурная навигация
   - Управление фокусом

2. **Визуальный слой** (shadcn/ui):
   - Стилизация через Tailwind CSS
   - Дизайн-токены
   - Варианты компонентов

#### Ключевые принципы

- **Open Code**: код компонентов открыт для модификации
- **Composition**: все компоненты используют общий композируемый интерфейс
- **Design Tokens**: централизованные значения для цветов, отступов, радиусов
- **Variants**: управление вариантами через Class Variance Authority (CVA)

### 2.2 Система дизайн-токенов

#### Цвета

**Семантические переменные CSS**:
```css
:root {
  --background: ...;
  --foreground: ...;
  --primary: ...;
  --primary-foreground: ...;
  --secondary: ...;
  --secondary-foreground: ...;
  --muted: ...;
  --muted-foreground: ...;
  --accent: ...;
  --accent-foreground: ...;
  --destructive: ...;
  --destructive-foreground: ...;
  --border: ...;
  --input: ...;
  --ring: ...;
}
```

**Формат**: OKLCH (современный цветовой формат)

**Паттерн**: пары background/foreground для гарантии контраста

#### Радиусы

```css
:root {
  --radius: 0.625rem;      /* default */
  --radius-sm: 4px;
  --radius-md: 8px;
  --radius-lg: 12px;
}
```

#### Типографика

- Размеры шрифтов
- Веса (font-weight)
- Высота строк (line-height)
- Семейства шрифтов

#### Отступы

- Базируются на Tailwind утилитах
- Стандартная шкала: 0.25rem, 0.5rem, 0.75rem, 1rem...

### 2.3 Компонент Button

#### Варианты

```typescript
variants: {
  variant: {
    default: "bg-primary text-primary-foreground",
    outline: "border border-input",
    secondary: "bg-secondary text-secondary-foreground",
    ghost: "hover:bg-accent",
    destructive: "bg-destructive text-destructive-foreground",
    link: "text-primary underline-offset-4"
  }
}
```

#### Размеры

```typescript
size: {
  sm: "h-9 px-3",
  default: "h-10 px-4 py-2",
  lg: "h-11 px-8",
  icon: "h-10 w-10"
}
```

### 2.4 Compound Components (Card)

```tsx
<Card>
  <CardHeader>
    <CardTitle>...</CardTitle>
    <CardDescription>...</CardDescription>
    <CardAction>...</CardAction>
  </CardHeader>
  <CardContent>...</CardContent>
  <CardFooter>...</CardFooter>
</Card>
```

**Паттерн**: каждая часть — отдельный компонент, гибкая композиция

---

## 3. Анализ Flutter компонентов

### 3.1 Архитектура Material Design в Flutter

#### ThemeData

- Единый источник истины для стилей
- Содержит цвета, типографику, формы, высоты, стили компонентов
- Передается в `MaterialApp`

#### Decoupling (2026)

- Material и Cupertino выносятся из SDK в отдельные пакеты
- Общая агностичная инфраструктура для тем в widgets layer
- Design tokens распространяются вниз по дереву

### 3.2 Паттерны компонентов Flutter

#### Базовые компоненты

- **Container**: аналог div, padding, margin, decoration
- **Column/Row**: вертикальная/горизонтальная раскладка
- **Text**: текстовые элементы
- **Button**: ElevatedButton, TextButton, OutlinedButton

#### Композиция

```dart
Scaffold(
  appBar: AppBar(title: Text('Title')),
  body: Column(
    children: [
      Container(
        padding: EdgeInsets.all(16),
        child: Text('Hello'),
      ),
      ElevatedButton(
        onPressed: () {},
        child: Text('Click'),
      ),
    ],
  ),
)
```

#### Наследование стиля

- `Theme.of(context)` для доступа к теме
- Каскадное наследование цветов и стилей

---

## 4. Рекомендуемая архитектура для gpui-flutter

### 4.1 Система тем (на основе shadcn/ui + GPUI лучших практик)

#### Структура темы

```rust
pub struct Theme {
    // Метаданные
    pub name: String,
    pub mode: ThemeMode, // Light | Dark
    
    // Цвета (семантические)
    pub colors: ThemeColors,
    
    // Размеры и отступы
    pub spacing: Spacing,
    
    // Радиусы
    pub radius: Radius,
    
    // Типографика
    pub typography: Typography,
    
    // Тени
    pub shadows: Shadows,
}

pub struct ThemeColors {
    // Базовые
    pub background: Hsla,
    pub foreground: Hsla,
    
    // Первичные
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    
    // Вторичные
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    
    // Muted (приглушенные)
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    
    // Accent (акцентные)
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    
    // Destructive (деструктивные действия)
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    
    // Границы и инпуты
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla, // focus ring
    
    // Дополнительные
    pub success: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
}

pub struct Radius {
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
    pub full: Pixels, // 9999px для круглых элементов
}

pub struct Spacing {
    pub xs: Pixels,   // 4px
    pub sm: Pixels,   // 8px
    pub md: Pixels,   // 16px
    pub lg: Pixels,   // 24px
    pub xl: Pixels,   // 32px
    pub xxl: Pixels,  // 48px
}

pub struct Typography {
    pub font_family: String,
    pub font_mono: String,
    
    // Размеры
    pub size_xs: Pixels,
    pub size_sm: Pixels,
    pub size_base: Pixels,
    pub size_lg: Pixels,
    pub size_xl: Pixels,
    pub size_2xl: Pixels,
    
    // Веса
    pub weight_normal: u16,
    pub weight_medium: u16,
    pub weight_semibold: u16,
    pub weight_bold: u16,
}
```

#### ThemeRegistry

```rust
pub struct ThemeRegistry {
    themes: HashMap<String, Theme>,
    active_theme: String,
}

impl ThemeRegistry {
    pub fn global() -> &'static Self;
    pub fn load_theme(&mut self, path: &Path) -> Result<()>;
    pub fn set_active(&mut self, name: &str);
    pub fn get_active(&self) -> &Theme;
}

// Глобальный доступ
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for WindowContext<'_> {
    fn theme(&self) -> &Theme {
        ThemeRegistry::global().get_active()
    }
}
```

#### Определение тем в JSON

```json
{
  "name": "shadcn-default",
  "mode": "light",
  "colors": {
    "background": "0 0% 100%",
    "foreground": "222.2 84% 4.9%",
    "primary": "222.2 47.4% 11.2%",
    "primary-foreground": "210 40% 98%",
    "secondary": "210 40% 96.1%",
    "secondary-foreground": "222.2 47.4% 11.2%",
    "muted": "210 40% 96.1%",
    "muted-foreground": "215.4 16.3% 46.9%",
    "accent": "210 40% 96.1%",
    "accent-foreground": "222.2 47.4% 11.2%",
    "destructive": "0 84.2% 60.2%",
    "destructive-foreground": "210 40% 98%",
    "border": "214.3 31.8% 91.4%",
    "input": "214.3 31.8% 91.4%",
    "ring": "222.2 84% 4.9%"
  },
  "radius": {
    "sm": "4px",
    "md": "8px",
    "lg": "12px"
  },
  "spacing": {
    "xs": "4px",
    "sm": "8px",
    "md": "16px",
    "lg": "24px",
    "xl": "32px"
  }
}
```

### 4.2 Компонентная архитектура

#### Базовые принципы

1. **От простого к сложному**: Container → Button → Card → Complex widgets
2. **Compound Components**: составные компоненты как в shadcn/ui
3. **Fluent API**: цепочные методы для конфигурации
4. **Variants через enum**: типобезопасные варианты

#### Базовая структура компонента

```rust
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext)>>,
}

pub enum ButtonVariant {
    Default,
    Outline,
    Secondary,
    Ghost,
    Destructive,
    Link,
}

pub enum ButtonSize {
    Sm,
    Md,
    Lg,
    Icon,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "".into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Md,
            disabled: false,
            on_click: None,
        }
    }
    
    // Fluent API
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
    
    // Shortcuts для вариантов
    pub fn primary(self) -> Self {
        self.variant(ButtonVariant::Default)
    }
    
    pub fn outline(self) -> Self {
        self.variant(ButtonVariant::Outline)
    }
    
    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }
    
    pub fn destructive(self) -> Self {
        self.variant(ButtonVariant::Destructive)
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        let (bg, fg, border) = match self.variant {
            ButtonVariant::Default => (
                theme.colors.primary,
                theme.colors.primary_foreground,
                theme.colors.primary,
            ),
            ButtonVariant::Outline => (
                theme.colors.background,
                theme.colors.foreground,
                theme.colors.border,
            ),
            ButtonVariant::Secondary => (
                theme.colors.secondary,
                theme.colors.secondary_foreground,
                theme.colors.secondary,
            ),
            ButtonVariant::Ghost => (
                Hsla::transparent_black(),
                theme.colors.foreground,
                Hsla::transparent_black(),
            ),
            ButtonVariant::Destructive => (
                theme.colors.destructive,
                theme.colors.destructive_foreground,
                theme.colors.destructive,
            ),
            ButtonVariant::Link => (
                Hsla::transparent_black(),
                theme.colors.primary,
                Hsla::transparent_black(),
            ),
        };
        
        let (height, px, py) = match self.size {
            ButtonSize::Sm => (px(36.), theme.spacing.sm, px(2.)),
            ButtonSize::Md => (px(40.), theme.spacing.md, px(8.)),
            ButtonSize::Lg => (px(44.), theme.spacing.lg, px(10.)),
            ButtonSize::Icon => (px(40.), px(0.), px(0.)),
        };
        
        div()
            .id(self.id)
            .h(height)
            .px(px)
            .py(py)
            .bg(bg)
            .text_color(fg)
            .border_1()
            .border_color(border)
            .rounded(theme.radius.md)
            .cursor_pointer()
            .when(self.disabled, |this| {
                this.cursor_default()
                    .opacity(0.5)
            })
            .when_some(self.on_click, |this, handler| {
                this.on_click(move |event, cx| handler(event, cx))
            })
            .hover(|this| {
                this.opacity(0.9)
            })
            .active(|this| {
                this.opacity(0.8)
            })
            .child(self.label)
    }
}
```

#### Использование

```rust
Button::new("save-btn")
    .label("Save Changes")
    .primary()
    .size(ButtonSize::Lg)
    .on_click(|_, cx| {
        println!("Saved!");
    })
```

#### Compound Component: Card

```rust
#[derive(IntoElement)]
pub struct Card {
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for Card {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .rounded(theme.radius.lg)
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.background)
            .p(theme.spacing.md)
            .shadow_sm()
            .children(self.children)
    }
}

pub struct CardHeader {
    children: Vec<AnyElement>,
}

impl CardHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for CardHeader {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing.xs)
            .pb(theme.spacing.md)
            .children(self.children)
    }
}

pub struct CardTitle {
    text: SharedString,
}

impl CardTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
        }
    }
}

impl RenderOnce for CardTitle {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .text_size(theme.typography.size_lg)
            .font_weight(theme.typography.weight_semibold)
            .text_color(theme.colors.foreground)
            .child(self.text)
    }
}

// CardDescription, CardContent, CardFooter аналогично...
```

#### Использование Card

```rust
Card::new()
    .child(
        CardHeader::new()
            .child(CardTitle::new("User Profile"))
            .child(CardDescription::new("Manage your account settings"))
    )
    .child(
        CardContent::new()
            .child("Your content here...")
    )
    .child(
        CardFooter::new()
            .child(
                Button::new("save")
                    .label("Save")
                    .primary()
            )
            .child(
                Button::new("cancel")
                    .label("Cancel")
                    .outline()
            )
    )
```

### 4.3 Утилиты для стилизации (StyledExt)

```rust
pub trait StyledExt: Styled + Sized {
    // Layout helpers
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }
    
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }
    
    // Gap utilities
    fn gap(self, gap: impl Into<DefiniteLength>) -> Self {
        self.gap(gap)
    }
    
    fn gap_1(self) -> Self {
        self.gap(px(4.))
    }
    
    fn gap_2(self) -> Self {
        self.gap(px(8.))
    }
    
    fn gap_3(self) -> Self {
        self.gap(px(12.))
    }
    
    fn gap_4(self) -> Self {
        self.gap(px(16.))
    }
    
    // Padding utilities (следуя Tailwind паттерну)
    fn p(self, padding: impl Into<DefiniteLength>) -> Self {
        self.p(padding)
    }
    
    fn px(self, padding: impl Into<DefiniteLength>) -> Self {
        self.px(padding)
    }
    
    fn py(self, padding: impl Into<DefiniteLength>) -> Self {
        self.py(padding)
    }
    
    fn p_1(self) -> Self {
        self.p(px(4.))
    }
    
    fn p_2(self) -> Self {
        self.p(px(8.))
    }
    
    fn p_4(self) -> Self {
        self.p(px(16.))
    }
    
    // Border radius utilities
    fn rounded_sm(self) -> Self {
        // Use theme radius
        self.rounded(px(4.))
    }
    
    fn rounded_md(self) -> Self {
        self.rounded(px(8.))
    }
    
    fn rounded_lg(self) -> Self {
        self.rounded(px(12.))
    }
    
    fn rounded_full(self) -> Self {
        self.rounded(px(9999.))
    }
}

// Implement for Div and all Styled types
impl<T: Styled> StyledExt for T {}
```

### 4.4 Размеры компонентов

```rust
pub enum Size {
    Xs,   // Extra small
    Sm,   // Small
    Md,   // Medium (default)
    Lg,   // Large
    Xl,   // Extra large
}

impl Size {
    pub fn to_pixels(&self) -> Pixels {
        match self {
            Size::Xs => px(24.),
            Size::Sm => px(32.),
            Size::Md => px(40.),
            Size::Lg => px(48.),
            Size::Xl => px(56.),
        }
    }
    
    pub fn padding(&self) -> (Pixels, Pixels) {
        match self {
            Size::Xs => (px(8.), px(4.)),
            Size::Sm => (px(12.), px(6.)),
            Size::Md => (px(16.), px(8.)),
            Size::Lg => (px(20.), px(10.)),
            Size::Xl => (px(24.), px(12.)),
        }
    }
    
    pub fn font_size(&self, theme: &Theme) -> Pixels {
        match self {
            Size::Xs => theme.typography.size_xs,
            Size::Sm => theme.typography.size_sm,
            Size::Md => theme.typography.size_base,
            Size::Lg => theme.typography.size_lg,
            Size::Xl => theme.typography.size_xl,
        }
    }
}

pub trait Sizable {
    fn size(self, size: Size) -> Self;
    fn xs(self) -> Self;
    fn sm(self) -> Self;
    fn md(self) -> Self;
    fn lg(self) -> Self;
    fn xl(self) -> Self;
}
```

---

## 5. Приоритетный план разработки (от базового к сложному)

### Фаза 1: Фундамент (Foundation)

#### 1.1 Система тем
- [ ] Определить структуры `Theme`, `ThemeColors`, `Spacing`, `Radius`, `Typography`
- [ ] Реализовать `ThemeRegistry` с загрузкой JSON
- [ ] Создать `ActiveTheme` trait для доступа к теме
- [ ] Создать 2-3 базовые темы (light, dark, shadcn-default)

#### 1.2 Утилиты стилизации
- [ ] Реализовать `StyledExt` trait с утилитами
- [ ] Создать `Size` enum и `Sizable` trait
- [ ] Добавить helper функции для spacing, padding, margin
- [ ] Документировать все утилиты

### Фаза 2: Базовые компоненты (Basic Components)

#### 2.1 Layout компоненты (аналоги Flutter)
- [ ] **Container**: базовый контейнер с padding, margin, decoration
- [ ] **HStack/Row**: горизонтальная раскладка
- [ ] **VStack/Column**: вертикальная раскладка
- [ ] **Spacer**: эластичный разделитель
- [ ] **Divider**: горизонтальная/вертикальная линия

#### 2.2 Текстовые компоненты
- [ ] **Text**: текст с вариантами размеров
- [ ] **Heading** (H1-H6): заголовки
- [ ] **Label**: маленький текст для форм
- [ ] **Code**: моноширинный код

#### 2.3 Кнопки
- [ ] **Button**: основная кнопка со всеми вариантами (default, outline, ghost, destructive, link)
- [ ] **IconButton**: кнопка только с иконкой
- [ ] **ButtonGroup**: группа кнопок

### Фаза 3: Основные UI компоненты (Core UI)

#### 3.1 Карточки и контейнеры
- [ ] **Card** + CardHeader, CardTitle, CardDescription, CardContent, CardFooter
- [ ] **Panel**: простая панель с рамкой
- [ ] **Well**: углубленный контейнер

#### 3.2 Списки
- [ ] **List**: базовый список
- [ ] **ListItem**: элемент списка с иконкой, текстом, action
- [ ] **ScrollView**: прокручиваемая область

#### 3.3 Навигация
- [ ] **Tabs**: вкладки
- [ ] **Breadcrumb**: хлебные крошки
- [ ] **Pagination**: постраничная навигация

### Фаза 4: Формы и инпуты (Forms & Inputs)

#### 4.1 Текстовые поля
- [ ] **Input/TextField**: базовый текстовый инпут
- [ ] **TextArea**: многострочное поле
- [ ] **SearchField**: поле поиска с иконкой

#### 4.2 Выбор
- [ ] **Checkbox**: чекбокс
- [ ] **Radio**: радиокнопка
- [ ] **Switch/Toggle**: переключатель
- [ ] **Slider**: слайдер

#### 4.3 Селекторы
- [ ] **Select/Dropdown**: выпадающий список
- [ ] **Combobox**: комбобокс с поиском
- [ ] **DatePicker**: выбор даты
- [ ] **ColorPicker**: выбор цвета

### Фаза 5: Overlay компоненты (Overlays)

#### 5.1 Модальные окна
- [ ] **Dialog/Modal**: модальное окно
- [ ] **AlertDialog**: диалог подтверждения
- [ ] **Sheet**: боковая панель

#### 5.2 Всплывающие элементы
- [ ] **Popover**: всплывающая подсказка с контентом
- [ ] **Tooltip**: простая подсказка
- [ ] **DropdownMenu**: выпадающее меню
- [ ] **ContextMenu**: контекстное меню

#### 5.3 Уведомления
- [ ] **Toast**: временное уведомление
- [ ] **Alert**: уведомление на странице
- [ ] **Badge**: значок с числом

### Фаза 6: Сложные компоненты (Advanced)

#### 6.1 Данные
- [ ] **Table**: таблица с сортировкой, фильтрацией
- [ ] **DataGrid**: продвинутая таблица
- [ ] **TreeView**: дерево элементов

#### 6.2 Навигация и структура
- [ ] **Sidebar**: боковая панель навигации
- [ ] **AppBar/Toolbar**: верхняя панель приложения
- [ ] **Scaffold**: каркас приложения (аналог Flutter Scaffold)
- [ ] **BottomNavigationBar**: нижняя навигация

#### 6.3 Специализированные
- [ ] **Calendar**: календарь
- [ ] **Chart**: графики (простые)
- [ ] **Progress**: индикаторы прогресса
- [ ] **Skeleton**: скелетон-загрузчики
- [ ] **Avatar**: аватар пользователя
- [ ] **Carousel**: карусель элементов

---

## 6. Ключевые паттерны и best practices

### 6.1 Композиция вместо наследования
```rust
// Хорошо: композиция
Card::new()
    .child(CardHeader::new())
    .child(CardContent::new())

// Плохо: монолитный компонент с флагами
Card::new()
    .with_header(true)
    .header_text("Title")
```

### 6.2 Типобезопасные варианты
```rust
// Хорошо: enum для вариантов
pub enum ButtonVariant {
    Default,
    Outline,
    Ghost,
}

// Плохо: строки
button.variant("outline") // может быть опечатка
```

### 6.3 Fluent API
```rust
// Хорошо: цепочка методов
Button::new("id")
    .label("Click")
    .primary()
    .large()
    .on_click(handler)

// Плохо: множественные вызовы
let mut btn = Button::new("id");
btn.set_label("Click");
btn.set_variant(Variant::Primary);
btn.set_size(Size::Large);
```

### 6.4 Семантические цвета
```rust
// Хорошо: семантические названия
theme.colors.destructive  // для delete кнопок
theme.colors.primary      // для основных действий

// Плохо: прямые цвета
Hsla::red()
Hsla::blue()
```

### 6.5 Доступность (Accessibility)
```rust
Button::new("btn")
    .label("Submit")
    .aria_label("Submit form")  // для screen readers
    .disabled(is_disabled)
    .on_click(handler)
```

### 6.6 Responsive sizing
```rust
// Использовать Size enum вместо хардкода
button.size(Size::Md)

// Адаптировать к экрану
let size = if is_mobile { Size::Sm } else { Size::Md };
```

---

## 7. Сравнительная таблица паттернов

| Аспект | shadcn/ui | GPUI (longbridge) | Zed | Рекомендация для gpui-flutter |
|--------|-----------|-------------------|-----|-------------------------------|
| **Темы** | CSS vars в JSON | JSON → Rust struct | JSON + Registry | JSON → Theme struct + Registry |
| **Цвета** | OKLCH | Hsla | Hsla | Hsla (нативно в GPUI) |
| **Компоненты** | Compound Components | RenderOnce + Render | Component trait | RenderOnce для stateless |
| **Варианты** | CVA (Class Variance) | Enum | Enum | Enum для типобезопасности |
| **Стили** | Tailwind utilities | Fluent API (StyledExt) | Div methods | StyledExt + Div methods |
| **Размеры** | sm/md/lg строки | Size enum | Pixels | Size enum с conversion |
| **Композиция** | Slots (children) | .child() methods | .child() methods | .child() + compound components |

---

## 8. Источники и ссылки

### Документация GPUI проектов
- [longbridge/gpui-component](https://github.com/longbridge/gpui-component) - Самая полная библиотека компонентов
- [zed-industries/zed](https://github.com/zed-industries/zed) - Референсная реализация тем
- [awesome-gpui](https://github.com/zed-industries/awesome-gpui) - Список GPUI проектов

### shadcn/ui документация
- [shadcn/ui официальный сайт](https://ui.shadcn.com/)
- [shadcn/ui theming](https://ui.shadcn.com/docs/theming)
- [Button component](https://ui.shadcn.com/docs/components/button)
- [Card component](https://ui.shadcn.com/docs/components/card)
- [The Anatomy of shadcn/ui Components](https://vercel.com/academy/shadcn-ui/extending-shadcn-ui-with-custom-components)
- [Why shadcn/ui is Different](https://vercel.com/academy/shadcn-ui/why-shadcn-ui-is-different)
- [Building a Scalable Design System with Shadcn/UI, Tailwind CSS, and Design Tokens](https://shadisbaih.medium.com/building-a-scalable-design-system-with-shadcn-ui-tailwind-css-and-design-tokens-031474b03690)

### Flutter документация
- [Material Design for Flutter](https://docs.flutter.dev/ui/design/material)
- [Material component widgets](https://docs.flutter.dev/ui/widgets/material)
- [Theming and Customization in Flutter](https://www.freecodecamp.org/news/theming-and-customization-in-flutter-a-handbook-for-developers/)
- [Decoupling Material and Cupertino in Flutter](https://www.freecodecamp.org/news/decoupling-material-and-cupertino-in-flutter)

---

## 9. Следующие шаги

### 9.1 Немедленные действия
1. ✅ Изучить существующие GPUI проекты - **ЗАВЕРШЕНО**
2. ✅ Проанализировать shadcn/ui дизайн-систему - **ЗАВЕРШЕНО**
3. 🔄 Создать структуру проекта `gpui-flutter`
4. 🔄 Реализовать базовую систему тем

### 9.2 Короткосрочные цели (1-2 недели)
- Реализовать Фазу 1 (Foundation): Theme system + utilities
- Реализовать Фазу 2 (Basic Components): Container, HStack, VStack, Text, Button
- Написать примеры использования (story/gallery app)

### 9.3 Среднесрочные цели (1-2 месяца)
- Реализовать Фазы 3-4 (Core UI + Forms)
- Документация и примеры
- Первый релиз v0.1.0

### 9.4 Долгосрочные цели (3+ месяца)
- Реализовать Фазы 5-6 (Overlays + Advanced)
- Полная документация
- Стабильный API v1.0.0

---

## 10. Вопросы для уточнения

Перед началом разработки стоит решить:

1. **Именование**: 
   - Использовать Flutter названия (Container, Column, Row) или shadcn (Card, Button)?
   - Предложение: Flutter названия для layout, shadcn для UI компонентов

2. **Иконки**:
   - Какую иконочную систему использовать? (Lucide, как в shadcn?)
   - Интеграция через SVG или font icons?

3. **Анимации**:
   - Встроенные анимации переходов (transitions)?
   - GPUI поддерживает анимации через `Animation` API

4. **Доступность**:
   - ARIA атрибуты и поддержка screen readers?
   - Клавиатурная навигация для всех компонентов?

5. **Тестирование**:
   - Unit тесты для компонентов?
   - Visual regression тесты?

---

**Конец документа**

Этот документ является отправной точкой для разработки `gpui-flutter` библиотеки компонентов.

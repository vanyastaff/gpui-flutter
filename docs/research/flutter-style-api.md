# Flutter-Style API для gpui-flutter

Дата: 2026-01-19

---

## Философия API

Вместо многоуровневых `.child()` вызовов, используем **именованные параметры** как во Flutter:

### ❌ Плохо (shadcn-style)
```rust
Card::new()
    .child(CardHeader::new()
        .child(CardTitle::new("Title"))
        .child(CardDescription::new("Description"))
    )
    .child(CardContent::new()
        .child("Content")
    )
    .child(CardFooter::new()
        .child(Button::new("ok"))
    )
```

### ✅ Хорошо (Flutter-style)
```rust
Card::new()
    .header(
        CardHeader::new()
            .title("User Profile")
            .description("Manage your account settings")
    )
    .content(
        CardContent::new()
            .child(Text::new("Your content here..."))
    )
    .footer(
        CardFooter::new()
            .children(vec![
                Button::new("save").label("Save").primary().into_any(),
                Button::new("cancel").label("Cancel").outline().into_any(),
            ])
    )
```

Или еще проще:

```rust
Card::new()
    .title("User Profile")
    .description("Manage your account settings")
    .content(Text::new("Your content here..."))
    .footer_buttons(vec![
        Button::new("save").label("Save").primary(),
        Button::new("cancel").label("Cancel").outline(),
    ])
```

---

## Примеры компонентов с Flutter API

### Card Component

```rust
#[derive(IntoElement)]
pub struct Card {
    // Optional slots
    title: Option<SharedString>,
    description: Option<SharedString>,
    header: Option<AnyElement>,
    content: Option<AnyElement>,
    footer: Option<AnyElement>,
    
    // Styling
    padding: Option<Pixels>,
    elevation: Elevation,
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            header: None,
            content: None,
            footer: None,
            padding: None,
            elevation: Elevation::Low,
        }
    }
    
    // Простые методы для текста
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }
    
    // Именованные слоты
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }
    
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
    
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
    
    // Convenience методы
    pub fn footer_buttons(mut self, buttons: Vec<impl IntoElement>) -> Self {
        let footer = Row::new()
            .gap(8.)
            .children(buttons.into_iter().map(|b| b.into_any_element()).collect());
        self.footer = Some(footer.into_any_element());
        self
    }
    
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = Some(padding.into());
        self
    }
    
    pub fn elevation(mut self, elevation: Elevation) -> Self {
        self.elevation = elevation;
        self
    }
}

impl RenderOnce for Card {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        Column::new()
            .rounded(theme.radius.lg)
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.background)
            .shadow(self.elevation.to_shadow())
            .overflow_hidden()
            // Header section
            .when(self.title.is_some() || self.description.is_some() || self.header.is_some(), |col| {
                col.child(
                    Column::new()
                        .p(theme.spacing.lg)
                        .gap(theme.spacing.xs)
                        .when_some(self.title.clone(), |col, title| {
                            col.child(
                                Text::new(title)
                                    .size(theme.typography.size_lg)
                                    .weight(theme.typography.weight_semibold)
                                    .color(theme.colors.foreground)
                            )
                        })
                        .when_some(self.description.clone(), |col, desc| {
                            col.child(
                                Text::new(desc)
                                    .size(theme.typography.size_sm)
                                    .color(theme.colors.muted_foreground)
                            )
                        })
                        .when_some(self.header, |col, header| {
                            col.child(header)
                        })
                )
            })
            // Content section
            .when_some(self.content, |col, content| {
                col.child(
                    Container::new()
                        .p(self.padding.unwrap_or(theme.spacing.lg))
                        .child(content)
                )
            })
            // Footer section
            .when_some(self.footer, |col, footer| {
                col.child(
                    Container::new()
                        .p(theme.spacing.lg)
                        .border_t_1()
                        .border_color(theme.colors.border)
                        .child(footer)
                )
            })
    }
}

pub enum Elevation {
    None,
    Low,
    Medium,
    High,
}

impl Elevation {
    fn to_shadow(&self) -> BoxShadow {
        match self {
            Elevation::None => BoxShadow::default(),
            Elevation::Low => BoxShadow {
                blur: px(4.),
                spread: px(0.),
                offset: point(px(0.), px(2.)),
                color: hsla(0., 0., 0., 0.1),
            },
            Elevation::Medium => BoxShadow {
                blur: px(8.),
                spread: px(0.),
                offset: point(px(0.), px(4.)),
                color: hsla(0., 0., 0., 0.15),
            },
            Elevation::High => BoxShadow {
                blur: px(16.),
                spread: px(0.),
                offset: point(px(0.), px(8.)),
                color: hsla(0., 0., 0., 0.2),
            },
        }
    }
}
```

### Использование Card (Flutter-style)

```rust
// Вариант 1: Простой (авто header)
Card::new()
    .title("Settings")
    .description("Manage your preferences")
    .content(
        Column::new()
            .gap(16.)
            .child(Text::new("Some settings here"))
    )
    .footer_buttons(vec![
        Button::new("save").label("Save").primary(),
        Button::new("cancel").label("Cancel").outline(),
    ])

// Вариант 2: Кастомный header
Card::new()
    .header(
        Row::new()
            .justify_between()
            .child(Text::new("Custom Header").bold())
            .child(IconButton::new("close").icon(Icon::X))
    )
    .content(MyWidget::new())
    .footer(
        Row::new()
            .gap(8.)
            .justify_end()
            .child(Button::new("ok").label("OK"))
    )

// Вариант 3: Минимальный
Card::new()
    .content(Text::new("Just content"))
```

---

## Container (аналог Flutter Container)

```rust
#[derive(IntoElement)]
pub struct Container {
    child: Option<AnyElement>,
    
    // Dimensions
    width: Option<Length>,
    height: Option<Length>,
    
    // Padding & Margin
    padding: EdgeInsets,
    margin: EdgeInsets,
    
    // Decoration
    background: Option<Hsla>,
    border: BorderStyle,
    border_radius: BorderRadius,
    
    // Shadow
    shadow: Option<BoxShadow>,
    
    // Alignment
    alignment: Alignment,
}

#[derive(Default, Clone)]
pub struct EdgeInsets {
    pub top: Pixels,
    pub right: Pixels,
    pub bottom: Pixels,
    pub left: Pixels,
}

impl EdgeInsets {
    pub fn all(value: f32) -> Self {
        Self {
            top: px(value),
            right: px(value),
            bottom: px(value),
            left: px(value),
        }
    }
    
    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: px(vertical),
            bottom: px(vertical),
            left: px(horizontal),
            right: px(horizontal),
        }
    }
    
    pub fn only(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: px(top),
            right: px(right),
            bottom: px(bottom),
            left: px(left),
        }
    }
}

#[derive(Clone)]
pub struct BorderRadius {
    pub top_left: Pixels,
    pub top_right: Pixels,
    pub bottom_right: Pixels,
    pub bottom_left: Pixels,
}

impl BorderRadius {
    pub fn all(radius: f32) -> Self {
        Self {
            top_left: px(radius),
            top_right: px(radius),
            bottom_right: px(radius),
            bottom_left: px(radius),
        }
    }
    
    pub fn circular(radius: f32) -> Self {
        Self::all(radius)
    }
    
    pub fn only(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
        Self {
            top_left: px(top_left),
            top_right: px(top_right),
            bottom_right: px(bottom_right),
            bottom_left: px(bottom_left),
        }
    }
}

pub enum Alignment {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Container {
    pub fn new() -> Self {
        Self {
            child: None,
            width: None,
            height: None,
            padding: EdgeInsets::default(),
            margin: EdgeInsets::default(),
            background: None,
            border: BorderStyle::default(),
            border_radius: BorderRadius::all(0.),
            shadow: None,
            alignment: Alignment::TopLeft,
        }
    }
    
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
    
    // Dimensions
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }
    
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }
    
    pub fn size(self, width: impl Into<Length>, height: impl Into<Length>) -> Self {
        self.width(width).height(height)
    }
    
    // Padding
    pub fn padding(mut self, padding: EdgeInsets) -> Self {
        self.padding = padding;
        self
    }
    
    pub fn p(mut self, value: f32) -> Self {
        self.padding = EdgeInsets::all(value);
        self
    }
    
    pub fn px(mut self, value: f32) -> Self {
        self.padding.left = px(value);
        self.padding.right = px(value);
        self
    }
    
    pub fn py(mut self, value: f32) -> Self {
        self.padding.top = px(value);
        self.padding.bottom = px(value);
        self
    }
    
    // Margin
    pub fn margin(mut self, margin: EdgeInsets) -> Self {
        self.margin = margin;
        self
    }
    
    pub fn m(mut self, value: f32) -> Self {
        self.margin = EdgeInsets::all(value);
        self
    }
    
    // Decoration
    pub fn background(mut self, color: impl Into<Hsla>) -> Self {
        self.background = Some(color.into());
        self
    }
    
    pub fn bg(mut self, color: impl Into<Hsla>) -> Self {
        self.background(color)
    }
    
    pub fn border_radius(mut self, radius: BorderRadius) -> Self {
        self.border_radius = radius;
        self
    }
    
    pub fn rounded(mut self, radius: f32) -> Self {
        self.border_radius = BorderRadius::all(radius);
        self
    }
    
    pub fn shadow(mut self, shadow: BoxShadow) -> Self {
        self.shadow = Some(shadow);
        self
    }
    
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl RenderOnce for Container {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let mut element = div();
        
        // Apply dimensions
        if let Some(width) = self.width {
            element = element.w(width);
        }
        if let Some(height) = self.height {
            element = element.h(height);
        }
        
        // Apply margin
        element = element
            .mt(self.margin.top)
            .mr(self.margin.right)
            .mb(self.margin.bottom)
            .ml(self.margin.left);
        
        // Apply padding
        element = element
            .pt(self.padding.top)
            .pr(self.padding.right)
            .pb(self.padding.bottom)
            .pl(self.padding.left);
        
        // Apply decoration
        if let Some(bg) = self.background {
            element = element.bg(bg);
        }
        
        element = element.rounded_corners_px([
            self.border_radius.top_left.0,
            self.border_radius.top_right.0,
            self.border_radius.bottom_right.0,
            self.border_radius.bottom_left.0,
        ]);
        
        // Apply shadow
        if let Some(shadow) = self.shadow {
            element = element.shadow(shadow);
        }
        
        // Apply alignment and child
        if let Some(child) = self.child {
            element = match self.alignment {
                Alignment::TopLeft => element.items_start().justify_start(),
                Alignment::TopCenter => element.items_center().justify_start(),
                Alignment::TopRight => element.items_end().justify_start(),
                Alignment::CenterLeft => element.items_start().justify_center(),
                Alignment::Center => element.items_center().justify_center(),
                Alignment::CenterRight => element.items_end().justify_center(),
                Alignment::BottomLeft => element.items_start().justify_end(),
                Alignment::BottomCenter => element.items_center().justify_end(),
                Alignment::BottomRight => element.items_end().justify_end(),
            };
            element = element.child(child);
        }
        
        element
    }
}
```

### Использование Container

```rust
// Простой контейнер
Container::new()
    .padding(EdgeInsets::all(16.))
    .background(theme.colors.background)
    .rounded(8.)
    .child(Text::new("Content"))

// С размерами
Container::new()
    .width(px(200.))
    .height(px(100.))
    .p(16.)
    .bg(theme.colors.primary)
    .child(Text::new("Fixed size"))

// С margin и alignment
Container::new()
    .margin(EdgeInsets::symmetric(16., 8.))
    .padding(EdgeInsets::all(12.))
    .alignment(Alignment::Center)
    .child(Text::new("Centered"))

// С тенью и border radius
Container::new()
    .p(20.)
    .bg(theme.colors.background)
    .border_radius(BorderRadius::all(12.))
    .shadow(BoxShadow {
        blur: px(8.),
        spread: px(0.),
        offset: point(px(0.), px(4.)),
        color: hsla(0., 0., 0., 0.1),
    })
    .child(MyWidget::new())
```

---

## Row & Column (Layout)

```rust
#[derive(IntoElement)]
pub struct Row {
    children: Vec<AnyElement>,
    gap: Pixels,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
    wrap: bool,
}

#[derive(IntoElement)]
pub struct Column {
    children: Vec<AnyElement>,
    gap: Pixels,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
}

pub enum MainAxisAlignment {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub enum CrossAxisAlignment {
    Start,
    End,
    Center,
    Stretch,
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: px(0.),
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Center,
            wrap: false,
        }
    }
    
    pub fn children(mut self, children: Vec<AnyElement>) -> Self {
        self.children = children;
        self
    }
    
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = px(gap);
        self
    }
    
    pub fn main_axis_alignment(mut self, alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = alignment;
        self
    }
    
    pub fn cross_axis_alignment(mut self, alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = alignment;
        self
    }
    
    // Shortcuts
    pub fn space_between(self) -> Self {
        self.main_axis_alignment(MainAxisAlignment::SpaceBetween)
    }
    
    pub fn center(self) -> Self {
        self.main_axis_alignment(MainAxisAlignment::Center)
            .cross_axis_alignment(CrossAxisAlignment::Center)
    }
    
    pub fn justify_end(self) -> Self {
        self.main_axis_alignment(MainAxisAlignment::End)
    }
    
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
}

impl RenderOnce for Row {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let mut element = div().flex().flex_row();
        
        // Apply gap
        if self.gap.0 > 0. {
            element = element.gap(self.gap);
        }
        
        // Apply main axis alignment
        element = match self.main_axis_alignment {
            MainAxisAlignment::Start => element.justify_start(),
            MainAxisAlignment::End => element.justify_end(),
            MainAxisAlignment::Center => element.justify_center(),
            MainAxisAlignment::SpaceBetween => element.justify_between(),
            MainAxisAlignment::SpaceAround => element.justify_around(),
            MainAxisAlignment::SpaceEvenly => element.justify_evenly(),
        };
        
        // Apply cross axis alignment
        element = match self.cross_axis_alignment {
            CrossAxisAlignment::Start => element.items_start(),
            CrossAxisAlignment::End => element.items_end(),
            CrossAxisAlignment::Center => element.items_center(),
            CrossAxisAlignment::Stretch => element.items_stretch(),
        };
        
        // Apply wrap
        if self.wrap {
            element = element.flex_wrap();
        }
        
        // Add children
        element.children(self.children)
    }
}

// Column аналогично, но с flex_col вместо flex_row
```

### Использование Row/Column

```rust
// Row
Row::new()
    .gap(16.)
    .space_between()
    .children(vec![
        Button::new("ok").label("OK").into_any(),
        Button::new("cancel").label("Cancel").into_any(),
    ])

// Column
Column::new()
    .gap(8.)
    .cross_axis_alignment(CrossAxisAlignment::Stretch)
    .child(Text::new("Title").bold())
    .child(Text::new("Description"))
    .child(Button::new("action").label("Click Me"))

// Вложенные layouts
Column::new()
    .gap(16.)
    .child(
        Row::new()
            .space_between()
            .child(Text::new("Settings"))
            .child(IconButton::new("close").icon(Icon::X))
    )
    .child(
        Column::new()
            .gap(8.)
            .child(TextField::new("name").label("Name"))
            .child(TextField::new("email").label("Email"))
    )
    .child(
        Row::new()
            .gap(8.)
            .justify_end()
            .child(Button::new("save").label("Save").primary())
            .child(Button::new("cancel").label("Cancel").outline())
    )
```

---

## TextField (Input)

```rust
#[derive(IntoElement)]
pub struct TextField {
    id: ElementId,
    
    // Value
    value: SharedString,
    placeholder: Option<SharedString>,
    
    // Label & Helper
    label: Option<SharedString>,
    helper_text: Option<SharedString>,
    error_text: Option<SharedString>,
    
    // Leading & Trailing
    leading_icon: Option<Icon>,
    trailing_icon: Option<Icon>,
    prefix: Option<SharedString>,
    suffix: Option<SharedString>,
    
    // State
    disabled: bool,
    readonly: bool,
    required: bool,
    
    // Styling
    variant: TextFieldVariant,
    size: Size,
    
    // Callbacks
    on_change: Option<Box<dyn Fn(String, &mut WindowContext)>>,
    on_submit: Option<Box<dyn Fn(String, &mut WindowContext)>>,
}

pub enum TextFieldVariant {
    Outlined,
    Filled,
    Underlined,
}

impl TextField {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "".into(),
            placeholder: None,
            label: None,
            helper_text: None,
            error_text: None,
            leading_icon: None,
            trailing_icon: None,
            prefix: None,
            suffix: None,
            disabled: false,
            readonly: false,
            required: false,
            variant: TextFieldVariant::Outlined,
            size: Size::Md,
            on_change: None,
            on_submit: None,
        }
    }
    
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }
    
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.label = Some(text.into());
        self
    }
    
    pub fn helper_text(mut self, text: impl Into<SharedString>) -> Self {
        self.helper_text = Some(text.into());
        self
    }
    
    pub fn error(mut self, text: impl Into<SharedString>) -> Self {
        self.error_text = Some(text.into());
        self
    }
    
    pub fn leading_icon(mut self, icon: Icon) -> Self {
        self.leading_icon = Some(icon);
        self
    }
    
    pub fn trailing_icon(mut self, icon: Icon) -> Self {
        self.trailing_icon = Some(icon);
        self
    }
    
    pub fn prefix(mut self, text: impl Into<SharedString>) -> Self {
        self.prefix = Some(text.into());
        self
    }
    
    pub fn suffix(mut self, text: impl Into<SharedString>) -> Self {
        self.suffix = Some(text.into());
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }
    
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    
    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    
    pub fn on_change(mut self, handler: impl Fn(String, &mut WindowContext) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
    
    pub fn on_submit(mut self, handler: impl Fn(String, &mut WindowContext) + 'static) -> Self {
        self.on_submit = Some(Box::new(handler));
        self
    }
}
```

### Использование TextField

```rust
// Простой input
TextField::new("email")
    .label("Email")
    .placeholder("Enter your email")
    .on_change(|value, cx| {
        println!("Changed: {}", value);
    })

// С иконками и helper text
TextField::new("search")
    .placeholder("Search...")
    .leading_icon(Icon::Search)
    .trailing_icon(Icon::X)
    .helper_text("Press Enter to search")
    .on_submit(|query, cx| {
        // Perform search
    })

// С ошибкой
TextField::new("password")
    .label("Password")
    .error("Password must be at least 8 characters")
    .required(true)

// С prefix/suffix
TextField::new("price")
    .label("Price")
    .prefix("$")
    .suffix("USD")
```

---

## Scaffold (Flutter Scaffold)

```rust
#[derive(IntoElement)]
pub struct Scaffold {
    // Slots
    app_bar: Option<AnyElement>,
    body: Option<AnyElement>,
    bottom_navigation_bar: Option<AnyElement>,
    floating_action_button: Option<AnyElement>,
    drawer: Option<AnyElement>,
    
    // Background
    background_color: Option<Hsla>,
}

impl Scaffold {
    pub fn new() -> Self {
        Self {
            app_bar: None,
            body: None,
            bottom_navigation_bar: None,
            floating_action_button: None,
            drawer: None,
            background_color: None,
        }
    }
    
    pub fn app_bar(mut self, app_bar: impl IntoElement) -> Self {
        self.app_bar = Some(app_bar.into_any_element());
        self
    }
    
    pub fn body(mut self, body: impl IntoElement) -> Self {
        self.body = Some(body.into_any_element());
        self
    }
    
    pub fn bottom_navigation_bar(mut self, bar: impl IntoElement) -> Self {
        self.bottom_navigation_bar = Some(bar.into_any_element());
        self
    }
    
    pub fn floating_action_button(mut self, fab: impl IntoElement) -> Self {
        self.floating_action_button = Some(fab.into_any_element());
        self
    }
    
    pub fn drawer(mut self, drawer: impl IntoElement) -> Self {
        self.drawer = Some(drawer.into_any_element());
        self
    }
    
    pub fn background_color(mut self, color: impl Into<Hsla>) -> Self {
        self.background_color = Some(color.into());
        self
    }
}

impl RenderOnce for Scaffold {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        
        Column::new()
            .size_full()
            .bg(self.background_color.unwrap_or(theme.colors.background))
            // AppBar
            .when_some(self.app_bar, |col, app_bar| {
                col.child(app_bar)
            })
            // Body
            .child(
                Container::new()
                    .flex_1()
                    .overflow_hidden()
                    .when_some(self.body, |container, body| {
                        container.child(body)
                    })
            )
            // Bottom Navigation Bar
            .when_some(self.bottom_navigation_bar, |col, bottom_bar| {
                col.child(bottom_bar)
            })
            // FAB (positioned absolutely)
            .when_some(self.floating_action_button, |col, fab| {
                col.child(
                    Container::new()
                        .absolute()
                        .bottom(px(16.))
                        .right(px(16.))
                        .child(fab)
                )
            })
    }
}
```

### Использование Scaffold

```rust
Scaffold::new()
    .app_bar(
        AppBar::new()
            .title("My App")
            .leading(IconButton::new("menu").icon(Icon::Menu))
            .actions(vec![
                IconButton::new("search").icon(Icon::Search).into_any(),
                IconButton::new("settings").icon(Icon::Settings).into_any(),
            ])
    )
    .body(
        Column::new()
            .p(16.)
            .gap(16.)
            .child(Text::new("Welcome!").bold().size(24.))
            .child(Card::new()
                .title("Card 1")
                .content(Text::new("Content here"))
            )
    )
    .floating_action_button(
        FloatingActionButton::new("add")
            .icon(Icon::Plus)
            .on_click(|_, cx| {
                // Add action
            })
    )
    .bottom_navigation_bar(
        BottomNavigationBar::new()
            .items(vec![
                BottomNavItem::new("home").icon(Icon::Home).label("Home"),
                BottomNavItem::new("search").icon(Icon::Search).label("Search"),
                BottomNavItem::new("profile").icon(Icon::User).label("Profile"),
            ])
            .current_index(0)
    )
```

---

## Сравнение подходов

### Child-based (shadcn-style)
```rust
Card::new()
    .child(CardHeader::new()
        .child(CardTitle::new("Title"))
    )
    .child(CardContent::new()
        .child(Text::new("Content"))
    )
```

**Минусы:**
- Глубокая вложенность
- Неясно, что где находится
- Много `.child()` вызовов

### Named slots (Flutter-style) ✅
```rust
Card::new()
    .title("Title")
    .content(Text::new("Content"))
```

**Плюсы:**
- Плоская структура
- Понятные именованные параметры
- Близко к Flutter API
- Опциональные части (не указал — не будет)

---

## Именование компонентов

### Layout
- `Container` (вместо `div`)
- `Row` (вместо `HStack`)
- `Column` (вместо `VStack`)
- `Stack` (для наложения слоев)
- `Spacer` (эластичный разделитель)
- `Divider` (линия-разделитель)
- `Expanded` (занимает доступное место)
- `Flexible` (гибкий размер)

### Material Components
- `Scaffold`
- `AppBar`
- `Card`
- `ListTile`
- `Drawer`
- `BottomNavigationBar`
- `FloatingActionButton`

### Inputs
- `TextField`
- `Checkbox`
- `Radio`
- `Switch`
- `Slider`
- `DropdownButton`

### Buttons
- `Button` (с вариантами: primary, outlined, text)
- `IconButton`
- `FloatingActionButton`

### Text
- `Text`
- `RichText`
- `SelectableText`

### Dialogs & Overlays
- `Dialog`
- `AlertDialog`
- `BottomSheet`
- `SnackBar`
- `Tooltip`

---

## Конец документа

Все компоненты следуют Flutter-style API с именованными методами вместо многоуровневых `.child()`.

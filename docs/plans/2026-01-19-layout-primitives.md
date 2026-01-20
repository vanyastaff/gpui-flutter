# Layout Primitives Implementation Plan

**Goal:** Implement basic Flutter-style layout primitives that will be the foundation for all other components.

**Architecture:** Simple wrapper components that provide Flutter-style API over GPUI's layout system.

---

## Components to Implement

### 1. Padding
Adds padding around a child element.

```rust
Padding::new(child)
    .all(16.0)
    // or
    .symmetric_h(16.0)
    .symmetric_v(8.0)
    // or
    .only_left(8.0)
    .only_top(4.0)
```

### 2. Align
Aligns a child element within its parent.

```rust
Align::new(child)
    .center()
    // or
    .top_left()
    .bottom_right()
    // or
    .alignment(Alignment::CenterLeft)
```

### 3. SizedBox
Fixed size container, or spacer when no child.

```rust
SizedBox::new()
    .width(100.0)
    .height(50.0)
    .child(content)

// As spacer
SizedBox::new().width(16.0) // horizontal spacer
SizedBox::new().height(8.0) // vertical spacer
```

### 4. Center
Centers its child (shorthand for Align::center).

```rust
Center::new(child)
```

### 5. Expanded / Flexible
Takes available space in Row/Column.

```rust
Expanded::new(child)

Flexible::new(child)
    .flex(2)
```

### 6. Spacer
Empty space that expands to fill available space.

```rust
Spacer::new()      // flex = 1
Spacer::new().flex(2)
```

---

## Implementation Order

1. **Padding** - simplest, just adds padding
2. **SizedBox** - fixed dimensions
3. **Align** - alignment within parent
4. **Center** - shorthand for Align
5. **Spacer** - flexible spacing
6. **Expanded/Flexible** - flexible sizing

---

## Task 1: Create layout module

**Files:**
- Create: `crates/ui/src/layout/mod.rs`
- Create: `crates/ui/src/layout/padding.rs`
- Create: `crates/ui/src/layout/sized_box.rs`
- Create: `crates/ui/src/layout/align.rs`
- Create: `crates/ui/src/layout/center.rs`
- Create: `crates/ui/src/layout/spacer.rs`
- Create: `crates/ui/src/layout/flexible.rs`

**Step 1: Create module structure**

```rust
// crates/ui/src/layout/mod.rs

mod padding;
mod sized_box;
mod align;
mod center;
mod spacer;
mod flexible;

pub use padding::Padding;
pub use sized_box::SizedBox;
pub use align::{Align, Alignment};
pub use center::Center;
pub use spacer::Spacer;
pub use flexible::{Expanded, Flexible};
```

**Step 2: Add to lib.rs**

```rust
pub mod layout;
```

**Step 3: Add to prelude**

```rust
pub use crate::layout::*;
```

---

## Task 2: Implement Padding

**File:** `crates/ui/src/layout/padding.rs`

```rust
use gpui::*;

/// Flutter-style Padding widget
#[derive(IntoElement)]
pub struct Padding {
    child: AnyElement,
    padding: Edges,
}

#[derive(Clone, Copy, Default)]
struct Edges {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

impl Padding {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            padding: Edges::default(),
        }
    }
    
    /// Set padding on all sides
    pub fn all(mut self, value: f32) -> Self {
        self.padding = Edges {
            top: value,
            right: value,
            bottom: value,
            left: value,
        };
        self
    }
    
    /// Set horizontal padding (left and right)
    pub fn symmetric_h(mut self, value: f32) -> Self {
        self.padding.left = value;
        self.padding.right = value;
        self
    }
    
    /// Set vertical padding (top and bottom)
    pub fn symmetric_v(mut self, value: f32) -> Self {
        self.padding.top = value;
        self.padding.bottom = value;
        self
    }
    
    /// Set symmetric padding (horizontal and vertical)
    pub fn symmetric(self, h: f32, v: f32) -> Self {
        self.symmetric_h(h).symmetric_v(v)
    }
    
    /// Set padding for specific edges
    pub fn only(mut self) -> PaddingBuilder {
        PaddingBuilder {
            padding: &mut self,
        }
    }
    
    pub fn only_left(mut self, value: f32) -> Self {
        self.padding.left = value;
        self
    }
    
    pub fn only_right(mut self, value: f32) -> Self {
        self.padding.right = value;
        self
    }
    
    pub fn only_top(mut self, value: f32) -> Self {
        self.padding.top = value;
        self
    }
    
    pub fn only_bottom(mut self, value: f32) -> Self {
        self.padding.bottom = value;
        self
    }
}

impl RenderOnce for Padding {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .pt(px(self.padding.top))
            .pr(px(self.padding.right))
            .pb(px(self.padding.bottom))
            .pl(px(self.padding.left))
            .child(self.child)
    }
}

// Helper for chaining only_* methods
pub struct PaddingBuilder<'a> {
    padding: &'a mut Padding,
}

impl<'a> PaddingBuilder<'a> {
    pub fn left(self, value: f32) -> Self {
        self.padding.padding.left = value;
        self
    }
    
    pub fn right(self, value: f32) -> Self {
        self.padding.padding.right = value;
        self
    }
    
    pub fn top(self, value: f32) -> Self {
        self.padding.padding.top = value;
        self
    }
    
    pub fn bottom(self, value: f32) -> Self {
        self.padding.padding.bottom = value;
        self
    }
}
```

---

## Task 3: Implement SizedBox

**File:** `crates/ui/src/layout/sized_box.rs`

```rust
use gpui::*;

/// Flutter-style SizedBox widget
#[derive(IntoElement)]
pub struct SizedBox {
    width: Option<f32>,
    height: Option<f32>,
    child: Option<AnyElement>,
}

impl SizedBox {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            child: None,
        }
    }
    
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
    
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
    
    pub fn size(self, width: f32, height: f32) -> Self {
        self.width(width).height(height)
    }
    
    pub fn square(self, size: f32) -> Self {
        self.width(size).height(size)
    }
    
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
    
    /// Shrink to fit child
    pub fn shrink() -> Self {
        Self::new()
    }
    
    /// Expand to fill parent
    pub fn expand() -> SizedBoxExpand {
        SizedBoxExpand
    }
}

impl Default for SizedBox {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SizedBox {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut element = div();
        
        if let Some(w) = self.width {
            element = element.w(px(w));
        }
        
        if let Some(h) = self.height {
            element = element.h(px(h));
        }
        
        if let Some(child) = self.child {
            element = element.child(child);
        }
        
        element
    }
}

/// Expanded sized box (fill parent)
pub struct SizedBoxExpand;

impl IntoElement for SizedBoxExpand {
    type Element = Div;
    
    fn into_element(self) -> Self::Element {
        div().w_full().h_full()
    }
}
```

---

## Task 4: Implement Align

**File:** `crates/ui/src/layout/align.rs`

```rust
use gpui::*;

/// Flutter-style Align widget
#[derive(IntoElement)]
pub struct Align {
    child: AnyElement,
    alignment: Alignment,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Align {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            alignment: Alignment::Center,
        }
    }
    
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
    
    // Shortcuts
    pub fn top_left(self) -> Self {
        self.alignment(Alignment::TopLeft)
    }
    
    pub fn top_center(self) -> Self {
        self.alignment(Alignment::TopCenter)
    }
    
    pub fn top_right(self) -> Self {
        self.alignment(Alignment::TopRight)
    }
    
    pub fn center_left(self) -> Self {
        self.alignment(Alignment::CenterLeft)
    }
    
    pub fn center(self) -> Self {
        self.alignment(Alignment::Center)
    }
    
    pub fn center_right(self) -> Self {
        self.alignment(Alignment::CenterRight)
    }
    
    pub fn bottom_left(self) -> Self {
        self.alignment(Alignment::BottomLeft)
    }
    
    pub fn bottom_center(self) -> Self {
        self.alignment(Alignment::BottomCenter)
    }
    
    pub fn bottom_right(self) -> Self {
        self.alignment(Alignment::BottomRight)
    }
}

impl RenderOnce for Align {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let element = div().flex();
        
        let element = match self.alignment {
            Alignment::TopLeft => element.justify_start().items_start(),
            Alignment::TopCenter => element.justify_center().items_start(),
            Alignment::TopRight => element.justify_end().items_start(),
            Alignment::CenterLeft => element.justify_start().items_center(),
            Alignment::Center => element.justify_center().items_center(),
            Alignment::CenterRight => element.justify_end().items_center(),
            Alignment::BottomLeft => element.justify_start().items_end(),
            Alignment::BottomCenter => element.justify_center().items_end(),
            Alignment::BottomRight => element.justify_end().items_end(),
        };
        
        element.child(self.child)
    }
}
```

---

## Task 5: Implement Center, Spacer, Flexible

Implement remaining simple components following the same pattern.

---

## Testing

Create simple test in gallery to verify each component works.

```rust
// In gallery
Padding::new(
    SizedBox::new()
        .width(100.0)
        .height(100.0)
        .child(Center::new("Centered"))
)
.all(16.0)
```

---

## Success Criteria

- ✅ All layout primitives compile
- ✅ Flutter-style API works
- ✅ Can be composed together
- ✅ Work with GPUI's layout system

Modules
_ownership_and_data_flow
In GPUI, every model or view in the application is actually owned by a single top-level object called the App. When a new entity or view is created (referred to collectively as entities), the application is given ownership of their state to enable their participation in a variety of app services and interaction with other entities.
colors
The default colors used by GPUI.
inspector_reflection
Provides definitions used by #[derive_inspector_reflection].
prelude
The GPUI prelude is a collection of traits and types that are widely used throughout the library. It is recommended to import this prelude into your application to avoid having to import each trait individually.
styled_reflection
Implements function reflection
Macros
actions
Defines and registers unit structs that can be used as actions. For more complex data types, derive Action.
border_style_methods
Generates methods for border styles.
box_shadow_style_methods
Generates methods for box shadow styles.
cursor_style_methods
Generates methods for cursor styles.
margin_style_methods
Generates methods for margin styles.
overflow_style_methods
Generates methods for overflow styles.
padding_style_methods
Generates methods for padding styles.
position_style_methods
Generates methods for position styles.
register_action
This can be used to register an action with the GPUI runtime when you want to manually implement the Action trait. Typically you should use the Action derive macro or actions! macro instead.
visibility_style_methods
Generates methods for visibility styles.
Structs
Anchored
An anchored element that can be used to display UI that will avoid overflowing the window bounds.
AnchoredState
The state that the anchored element element uses to track its children.
Animation
An animation that can be applied to an element.
AnimationElement
A GPUI element that applies an animation to another element
AnyDrag
Contains state associated with an active drag operation, started by dragging an element within the window or by dragging into the app from the underlying platform.
AnyElement
A dynamically typed element that can be used to store any element type.
AnyEntity
A dynamically typed reference to a entity, which can be downcast into a Entity<T>.
AnyImageCache
A dynamically typed image cache, which can be used to store any image cache
AnyTooltip
Contains state associated with a tooltip. You’ll only need this struct if you’re implementing tooltip behavior on a custom element. Otherwise, use Div::tooltip.
AnyView
A dynamically-typed handle to a view, which can be downcast to a Entity for a specific type.
AnyWeakEntity
A type erased, weak reference to a entity.
AnyWeakView
A weak, dynamically-typed view handle that does not prevent the view from being released.
AnyWindowHandle
A handle to a window with any root view type, which can be downcast to a window with a specific root view type.
App
Contains the state of the full application, and passed as a reference to a variety of callbacks. Other Context derefs to this type. You need a reference to an App to access the state of a Entity.
Application
A reference to a GPUI application, typically constructed in the main function of your app. You won’t interact with this type much outside of initial configuration and startup.
ArenaClearNeeded
Returned when the element arena has been used and so must be cleared before the next draw.
AsyncApp
An async-friendly version of App with a static lifetime so it can be held across await points in async code. You’re provided with an instance when calling App::spawn, and you can also create one with App::to_async. Internally, this holds a weak reference to an App, so its methods are fallible to protect against cases where the App is dropped.
AsyncWindowContext
A cloneable, owned handle to the application context, composed with the window associated with the current task.
Background
A background color, which can be either a solid color or a linear gradient.
BackgroundExecutor
A pointer to the executor that is currently running, for spawning background tasks.
BindingIndex
Index of a binding within a keymap.
Boundary
A boundary between two lines of text.
Bounds
Represents a rectangular area in a 2D space with an origin point and a size.
BoundsRefinement
A refinable version of [#ident], see that documentation for details.
BoxShadow
The possible values of the box-shadow property
Canvas
A canvas element, meant for accessing the low level paint API without defining a whole custom element
Capslock
The state of the capslock key at some point in time
Cascade
A cascade of refinements that can be merged in priority order.
CascadeSlot
A handle to a specific slot in a cascade.
ClipboardItem
A clipboard item that should be copied to the clipboard
ClipboardString
A clipboard item that should be copied to the clipboard
ContentMask
Indicates which region of the window is visible. Content falling outside of this mask will not be rendered. Currently, only rectangular content masks are supported, but we give the mask its own type to leave room to support more complex shapes in the future.
Context
The app context, with specialized behavior for the given entity.
ContextEntry
An entry in a KeyContext
Corners
Represents the corners of a box in a 2D space, such as border radius.
CornersRefinement
A refinable version of [#ident], see that documentation for details.
DebugBelow
Use this struct for interfacing with the ‘debug_below’ styling from your own elements. If a parent element has this style set on it, then this struct will be set as a global in GPUI.
DecorationRun
Set the text decoration for a run of text.
Deferred
An element which delays the painting of its child until after all of its ancestors, while keeping its layout as part of the current element tree.
DeferredScrollToItem
DevicePixels
Represents physical pixels on the display.
DismissEvent
Emitted by implementers of ManagedView to indicate the view should be dismissed, such as when a view is presented as a modal.
DisplayId
An opaque identifier for a hardware display
Div
A Div element, the all-in-one element for building complex UIs in GPUI
DivFrameState
A frame state for a Div element, which contains layout IDs for its children.
DivInspectorState
Interactivity state displayed an manipulated in the inspector.
DragMoveEvent
An event for when a drag is moving over this element, with the given state type.
Drawable
A wrapper around an implementer of Element that allows it to be drawn in a window.
DummyKeyboardMapper
A dummy implementation of the platform keyboard mapper
Edges
Represents the edges of a box in a 2D space, such as padding or margin.
EdgesRefinement
A refinable version of [#ident], see that documentation for details.
ElementClickedState
Whether or not the element or a group that contains it is clicked by the mouse.
ElementInputHandler
The canonical implementation of [crate::PlatformInputHandler]. Call Window::handle_input with an instance during your element’s paint.
Empty
The empty element, which renders nothing.
EmptyView
A view that renders nothing
Entity
A strong, well-typed reference to a struct which is managed by GPUI
EntityId
A unique identifier for a entity across the application.
ExternalPaths
A collection of paths from the platform, such as from a file drop.
FallbackPromptRenderer
The default GPUI fallback for rendering prompts, when the platform doesn’t support it.
FillOptions
Parameters for the fill tessellator.
FocusHandle
A handle which can be used to track and manipulate the focused element in a window.
FocusId
A globally unique identifier for a focusable element.
FocusOutEvent
This is provided when subscribing for Context::on_focus_out events.
Font
The configuration details for identifying a specific font.
FontFallbacks
The fallback fonts that can be configured for a given font. Fallback fonts family names are stored here.
FontFamilyId
An opaque identifier for a specific font family.
FontFeatures
The OpenType features that can be configured for a given font.
FontId
An opaque identifier for a specific font.
FontMetrics
A struct for storing font metrics. It is used to define the measurements of a typeface.
FontRun
A run of text with a single font.
FontWeight
The degree of blackness or stroke thickness of a font. This value ranges from 100.0 to 900.0, with 400.0 as normal.
ForegroundExecutor
A pointer to the executor that is currently running, for spawning tasks on the main thread.
GlobalElementId
A globally unique identifier for an element, used to track state across frames.
GlyphId
An identifier for a specific glyph, as returned by WindowTextSystem::layout_line.
GpuSpecs
Information about the GPU GPUI is running on.
GpuiBorrow
A mutable reference to an entity owned by GPUI
GridLocation
A location in a grid layout.
GroupStyle
The styling information for a given group.
HighlightStyle
A highlight style to apply, similar to a TextStyle except for a single font, uniformly sized and spaced text.
Hitbox
A rectangular region that potentially blocks hitboxes inserted prior. See Window::insert_hitbox for more details.
HitboxId
An identifier for a Hitbox which also includes HitboxBehavior.
Hsla
An HSLA color
Image
An image, with a format and certain bytes
ImageCacheElement
An image cache element.
ImageFormatIter
An iterator over the variants of ImageFormat
ImageId
A unique identifier for the image cache
ImageStyle
The style of an image element.
Img
An image element.
ImgLayoutState
The image layout state between frames
Inspector
Manages inspector state - which element is currently selected and whether the inspector is in picking mode.
InspectorElementId
A unique identifier for an element that can be inspected.
InspectorElementPath
GlobalElementId qualified by source location of element construction.
InteractiveElementState
The per-frame state of an interactive element. Used for tracking stateful interactions like clicks and scroll offsets.
InteractiveText
A text element that can be interacted with.
Interactivity
The interactivity struct. Powers all of the general-purpose interactivity in the Div element.
InvalidKeystrokeError
Error type for Keystroke::parse. This is used instead of anyhow::Error so that Zed can use markdown to display it.
ItemSize
The size of the item and its contents.
KeyBinding
A keybinding and its associated metadata, from the keymap.
KeyBindingMetaIndex
A unique identifier for retrieval of metadata associated with a key binding. Intended to be used as an index or key into a user-defined store of metadata associated with the binding, such as the source of the binding.
KeyContext
A datastructure for resolving whether an action should be dispatched at this point in the element tree. Contains a set of identifiers and/or key value pairs representing the current context for the keymap.
KeyDownEvent
The key down event equivalent for the platform.
KeyUpEvent
The key up event equivalent for the platform.
KeybindingKeystroke
Represents a keystroke that can be used in keybindings and displayed to the user.
KeyboardClickEvent
A click event that was generated by a keyboard button being pressed and released.
Keymap
A collection of key bindings for the user’s application.
KeymapVersion
An opaque identifier of which version of the keymap is currently active. The keymap’s version is changed whenever bindings are added or removed.
Keystroke
A keystroke and associated metadata generated by the platform
KeystrokeEvent
A keystroke event, and potentially the associated action
LayoutId
A unique identifier for a layout node, generated when requesting a layout from Taffy
LineLayout
A laid out and styled line of text
LineWrapper
The GPUI line wrapper, used to wrap lines of text to a given width.
LineWrapperHandle
A handle into the text system, which can be used to compute the wrapped layout of text
LinearColorStop
A color stop in a linear gradient.
List
A list element
ListOffset
An offset into the list’s items, in terms of the item index and the number of pixels off the top left of the item.
ListPrepaintState
Frame state used by the List element after layout.
ListScrollEvent
A scroll event that has been converted to be in terms of the list’s items.
ListState
The list state that views must hold on behalf of the list element.
Menu
A menu of the application, either a main menu or a submenu
Modifiers
The state of the modifier keys at some point in time
ModifiersChangedEvent
The modifiers changed event equivalent for the platform.
MouseClickEvent
A click event, generated when a mouse button is pressed and released.
MouseDownEvent
A mouse down event from the platform
MouseExitEvent
A mouse exit event from the platform, generated when the mouse leaves the window.
MouseMoveEvent
A mouse move event from the platform
MouseUpEvent
A mouse up event from the platform
NoAction
Action with special handling which unbinds the keybinding this is associated with, if it is the highest precedence match.
OsMenu
OS menus are menus that are recognized by the operating system This allows the operating system to provide specialized items for these menus
OwnedMenu
A menu of the application, either a main menu or a submenu
OwnedOsMenu
OS menus are menus that are recognized by the operating system This allows the operating system to provide specialized items for these menus
PaintQuad
A rectangle to be rendered in the window at the given position and size. Passed as an argument Window::paint_quad.
Path
A line made up of a series of vertices and control points.
PathBuilder
A Path builder.
PathPromptOptions
The options that can be configured for a file dialog prompt
Percentage
A type representing a percentage value.
Pixels
Represents a length in pixels, the base unit of measurement in the UI framework.
Point
Describes a location in a 2D cartesian space.
PointRefinement
A refinable version of [#ident], see that documentation for details.
PromptHandle
A handle to a prompt that can be used to interact with it.
PromptResponse
The event emitted when a prompt’s option is selected. The usize is the index of the selected option, from the actions passed to the prompt.
Radians
Represents an angle in Radians
Rems
Represents a length in rems, a unit based on the font-size of the window, which can be assigned with Window::set_rem_size.
RenderImage
A cached and processed image, in BGRA format
RenderablePromptHandle
A prompt handle capable of being rendered in a window.
Reservation
Returned by Context::reserve_entity to later be passed to Context::insert_entity. Allows you to obtain the EntityId for a entity before it is created.
RetainAllImageCache
An implementation of ImageCache, that uses an LRU caching strategy to unload images when the cache is full
RetainAllImageCacheProvider
A provider struct for creating a retain-all image cache inline
Rgba
An RGBA color
ScaledPixels
Represents scaled pixels that take into account the device’s scale factor.
Scope
Scope manages a set of tasks that are enqueued and waited on together. See BackgroundExecutor::scoped.
ScreenCaptureFrame
A frame of video captured from a screen.
ScrollAnchor
Represents an element that can be scrolled to in its parent element. Contrary to ScrollHandle::scroll_to_active_item, an anchored element does not have to be an immediate child of the parent.
ScrollHandle
A handle to the scrollable aspects of an element. Used for accessing scroll state, like the current scroll offset, and for mutating the scroll state, like scrolling to a specific child.
ScrollWheelEvent
A mouse wheel event from the platform
SemanticVersion
A semantic version number.
ShapedGlyph
A single glyph, ready to paint.
ShapedLine
A line of text that has been shaped and decorated.
ShapedRun
A run of text that has been shaped .
SharedString
A shared string is an immutable string that can be cheaply cloned in GPUI tasks. Essentially an abstraction over an Arc<str> and &'static str,
SharedUri
A SharedString containing a URI.
Size
A structure representing a two-dimensional size with width and height in a given unit.
SizeRefinement
A refinable version of [#ident], see that documentation for details.
SourceMetadata
Metadata for a given ScreenCaptureSource
Stateful
A wrapper around an element that can store state, produced after assigning an ElementId.
StrikethroughStyle
The properties that can be applied to a strikethrough.
StrikethroughStyleRefinement
A refinable version of [#ident], see that documentation for details.
StrokeOptions
Parameters for the tessellator.
Style
The CSS styling that can be applied to an element via the Styled trait
StyleRefinement
A refinable version of [#ident], see that documentation for details.
StyledText
Renders text with runs of different styles.
Subscription
A handle to a subscription created by GPUI. When dropped, the subscription is cancelled and the callback will no longer be invoked.
Surface
A surface element.
Svg
An SVG element.
SystemWindowTabController
A controller for managing window tabs.
Task
Task is a primitive that allows work to happen in the background.
TaskLabel
A task label is an opaque identifier that you can use to refer to a task in tests.
TextLayout
The Layout for TextElement. This can be used to map indices to pixels and vice versa.
TextRun
A styled run of text, for use in crate::TextLayout.
TextStyle
The properties that can be used to style text in GPUI
TextStyleRefinement
A refinable version of [#ident], see that documentation for details.
TextSystem
The GPUI text rendering sub system.
Tiling
A type to describe which sides of the window are currently tiled in some way
Timeout
Error returned by with_timeout when the timeout duration elapsed before the future resolved
Timer
A future or stream that emits timed events.
TitlebarOptions
The options that can be configured for a window’s titlebar
TooltipId
An identifier for a tooltip.
Transformation
A transformation to apply to an SVG element.
TransformationMatrix
A data type representing a 2 dimensional transformation that can be applied to an element.
UTF16Selection
A struct representing a selection in a text buffer, in UTF16 characters. This is different from a range because the head may be before the tail.
UnderlineStyle
The properties that can be applied to an underline.
UnderlineStyleRefinement
A refinable version of [#ident], see that documentation for details.
UniformList
A list element for efficiently laying out and displaying a list of uniform-height elements.
UniformListFrameState
Frame state used by the UniformList.
UniformListScrollHandle
A handle for controlling the scroll position of a uniform list. This should be stored in your view and passed to the uniform_list on each frame.
UniformListScrollState
WeakEntity
A weak reference to a entity of the given type.
WeakFocusHandle
A weak reference to a focus handle.
Window
Holds the state for a specific window.
WindowControls
What window controls this platform supports
WindowHandle
A handle to a window with a specific root view type. Note that this does not keep the window alive on its own.
WindowId
A unique identifier for a window.
WindowOptions
The variables that can be configured when creating a new window
WindowTextSystem
The GPUI text layout subsystem.
WrapBoundary
A boundary at which a line was wrapped
WrappedLine
A line of text that has been shaped, decorated, and wrapped by the text layout system.
WrappedLineLayout
A line of text that has been wrapped to fit a given width
Enums
AbsoluteLength
Represents an absolute length in pixels or rems.
ActionBuildError
Error type for Keystroke::parse. This is used instead of anyhow::Error so that Zed can use markdown to display it.
AlignContent
Sets the distribution of space between and around content items For Flexbox it controls alignment in the cross axis For Grid it controls alignment in the block axis
AlignItems
Used to control how child nodes are aligned. For Flexbox it controls alignment in the cross axis For Grid it controls alignment in the block axis
AnchoredFitMode
Which algorithm to use when fitting the anchored element to be inside the window.
AnchoredPositionMode
Which algorithm to use when positioning the anchored element.
ArcCow
AssetLogger
An asset Loader which logs the Err variant of a Result during loading
AvailableSpace
The space available for an element to be laid out in
Axis
Axis in a 2D cartesian space.
BorderStyle
The style of a border.
ClickEvent
A click event, generated when a mouse button or keyboard button is pressed and released.
ClipboardEntry
Either a ClipboardString or a ClipboardImage
ColorSpace
A color space for color interpolation.
Corner
Identifies a corner of a 2d box.
CursorStyle
The style of the cursor (pointer)
Decorations
A type to describe how this window is currently configured
DefiniteLength
A non-auto length that can be defined in pixels, rems, or percent of parent.
DispatchPhase
Represents the two different phases when dispatching events.
Display
Sets the layout used for the children of this node
ElementId
An identifier for an Element.
FileDropEvent
A file drop event from the platform, generated when files are dragged and dropped onto the window.
Fill
The kinds of fill that can be applied to a shape.
FillRule
The fill rule defines how to determine what is inside and what is outside of the shape.
FlexDirection
The direction of the flexbox layout main axis.
FlexWrap
Controls whether flex items are forced onto one line or can wrap onto multiple lines.
FontStyle
Allows italic or oblique faces to be selected.
GridPlacement
The placement of an item within a grid layout’s column or row.
HitboxBehavior
How the hitbox affects mouse behavior.
ImageAssetLoader
An image loader for the GPUI asset system
ImageCacheError
An error that can occur when interacting with the image cache.
ImageCacheItem
An image cache item
ImageFormat
One of the editor’s supported image formats (e.g. PNG, JPEG) - used when dealing with images in the clipboard
ImageSource
A source of image content.
KeyBindingContextPredicate
A datastructure for resolving whether an action should be dispatched Representing a small language for describing which contexts correspond to which actions.
KeyboardButton
An enum representing the keyboard button that was pressed for a click event.
Length
A length that can be defined in pixels, rems, percent of parent, or auto.
LineFragment
A fragment of a line that can be wrapped.
ListAlignment
Whether the list is scrolling from top to bottom or bottom to top.
ListHorizontalSizingBehavior
The horizontal sizing behavior to apply during layout.
ListMeasuringBehavior
The measuring behavior to apply during layout.
ListSizingBehavior
The sizing behavior to apply during layout.
MenuItem
The different kinds of items that can be in a menu
MouseButton
An enum representing the mouse button that was pressed.
NavigationDirection
A navigation direction, such as back or forward.
ObjectFit
How to fit the image into the bounds of the element.
OsAction
OS actions are actions that are recognized by the operating system This allows the operating system to provide specialized behavior for these actions
Overflow
How children overflowing their container should affect layout
OwnedMenuItem
The different kinds of items that can be in a menu
PathStyle
Style of the PathBuilder
PlatformInput
An enum corresponding to all kinds of platform input events.
Position
The positioning strategy for this item.
PromptButton
Prompt Button
PromptLevel
What kind of prompt styling to show
ResizeEdge
Which part of the window to resize
Resource
An enum representing
ScrollDelta
The scroll delta for a scroll wheel event.
ScrollStrategy
Where to place the element scrolled to.
SurfaceSource
A source of a surface’s content.
SystemMenuType
The type of system menu
TextAlign
How to align text within the element
TextOverflow
How to truncate text that overflows the width of the element
TouchPhase
The phase of a touch motion event. Based on the winit enum of the same name.
Visibility
The value of the visibility property, similar to the CSS property visibility
WhiteSpace
How to handle whitespace in text
WindowAppearance
The appearance of the window, as defined by the operating system.
WindowBackgroundAppearance
The appearance of the background of the window itself, when there is no content or the content is transparent.
WindowBounds
Represents the status of how a window should be opened.
WindowControlArea
A type of window control area that corresponds to the platform window.
WindowDecorations
A type to describe the appearance of a window
WindowKind
The kind of window to create
Constants
KEYSTROKE_PARSE_EXPECTED_MESSAGE
Sentence explaining what keystroke parser expects, starting with “Expected …”
LOADING_DELAY
The delay before showing the loading state.
SHUTDOWN_TIMEOUT
The duration for which futures returned from Context::on_app_quit can run before the application fully quits.
Traits
Action
Actions are used to implement keyboard-driven UI. When you declare an action, you can bind keys to the action in the keymap and listeners for that action in the element tree.
Along
A trait for accessing the given unit along a certain axis.
AnimationExt
An extension trait for adding the animation wrapper to both Elements and Components
AppContext
The context trait, allows the different contexts in GPUI to be used interchangeably for certain operations.
AsKeystroke
This is a helper trait so that we can simplify the implementation of some functions
Asset
A trait for asynchronous asset loading.
AssetSource
A source of assets for this app to use.
BorrowAppContext
A helper trait for auto-implementing certain methods on contexts that can be used interchangeably.
Element
Implemented by types that participate in laying out and painting the contents of a window. Elements form a tree and are laid out according to web-based layout rules, as implemented by Taffy. You can create custom elements by implementing this trait, see the module-level documentation for more details.
EntityInputHandler
Implement this trait to allow views to handle textual input when implementing an editor, field, etc.
EventEmitter
A trait for tying together the types of a GPUI entity and the events it can emit.
Flatten
A flatten equivalent for anyhow Results.
Focusable
Focusable allows users of your view to easily focus it (using window.focus_view(cx, view))
FutureExt
Extensions for Future types that provide additional combinators and utilities.
Global
A marker trait for types that can be stored in GPUI’s global state.
Half
Provides a trait for types that can calculate half of their value.
ImageCache
An object that can handle the caching and unloading of images. Implementations of this trait should ensure that images are removed from all windows when they are no longer needed.
ImageCacheProvider
An object that can create an ImageCache during the render phase. See the ImageCache trait for more information.
InputEvent
An event from a platform input source.
InputHandler
Zed’s interface for handling text input from the platform’s IME system This is currently a 1:1 exposure of the NSTextInputClient API:
InteractiveElement
A trait for elements that want to use the standard GPUI event handlers that don’t require any state.
IntoElement
Implemented by any type that can be converted into an element.
IsEmpty
IsZero
A trait for checking if a value is zero.
KeyEvent
A key event from the platform.
ManagedView
ManagedView is a view (like a Modal, Popover, Menu, etc.) where the lifecycle of the view is handled by another view.
MouseEvent
A mouse event from the platform.
Negate
Provides a trait for types that can negate their values.
ParentElement
This is a helper trait to provide a uniform interface for constructing elements that can accept any number of any kind of child elements
PlatformDisplay
A handle to a platform’s display, e.g. a monitor or laptop screen.
PlatformKeyboardLayout
A trait for platform-specific keyboard layouts
PlatformKeyboardMapper
A trait for platform-specific keyboard mappings
Prompt
A prompt that can be rendered in the window.
ReadGlobal
A trait for reading a global value from the context.
Refineable
A trait for types that can be refined with partial updates.
Render
An object that can be drawn to the screen. This is the trait that distinguishes “views” from other entities. Views are Entity’s which impl Render and drawn to the screen.
RenderOnce
You can derive IntoElement on any type that implements this trait. It is used to construct reusable components out of plain data. Think of components as a recipe for a certain pattern of elements. RenderOnce allows you to invoke this pattern, without breaking the fluent builder pattern of the element APIs.
ScreenCaptureSource
A source of on-screen video content that can be captured.
ScreenCaptureStream
A video stream captured from a screen.
StatefulInteractiveElement
A trait for elements that want to use the standard GPUI interactivity features that require state.
Styled
A trait for elements that can be styled. Use this to opt-in to a utility CSS-like styling API.
StyledImage
Style an image element.
UniformListDecoration
A decoration for a UniformList. This can be used for various things, such as rendering indent guides, or other visual effects.
UpdateGlobal
A trait for updating a global value in the context.
VisualContext
This trait is used for the different visual contexts in GPUI that require a window to be present.
Functions
anchored
anchored gives you an element that will avoid overflowing the window bounds. Its children should have no margin to avoid measurement issues.
auto
Returns a Length representing an automatic length.
background_executor
Returns a background executor for the current platform.
black
Pure black in Hsla
blue
The color blue in Hsla
bounce
Apply the given easing function, first in the forward direction and then in the reverse direction
bounds
Create a bounds with the given origin and size
canvas
Construct a canvas element with the given paint callback. Useful for adding short term custom drawing to a view.
combine_highlights
Combine and merge the highlights and ranges in the two iterators.
deferred
Builds a Deferred element, which delays the layout and paint of its child.
div
Construct a new Div element
ease_in_out
The quadratic ease-in-out function, which starts and ends slowly but speeds up in the middle
ease_out_quint
The Quint ease-out function, which starts quickly and decelerates to a stop
fallback_prompt_renderer
Use this function in conjunction with App::set_prompt_builder to force GPUI to always use the fallback prompt renderer.
fill
Creates a filled quad with the given bounds and background color.
font
Get a Font for a given name.
generate_list_of_all_registered_actions
Generate a list of all the registered actions. Useful for transforming the list of available actions into a format suited for static analysis such as in validating keymaps, or generating documentation.
green
The color green in Hsla
guess_compositor
Return which compositor we’re guessing we’ll use. Does not attempt to connect to the given compositor
hash
Use a quick, non-cryptographically secure hash function to get an identifier from data
hsla
Construct an Hsla object from plain values
image_cache
An image cache element, all its child img elements will use the cache specified by this element. Note that this could as simple as passing an Entity<T: ImageCache>
img
Create a new image element.
is_no_action
Returns whether or not this action represents a removed key binding.
linear
The linear easing function, or delta itself
linear_color_stop
Creates a new linear color stop.
linear_gradient
Creates a LinearGradient background color.
list
Construct a new list element
opaque_grey
Opaque grey in Hsla, values will be clamped to the range [0, 1]
outline
Creates a rectangle outline with the given bounds, border color, and a 1px border width
pattern_slash
Creates a hash pattern background
percentage
Generate a Radian from a percentage of a full circle.
phi
Returns the Golden Ratio, i.e. ~(1.0 + sqrt(5.0)) / 2.0.
point
Constructs a new Point<T> with the given x and y coordinates.
pulsating_between
A custom easing function for pulsating alpha that slows down as it approaches 0.1
px
Constructs a Pixels value representing a length in pixels.
quad
Creates a quad with the given parameters.
quadratic
The quadratic easing function, delta * delta
radians
Create a Radian from a raw value
red
The color red in Hsla
relative
Constructs a DefiniteLength representing a relative fraction of a parent size.
rems
Constructs a Rems value representing a length in rems.
retain_all
Constructs a retain-all image cache that uses the element state associated with the given ID.
rgb
Convert an RGB hex color code number to a color type
rgba
Convert an RGBA hex color code number to Rgba
size
Constructs a new Size<T> with the provided width and height.
solid_background
Creates a solid background color.
surface
Create a new surface element.
svg
Create a new SVG element.
transparent_black
Transparent black in Hsla
transparent_white
Transparent white in Hsla
uniform_list
uniform_list provides lazy rendering for a set of items that are of uniform height. When rendered into a container with overflow-y: hidden and a fixed (or max) height, uniform_list will only render the visible subset of items.
white
Pure white in Hsla
yellow
The color yellow in Hsla
Type Aliases
AlignSelf
Used to control how the specified nodes is aligned. Overrides the parent Node’s AlignItems property. For Flexbox it controls alignment in the cross axis For Grid it controls alignment in the block axis
ImageLoadingTask
An image loading task associated with an image cache.
ImgResourceLoader
A type alias to the resource loader that the img() element uses.
InspectorRenderer
Function set on App to render the inspector UI.
JustifyContent
Sets the distribution of space between and around content items For Flexbox it controls alignment in the main axis For Grid it controls alignment in the inline axis
JustifyItems
Used to control how child nodes are aligned. Does not apply to Flexbox, and will be ignored if specified on a flex container For Grid it controls alignment in the inline axis
JustifySelf
Used to control how the specified nodes is aligned. Overrides the parent Node’s JustifyItems property. Does not apply to Flexbox, and will be ignored if specified on a flex child For Grid it controls alignment in the inline axis
Result
Result<T, Error>
Transform
Alias for euclid::default::Transform2D<f32>
Attribute Macros
ctor
test
#[gpui::test] can be used to annotate test functions that run with GPUI support.
Derive Macros
Action
Action derive macro - see the trait documentation for details.
AppContext
#[derive(AppContext)] is used to create a context out of anything that holds a &mut App Note that a #[app] attribute is required to identify the variable holding the &mut App.
IntoElement
#[derive(IntoElement)] is used to create a Component out of anything that implements the RenderOnce trait.
Refineable
VisualContext
#[derive(VisualContext)] is used to create a visual context out of anything that holds a &mut Window and implements AppContext Note that a #[app] and a #[window] attribute are required to identify the variables holding the &mut App, and &mut Window respectively.

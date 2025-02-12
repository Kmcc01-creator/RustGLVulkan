
// Layout management
// src/debug/visualization/layout.rs
pub struct LayoutEngine {
    constraints: LayoutConstraints,
    widgets: Vec<WidgetLayout>,
}

pub struct WidgetLayout {
    position: (u32, u32),
    size: (u32, u32),
    constraints: LayoutConstraints,
}

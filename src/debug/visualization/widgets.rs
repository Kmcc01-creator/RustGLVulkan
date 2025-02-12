
// Visualization widgets
// src/debug/visualization/widgets.rs
pub trait DebugWidget {
    fn draw(&self, renderer: &mut dyn VisualizationRenderer);
    fn update(&mut self, data: &DebugData);
    fn get_size(&self) -> (u32, u32);
}

pub struct MemoryMapWidget {
    palette: ColorPalette,
    block_size: u32,
    highlight_regions: Vec<MemoryRegion>,
    layout: WidgetLayout,
}

pub struct TimelineWidget {
    palette: ColorPalette,
    time_range: Range<f64>,
    events: Vec<TimelineEvent>,
    layout: WidgetLayout,
}

pub struct HeatMapWidget {
    palette: ColorPalette,
    data: Vec<f32>,
    dimensions: (u32, u32),
    scale: (f32, f32),
}


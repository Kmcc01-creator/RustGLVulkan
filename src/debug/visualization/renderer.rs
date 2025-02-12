// Visualization system
// src/debug/visualization/renderer.rs
pub struct DebugVisualizer {
    renderer: Box<dyn VisualizationRenderer>,
    layout_engine: LayoutEngine,
    active_palettes: HashMap<String, ColorPalette>,
    widgets: Vec<Box<dyn DebugWidget>>,
}

pub trait VisualizationRenderer {
    fn begin_frame(&mut self);
    fn end_frame(&mut self);
    fn draw_memory_map(&mut self, data: &MemoryMapData);
    fn draw_timeline(&mut self, data: &TimelineData);
    fn draw_heat_map(&mut self, data: &HeatMapData);
    fn draw_graph(&mut self, data: &GraphData);
}

// src/ui/components/memory_view.rs
pub struct MemoryView {
    bounds: Rect,
    stats: MemoryStatsUpdate,
    graph_data: VecDeque<(f64, f64)>,
    color_scheme: ColorScheme,
}

impl MemoryView {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            stats: MemoryStatsUpdate::default(),
            graph_data: VecDeque::with_capacity(100),
            color_scheme: ColorScheme::default(),
        }
    }
}

impl UIComponent for MemoryView {
    fn render(&self, renderer: &mut UIRenderer) {
        // Render memory stats
        renderer.draw_text(
            &format!("Total: {} MB", self.stats.total_allocated / 1024 / 1024),
            self.bounds.top_left(),
            TextStyle::default(),
        );
        
        // Render memory graph
        let graph_bounds = self.bounds.shrink(10.0);
        renderer.begin_graph(graph_bounds);
        renderer.draw_line_graph(&self.graph_data, self.color_scheme.primary);
        renderer.end_graph();
        
        // Render memory blocks
        self.render_memory_blocks(renderer);
    }
    
    fn handle_event(&mut self, event: UIEvent) -> bool {
        match event {
            UIEvent::MouseOver(pos) if self.bounds.contains(pos) => {
                // Handle hover interaction
                true
            }
            _ => false,
        }
    }
}

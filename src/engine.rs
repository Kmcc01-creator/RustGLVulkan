
// Example usage in main engine loop:
// src/engine.rs
impl Engine {
    pub fn run(&mut self) {
        let (mut ui_bridge, ui_sender) = UIBridge::new();
        let mut ui_renderer = UIRenderer::new(self.gl.clone());
        
        let mut memory_view = MemoryView::new(Rect::new(10.0, 10.0, 300.0, 200.0));
        
        loop {
            // Update engine state
            self.update();
            
            // Send memory stats to UI
            if let Some(stats) = self.memory_manager.get_stats() {
                ui_sender.send(UIMessage::MemoryStats(stats.into()));
            }
            
            // Process UI messages
            ui_bridge.process_messages();
            
            // Render main engine content
            self.render();
            
            // Render UI overlay
            ui_renderer.begin_frame(self.viewport);
            memory_view.render(&mut ui_renderer);
            ui_renderer.end_frame();
            
            // Swap buffers
            self.window.swap_buffers();
        }
    }
}
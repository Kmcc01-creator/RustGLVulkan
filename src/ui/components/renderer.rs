

// src/ui/renderer.rs
pub struct UIRenderer {
    gl: gl::Gl,
    text_renderer: TextRenderer,
    shape_renderer: ShapeRenderer,
    current_viewport: Rect,
}

impl UIRenderer {
    pub fn new(gl: gl::Gl) -> Self {
        Self {
            gl,
            text_renderer: TextRenderer::new(&gl),
            shape_renderer: ShapeRenderer::new(&gl),
            current_viewport: Rect::default(),
        }
    }
    
    pub fn begin_frame(&mut self, viewport: Rect) {
        self.current_viewport = viewport;
        unsafe {
            self.gl.Enable(gl::BLEND);
            self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }
    
    pub fn draw_line_graph(&mut self, data: &[(f64, f64)], color: Color) {
        self.shape_renderer.begin(gl::LINE_STRIP);
        for &(x, y) in data {
            self.shape_renderer.vertex(x, y, color);
        }
        self.shape_renderer.end();
    }
}

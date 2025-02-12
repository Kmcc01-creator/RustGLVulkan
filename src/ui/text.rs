// src/ui/text.rs
pub struct TextRenderer {
    shader: Shader,
    vertex_buffer: VertexBuffer,
    atlas: TextureAtlas,
    characters: HashMap<char, CharacterInfo>,
}

impl TextRenderer {
    pub fn new(gl: &gl::Gl) -> Self {
        // Initialize font atlas
        let atlas = TextureAtlas::new(512, 512);
        let characters = Self::generate_character_set(&atlas);
        
        // Create shader for text rendering
        let shader = Shader::new(gl, TEXT_VERTEX_SHADER, TEXT_FRAGMENT_SHADER);
        
        // Create vertex buffer for quads
        let vertex_buffer = VertexBuffer::new(gl, gl::DYNAMIC_DRAW);
        
        Self {
            shader,
            vertex_buffer,
            atlas,
            characters,
        }
    }
    
    pub fn draw_text(&mut self, text: &str, position: Vector2, style: &TextStyle) {
        self.shader.bind();
        self.atlas.bind();
        
        let mut x = position.x;
        let y = position.y;
        
        for c in text.chars() {
            if let Some(info) = self.characters.get(&c) {
                self.draw_character(c, x, y, info, style);
                x += info.advance;
            }
        }
    }
}
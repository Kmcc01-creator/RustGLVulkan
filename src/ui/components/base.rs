
// src/ui/components/base.rs
pub trait UIComponent {
    fn render(&self, renderer: &mut UIRenderer);
    fn handle_event(&mut self, event: UIEvent) -> bool;
    fn update(&mut self, state: &UIState);
    fn get_bounds(&self) -> Rect;
}
# Compositor Support Notes

## Supported Compositors
1. KWin (KDE)
   - Blur effects
   - Window shadows
   - Animations
   - Rounded corners

2. Mutter (GNOME)
   - Client-side decorations
   - Scale factor handling
   - Gesture support

3. Picom
   - Dual Kawase blur
   - Shadows
   - Fading
   - Rounded corners

## Effect Implementation Status
- [x] Basic window transparency
- [x] Shadow effects
- [ ] Gaussian blur
- [ ] Animation system
- [ ] Corner masking

## Desktop Environment Specific Features
- KDE Plasma
  - Window rules system
  - Activity support
  - Desktop effects API

- GNOME
  - Shell extensions integration
  - Mutter clutter scene graph

- XFCE
  - Basic compositing
  - Window manager hints

## Performance Considerations
- Blur effect optimization
- Shadow caching
- Animation frame timing
- Vsync handling

## Future Plans
1. Shader-based effects
2. Custom animation system
3. Per-pixel alpha support
4. Monitor layout handling

# `toa_animator`

An opinionated 2d animator.

## Todo:
- [ ] Create the art format: https://github.com/NoahShomette/toa_animator/issues/1
- [ ] Add offsets/marker components for sub graphics
- [ ] Add sub graphics/animations
- [ ] Add sound effect markers to animations
- [ ] Add tweening markers to animations
- [ ] Create the basic animation editor cli (https://github.com/NoahShomette/Astroculture-or-Bust/issues/21)
- [ ] Create a basic widget for the editor so it can be dropped into any other projects
- [ ] Create a basic standalone editor
- [ ] Create a one place storage for animations, sound, tweens.
  - [ ] Add a trait based getter from this storage to enable custom modifications of the animations (Maybe just do this under leafwing manifest? Or instead require registering specific markers for each type that will be automatically handled and allow passing in a custom function)
- [ ] Add automatic handling of states - change sprites, play sounds, play tweens, etc. This would be especially needed as the format got more complex and we had many different things that needed to be set/changed as frames changed and animations changed
- [ ] Animation variants - multiple options from just setting the animation to always that variant, randomly playing the variant, picking a random variant when changing into it, always picking one variant, etc.

## Exploration

### State
To really make `toa_animator` easily droppable into projects as well as unlock all its potential automatically managing things we need to merge in the concept of states. Animations are state driven. [seldom_state](https://github.com/Seldom-SE/seldom_state) is a great crate but is limited since only one state can be on an entity at once. Experiement with [bevy_reactor](https://github.com/viridia/bevy_reactor?tab=readme-ov-file#introduction-to-reactive-contexts) to implement a multi state heirarchal system. 

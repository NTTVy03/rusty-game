# Rusty engine

Make a game with Rusty engine

Join course: [Rust for the daily practitioner](https://bosch-bgsv.udemy.com/course/ultimate-rust-2/learn/lecture/32352640#overview); OR follow this tutorial: [Rusty Engine Tutorial](https://cleancut.github.io/rusty_engine/)

1. [Configuration](https://cleancut.github.io/rusty_engine/05-config.html)
2. [Asset pack](https://cleancut.github.io/rusty_engine/10-assets.html)
3. [Engine Initialization](https://cleancut.github.io/rusty_engine/15-init.html)
4. [Game State](https://cleancut.github.io/rusty_engine/20-game-state.html)
5. [Game Logic Function](https://cleancut.github.io/rusty_engine/25-game-logic-function.html)
    * about 60 frames each second
    * Rusty Engine tries to run your game logic function once each frame
    * a frame produces a new image to display on the screen
6. [Sprite](https://cleancut.github.io/rusty_engine/50-sprite.html)
    * A sprite in Rusty Engine is a 2D image. You will use sprites for all the graphics in your game.
    * [Create Sprite](https://cleancut.github.io/rusty_engine/55-sprite-creation.html)
    * [Placement](https://cleancut.github.io/rusty_engine/60-sprite-placement.html)
        * Translation: coordinates of your sprite's position on the screen
        * Rotation: the angle in radians from the positive X axis
        * Scale
        * Layer: affects what sprite or text is "on top" of another sprite or text when they overlap
    * [Sprite Collisions](https://cleancut.github.io/rusty_engine/65-sprite-collider.html)
        * When two sprites begin or end overlapping, a `CollisionEvent` will be produced (example: `CollisionEvent { state: Begin, pair: CollisionPair("car1", "player") }`)
        * Colliders are convex polygons that are used to detect if a collision has occurred between two sprites.
# The file of things yet to do.

- [x] Implement the textures
- [x] Add movement logic
  - [ ] Add logic for handling sprite rotation
- [ ] Add collision logic
- [ ] Add fruit spawner

## Movement Logic

The conundrum of movement in this game is how best to represent the snake.
In ECS, you generally want to have each _thing_ be a separate component or even entity.
Question then becomes, what constitutes a component here?
The snake itself, or each individual segment?

### Option 1: The Snake as an Entity

The snake is the entity and each segment is a component.
Trouble immediately rears its ugly head, as it suddenly stops being obvious how one would keep track of every segment's position.

### Option 2: The segment is the Entity [CHOSEN STRATEGY]

Each segment is an entity with an associated transform in and of itself.
This has the obvious benefit that the position of a segment is always known and accounted for.

#### Turning Option 1: Follow the leader [CHOSEN STRATEGY]

Turning would be accomplished by updating the orientation fo the head, and then let all the other segments follow suit.

**BENEFITS**:

- Snake is physically moved, no need to change segment sprites
- Does not suffer from race conditions as all the segments are moving at the same pace, and will only reach the "turning point" once it overlaps a given coordinate.

**DRAWBACKS**:

- Requires a weird "Follow the leader" algorithm that needs to account for every coordinate on which the head turned, in order to facilitate the famous zig-zag patterns possible in the game.

This could possibly be implemented with a collection, which records a coordinate and an orientation and changes the orientation of a segment once it reaches that position.
Once the tail reaches the coordinate, the entry is removed from the collection.
Alternatively, if implemented via a HashMap, the head of the snake will have exclusive update rights, and if it passes over an old coordinate, it simply gets updated.

**NOTE**: This has the possibility of becoming slower with more entries.

#### Turning Option 2: Spawn and Remove

Another possibility would be to spawn a segment ahead and remove one behind, leaving everything else static.

**BENEFITS**:

- Does not require implementing any complex movement logic as nothing actually _moves_.
- Figuring out what's a head and what's a tail is encoded in the Snekment struct. Drawing the sprite happens during rendering anyway.

**DRAWBACKS**:

- The sprite of the snake head and tail needs to be updated every time.
  As a segment is added ahead, the old head needs to become a body, the old tail removed, and a new tail put in place of the penultimate segment.
- This may cause unforseen problems.

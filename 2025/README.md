# Advent of Code 2025

Goals for this year are to give it a shot in gdscript in Godot game engine.

## gdscript
Based on python but missing many of the basic containers and structures.
Local implementations:
- Set
- Union Find
- Graph functions (DFS, BFS)

# Rules

## Plugins
Outside of GdUnit4 for Unit testing, I don't plan on installing any plugins.

## Build Mode
The project is written up in `Compatibility` mode because it's all text
based and UI-Control nodes.  

## AI Use
AI will not be used for anything other than as a short cut for writing unit tests
against the helper classes I've written myself.

# Setup
If you want to run some of this yourself, log onto Adevent of Code and get your SESSION ID
from your login.  This will allow the auto-download of your input which will be stored locally.

Set SESSIONID as an environmental variable prior to launching the Godot engine.

The days are set up to store and retain the test input day over day as is.

# Thoughts on Conclusion of 2025
I learned a lot about Godot.  Originally I was making each day a static class, but there really is no reason to do this.  And by not doing that, I can have class variables that are accessible to each function/method in the class.

The reduced set of days for Adevent of Code was a bit of a plus.  The last 3 days were difficult questions but doable.  And I'm not spending 4 hours on Christmas day doing the last solve.

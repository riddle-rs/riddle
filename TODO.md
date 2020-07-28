# TODO

* Add support for setting a world transform matrix in the renderer
* See how much of the global Riddle state I can put behind an Rc to
  avoid having to pass the RiddleContext around for things like
  getting the fps or time.
* Convert from SPIRV to WGSL.
* SpriteRenderArgs -> SpriteRenderCommand.
* Implement timers.
* Support for sampler filter modes in sprite renderer.
* Don't burn an entire command buffer on clearing the frame, do it at the beginning of the next frame buffer load, or force one at submit time if there were no loads since the clear call.
* Input support for controllers.
* Virtual keys support
* Modifier support
* Move to github issues
* Audio volume controls
* Quick fade in/out on stop/pause/resume to avoid popping
* Convert to using Arc::cyclic_new instead of Rc::new where possible

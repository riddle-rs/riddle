(function() {var implementors = {};
implementors["riddle_audio"] = [{"text":"impl Deref for AudioSystemHandle","synthetic":false,"types":[]}];
implementors["riddle_input"] = [{"text":"impl Deref for InputSystemHandle","synthetic":false,"types":[]}];
implementors["riddle_platform_winit"] = [{"text":"impl Deref for PlatformSystemHandle","synthetic":false,"types":[]},{"text":"impl Deref for WindowHandle","synthetic":false,"types":[]}];
implementors["riddle_renderer_wgpu"] = [{"text":"impl Deref for RendererHandle","synthetic":false,"types":[]}];
implementors["riddle_time"] = [{"text":"impl Deref for TimeSystemHandle","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
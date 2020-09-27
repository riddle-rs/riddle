(function() {var implementors = {};
implementors["riddle_common"] = [{"text":"impl&lt;T:&nbsp;Clone&gt; Default for EventPub&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone&gt; Default for EventSub&lt;T&gt;","synthetic":false,"types":[]}];
implementors["riddle_math"] = [{"text":"impl&lt;T:&nbsp;Default&gt; Default for Vector2&lt;T&gt;","synthetic":false,"types":[]}];
implementors["riddle_platform_common"] = [{"text":"impl Default for LogicalPosition","synthetic":false,"types":[]}];
implementors["riddle_platform_winit"] = [{"text":"impl Default for WindowBuilder","synthetic":false,"types":[]}];
implementors["riddle_renderer_wgpu"] = [{"text":"impl Default for SpriteRenderCommand","synthetic":false,"types":[]},{"text":"impl&lt;'a, Device&gt; Default for WGPUSpriteAtlasBuilder&lt;'a, Device&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Device: WGPUDevice,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Default for FilterMode","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
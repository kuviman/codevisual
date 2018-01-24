initSidebarItems({"enum":[["Api","All APIs related to OpenGL that you can possibly get while using glutin."],["ContextError","Error that can happen when manipulating an OpenGL context."],["ControlFlow","Returned by the user callback given to the `EventsLoop::run_forever` method."],["CreationError","Error that can happen while creating a window or a headless renderer."],["CursorState","Describes how winit handles the cursor."],["DeviceEvent","Represents raw hardware events that are not associated with any particular window."],["ElementState","Describes the input state of a key."],["Event","Describes a generic event."],["GlProfile","Describes the requested OpenGL context profiles."],["GlRequest","Describes the OpenGL API and version that are being requested when a context is created."],["MouseButton","Describes a button of a mouse controller."],["MouseCursor","Describes the appearance of the mouse cursor."],["MouseScrollDelta","Describes a difference in the mouse scroll wheel state."],["ReleaseBehavior","The behavior of the driver when you change the current context."],["Robustness","Specifies the tolerance of the OpenGL context to faults. If you accept raw OpenGL commands and/or raw shader code from an untrusted source, you should definitely care about this."],["TouchPhase","Describes touch-screen input state."],["VirtualKeyCode","Symbolic name for a keyboard key."],["WindowCreationError","Error that can happen while creating a window or a headless renderer."],["WindowEvent","Describes an event from a `Window`."]],"mod":[["os","Contains traits with platform-specific methods in them."]],"static":[["GL_CORE","The minimum core profile GL context. Useful for getting the minimum required GL version while still running on OSX, which often forbids the compatibility profile features."]],"struct":[["AvailableMonitorsIter","An iterator for the list of available monitors."],["Context","Represents an OpenGL context."],["ContextBuilder","Object that allows you to build `Context`s."],["DeviceId","Identifier of an input device."],["EventsLoop","Provides a way to retreive events from the system and from the windows that were registered to the events loop."],["EventsLoopClosed","The error that is returned when an `EventsLoopProxy` attempts to wake up an `EventsLoop` that no longer exists."],["EventsLoopProxy","Used to wake up the `EventsLoop` from another thread."],["GlAttributes","Attributes to use when creating an OpenGL context."],["GlWindow","Represents an OpenGL context and a Window with which it is associated."],["HeadlessContext","Represents a headless OpenGL context."],["HeadlessRendererBuilder","Object that allows you to build headless contexts."],["KeyboardInput","Describes a keyboard input event."],["ModifiersState","Represents the current state of the keyboard modifiers"],["MonitorId","Identifier for a monitor."],["PixelFormat","Describes a possible format. Unused."],["PixelFormatRequirements","Describes how the backend should choose a pixel format."],["Touch","Represents touch event"],["Window","Represents a window."],["WindowAttributes","Attributes to use when creating a window."],["WindowBuilder","Object that allows you to build windows."],["WindowId","Identifier of a window. Unique for each window."]],"trait":[["GlContext","A trait for types associated with a GL context."]],"type":[["AxisId","Identifier for a specific analog axis on some device."],["ButtonId","Identifier for a specific button on some device."],["ScanCode","Hardware-dependent keyboard scan code."]]});
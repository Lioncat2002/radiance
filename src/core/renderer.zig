const rl = @import("raylib");

pub const Renderer = struct {
    screen_width: u16,
    screen_height: u16,
    pub fn new(screen_width: u16, screen_height: u16) Renderer {
        return Renderer{ .screen_width = screen_width, .screen_height = screen_height };
    }

    pub fn load(self: *Renderer) void {
        rl.initWindow(self.screen_width, self.screen_height, "radiance");
        rl.setTargetFPS(60);
    }

    pub fn update(self: *Renderer) void {
        _ = self;
    }

    pub fn draw(self: *Renderer) void {
        _ = self;
        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(rl.Color.white);

        rl.drawText("Congrats! You \n created your first window!", 190, 200, 20, rl.Color.light_gray);
    }

    pub fn event_loop(self: *Renderer) void {
        while (!rl.windowShouldClose()) { // Detect window close button or ESC key
            // Update
            //----------------------------------------------------------------------------------
            // TODO: Update your variables here
            //----------------------------------------------------------------------------------
            self.update();
            // Draw
            //----------------------------------------------------------------------------------
            self.draw();
            //----------------------------------------------------------------------------------

        }
        rl.closeWindow();
    }
};

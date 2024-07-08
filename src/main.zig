const rl = @import("raylib");
const std = @import("std");
const Renderer = @import("core/renderer.zig").Renderer;
const PieceTable = @import("core/piece_table.zig").PieceTable;

pub fn main() anyerror!void {
    // Initialization
    //--------------------------------------------------------------------------------------
    const screenWidth = 800;
    const screenHeight = 450;

    const r = Renderer.new(screenWidth, screenHeight);
    _ = r;
    //--------------------------------------------------------------------------------------
    //r.load();
    // Main game loop
    //r.event_loop();

    var allocator = std.heap.page_allocator;
    const originalText = "hello world";
    var pieceTable = try PieceTable.new(&allocator, originalText);
    defer pieceTable.destroy();

    try pieceTable.insert(originalText.len, " beautiful");

    const currentText = try pieceTable.get_text(&allocator);
    defer allocator.free(currentText);
    std.debug.print("\nCurrent text: {s}\n", .{currentText});
}
//const std = @import("std");
//const texteditor = @import("texteditor");

//pub fn main() !void {
//   var editor = try texteditor.init(texteditor.Buffer.initAllocator(std.heap.page_allocator), "meow");
//   defer editor.deinit();

// Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
//   std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

// stdout is for the actual output of your application, for example if you
// are implementing gzip, then only the compressed bytes should be sent to
// stdout, not any debugging messages.
//    const stdout_file = std.io.getStdOut().writer();
//    var bw = std.io.bufferedWriter(stdout_file);
//    const stdout = bw.writer();

//    try stdout.print("Run `zig build test` to run the tests.\n", .{});

//    try bw.flush(); // don't forget to flush!
//}

const std = @import("std");
pub const PieceSource = enum { Original, Add };

pub const Piece = struct {
    source: PieceSource,
    start: usize,
    length: usize,
};

pub const PieceTable = struct {
    original: []const u8,
    add: []u8,
    pieces: std.ArrayList(Piece),

    pub fn new(allocator: *std.mem.Allocator, original: []const u8) !PieceTable {
        var pieces = std.ArrayList(Piece).init(allocator.*);
        try pieces.append(Piece{ .source = .Original, .start = 0, .length = original.len });
        return PieceTable{ .original = original, .add = &[_]u8{}, .pieces = pieces };
    }

    pub fn insert(self: *PieceTable, pos: usize, text: []const u8) !void {
        const add_pos = self.add.len;
        self.add = try std.fmt.allocPrint(self.pieces.allocator, "{s}{s}", .{ self.add, text });
        var piece_idx = self.find_piece(pos);
        const piece = self.pieces.items[piece_idx];
        //incase of split
        if (piece.start + piece.length > pos) {
            const split_offset = pos - piece.start;
            const new_piece = Piece{ .source = piece.source, .start = piece.start + split_offset, .length = piece.length - split_offset };
            self.pieces.items[piece_idx].length = split_offset;
            try self.pieces.insert(piece_idx + 1, new_piece);
            piece_idx += 1;
        } else if (self.pieces.items.len <= 1) {
            //handle edge cases when less than 2 elements present in piecetable
            const new_piece = Piece{ .source = .Add, .start = add_pos, .length = text.len };
            try self.pieces.append(new_piece);
        } else {
            //insert a new piece for added text
            const new_piece = Piece{ .source = .Add, .start = add_pos, .length = text.len };
            try self.pieces.insert(piece_idx, new_piece);
        }
    }

    pub fn destroy(self: *PieceTable) void {
        self.pieces.deinit();
    }

    pub fn find_piece(self: *PieceTable, pos: usize) usize {
        var offset: usize = 0;
        for (self.pieces.items, 0..) |piece, i| {
            if (offset + piece.length > pos) {
                return i;
            }
            offset += piece.length;
        }
        return self.pieces.items.len - 1;
    }

    pub fn get_text(self: *PieceTable, allocator: *std.mem.Allocator) ![]u8 {
        var builder: []u8 = "";

        for (self.pieces.items) |piece| {
            const source = switch (piece.source) {
                .Original => self.original,
                .Add => self.add,
            };
            //try builder.appendSlice(source[piece.start .. piece.start + piece.length]);
            builder = try std.fmt.allocPrint(allocator.*, "{s}{s}", .{ builder, source[piece.start .. piece.start + piece.length] });
        }

        return builder;
    }
};

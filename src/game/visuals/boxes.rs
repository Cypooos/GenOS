use alloc::format;

// use crate::io::{vga_writer, vga_writer::WRITER};

pub fn box_simple(pos: (usize, usize), size: (usize, usize)) {
    vga_write!(
        pos.0,
        pos.1,
        "{:\u{C4}<width$}\u{BF}\n{}{:\u{C4}<width$}\u{D9}",
        "\u{DA}",
        format!("{: <width$}\u{B3}\n", "\u{B3}", width = size.0).repeat(size.1),
        "\u{C0}",
        width = size.0
    );
}

pub fn box_double(pos: (usize, usize), size: (usize, usize)) {
    vga_write!(
        pos.0,
        pos.1,
        "{:\u{CD}<width$}\u{BB}\n{}{:\u{CD}<width$}\u{BC}",
        "\u{C9}",
        format!("{: <width$}\u{BA}\n", "\u{BA}", width = size.0).repeat(size.1),
        "\u{C8}",
        width = size.0
    );
}

pub fn old_box_simple(pos: (usize, usize), size: (usize, usize)) {
    // Top/Bottom
    vga_write!(
        pos.0,
        pos.1,
        "{:\u{C4}<width$}\u{BF}",
        "\u{DA}",
        width = size.0
    );
    vga_write!(
        pos.0,
        pos.1 + size.1,
        "{:\u{C0}<width$}\u{BF}",
        "\u{D9}",
        width = size.0
    );
    for _x in 1..size.1 {
        vga_write!(
            pos.0,
            pos.1 + size.1,
            "{: <width$}\u{B3}",
            "\u{B3}",
            width = size.0
        );
    }
}

pub struct Display {
    /// the definition of the display pixels
    display: [[u8; 160]; 144],
    // 8x8 areas of the display
    tiles: [[u8; 20]; 18],
}

pub const BITRATES: [[u16; 16]; 5] = [
    [
        0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 0,
    ],
    [
        0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 0,
    ],
    [
        0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0,
    ],
    [
        0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256, 0,
    ],
    [
        0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0,
    ],
];
pub const SAMPLING_FREQ: [[u16; 4]; 4] = [
    [44100, 48000, 32000, 0],
    [22050, 24000, 16000, 0],
    [11025, 12000, 8000, 0],
    [0, 0, 0, 0],
];
pub const SAMPLES_PER_FRAME: [[u32; 4]; 2] = [[384, 1152, 1152, 0], [384, 1152, 576, 0]];

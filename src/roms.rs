/// A group of Chip8 programs for testing the interpreter
pub mod testroms {
    /// A Chip8 program that displays the IBM logo
    pub const IBM_LOGO: [u16; 66] = [
        0x00e0, 0xa22a, 0x600c, 0x6108, 0xd01f, 0x7009, 0xa239, 0xd01f, 0xa248, 0x7008, 0xd01f,
        0x7004, 0xa257, 0xd01f, 0x7008, 0xa266, 0xd01f, 0x7008, 0xa275, 0xd01f, 0x1228, 0xff00,
        0xff00, 0x3c00, 0x3c00, 0x3c00, 0x3c00, 0xff00, 0xffff, 0x00ff, 0x0038, 0x003f, 0x003f,
        0x0038, 0x00ff, 0x00ff, 0x8000, 0xe000, 0xe000, 0x8000, 0x8000, 0xe000, 0xe000, 0x80f8,
        0x00fc, 0x003e, 0x003f, 0x003b, 0x0039, 0x00f8, 0x00f8, 0x0300, 0x0700, 0x0f00, 0xbf00,
        0xfb00, 0xf300, 0xe300, 0x43e0, 0x00e0, 0x0080, 0x0080, 0x0080, 0x0080, 0x00e0, 0x00e0,
    ];

    pub const OP_TEST: [u16; 239] = [
        0x124e, 0xeaac, 0xaaea, 0xceaa, 0xaaae, 0xe0a0, 0xa0e0, 0xc040, 0x40e0, 0xe020, 0xc0e0,
        0xe060, 0x20e0, 0xa0e0, 0x2020, 0x6040, 0x2040, 0xe080, 0xe0e0, 0xe020, 0x2020, 0xe0e0,
        0xa0e0, 0xe0e0, 0x20e0, 0x40a0, 0xe0a0, 0xe0c0, 0x80e0, 0xe080, 0xc080, 0xa040, 0xa0a0,
        0xa202, 0xdab4, 0x00ee, 0xa202, 0xdab4, 0x13dc, 0x6801, 0x6905, 0x6a0a, 0x6b01, 0x652a,
        0x662b, 0xa216, 0xd8b4, 0xa23e, 0xd9b4, 0xa202, 0x362b, 0xa206, 0xdab4, 0x6b06, 0xa21a,
        0xd8b4, 0xa23e, 0xd9b4, 0xa206, 0x452a, 0xa202, 0xdab4, 0x6b0b, 0xa21e, 0xd8b4, 0xa23e,
        0xd9b4, 0xa206, 0x5560, 0xa202, 0xdab4, 0x6b10, 0xa226, 0xd8b4, 0xa23e, 0xd9b4, 0xa206,
        0x76ff, 0x462a, 0xa202, 0xdab4, 0x6b15, 0xa22e, 0xd8b4, 0xa23e, 0xd9b4, 0xa206, 0x9560,
        0xa202, 0xdab4, 0x6b1a, 0xa232, 0xd8b4, 0xa23e, 0xd9b4, 0x2242, 0x6817, 0x691b, 0x6a20,
        0x6b01, 0xa20a, 0xd8b4, 0xa236, 0xd9b4, 0xa202, 0xdab4, 0x6b06, 0xa22a, 0xd8b4, 0xa20a,
        0xd9b4, 0xa206, 0x8750, 0x472a, 0xa202, 0xdab4, 0x6b0b, 0xa22a, 0xd8b4, 0xa20e, 0xd9b4,
        0xa206, 0x672a, 0x87b1, 0x472b, 0xa202, 0xdab4, 0x6b10, 0xa22a, 0xd8b4, 0xa212, 0xd9b4,
        0xa206, 0x6678, 0x671f, 0x8762, 0x4718, 0xa202, 0xdab4, 0x6b15, 0xa22a, 0xd8b4, 0xa216,
        0xd9b4, 0xa206, 0x6678, 0x671f, 0x8763, 0x4767, 0xa202, 0xdab4, 0x6b1a, 0xa22a, 0xd8b4,
        0xa21a, 0xd9b4, 0xa206, 0x668c, 0x678c, 0x8764, 0x4718, 0xa202, 0xdab4, 0x682c, 0x6930,
        0x6a34, 0x6b01, 0xa22a, 0xd8b4, 0xa21e, 0xd9b4, 0xa206, 0x668c, 0x6778, 0x8765, 0x47ec,
        0xa202, 0xdab4, 0x6b06, 0xa22a, 0xd8b4, 0xa222, 0xd9b4, 0xa206, 0x66e0, 0x866e, 0x46c0,
        0xa202, 0xdab4, 0x6b0b, 0xa22a, 0xd8b4, 0xa236, 0xd9b4, 0xa206, 0x660f, 0x8666, 0x4607,
        0xa202, 0xdab4, 0x6b10, 0xa23a, 0xd8b4, 0xa21e, 0xd9b4, 0xa3e8, 0x6000, 0x6130, 0xf155,
        0xa3e9, 0xf065, 0xa206, 0x4030, 0xa202, 0xdab4, 0x6b15, 0xa23a, 0xd8b4, 0xa216, 0xd9b4,
        0xa3e8, 0x6689, 0xf633, 0xf265, 0xa202, 0x3001, 0xa206, 0x3103, 0xa206, 0x3207, 0xa206,
        0xdab4, 0x6b1a, 0xa20e, 0xd8b4, 0xa23e, 0xd9b4, 0x1248, 0x13dc,
    ];

    pub const BC_TEST: [u16; 235] = [
        0x00e0, 0x6300, 0x6401, 0x65ee, 0x35ee, 0x1310, 0x6300, 0x6402, 0x65ee, 0x66ee, 0x5560,
        0x1310, 0x6300, 0x6403, 0x65ee, 0x45fd, 0x1310, 0x6300, 0x6404, 0x65ee, 0x7501, 0x35ef,
        0x1310, 0x6300, 0x6405, 0x6f01, 0x65ee, 0x66ef, 0x8565, 0x3f00, 0x1310, 0x6300, 0x6406,
        0x6f00, 0x65ef, 0x66ee, 0x8565, 0x3f01, 0x1310, 0x6f00, 0x6300, 0x6407, 0x65ee, 0x66ef,
        0x8567, 0x3f01, 0x1310, 0x6300, 0x6408, 0x6f01, 0x65ef, 0x66ee, 0x8567, 0x3f00, 0x1310,
        0x6300, 0x6409, 0x65f0, 0x660f, 0x8561, 0x35ff, 0x1310, 0x6301, 0x6400, 0x65f0, 0x660f,
        0x8562, 0x3500, 0x1310, 0x6301, 0x6401, 0x65f0, 0x660f, 0x8563, 0x35ff, 0x1310, 0x6f00,
        0x6301, 0x6402, 0x6581, 0x850e, 0x3f01, 0x1310, 0x6301, 0x6403, 0x6f01, 0x6547, 0x850e,
        0x3f00, 0x1310, 0x6301, 0x6404, 0x6f00, 0x6501, 0x8506, 0x3f01, 0x1310, 0x6301, 0x6405,
        0x6f01, 0x6502, 0x8506, 0x3f00, 0x1310, 0x6301, 0x6406, 0x6015, 0x6178, 0xa3d0, 0xf155,
        0xf165, 0x3015, 0x1310, 0x3178, 0x1310, 0x6301, 0x6407, 0x608a, 0xa3d0, 0xf033, 0xa3d0,
        0xf065, 0x3001, 0x1310, 0x6001, 0xf01e, 0xf065, 0x3003, 0x1310, 0x6001, 0xf01e, 0xf065,
        0x3008, 0x1310, 0x1332, 0x130e, 0xa32a, 0x6013, 0x6109, 0xd018, 0xf329, 0x6022, 0x610b,
        0xd015, 0xf429, 0x6028, 0x610b, 0xd015, 0x130e, 0xfff0, 0xf0ff, 0xf0f0, 0xf0ff, 0xa358,
        0x6015, 0x610b, 0x6308, 0xd018, 0x7008, 0xf31e, 0x302d, 0x133a, 0xa370, 0x6002, 0x6118,
        0x6308, 0xd018, 0x7005, 0xf31e, 0x303e, 0x134c, 0x130e, 0xf088, 0x88f0, 0x8888, 0x88f0,
        0x7884, 0x8484, 0x8484, 0x8478, 0x84c4, 0xa494, 0x8c84, 0x8484, 0xc0a0, 0xa0c0, 0xa0a0,
        0xc000, 0x0000, 0xa0a0, 0xe020, 0x20e0, 0x0000, 0x0000, 0x0000, 0x0000, 0xc0a0, 0xa0c0,
        0xa0a0, 0xc000, 0x0000, 0x60a0, 0xc080, 0x6000, 0x0000, 0x6080, 0x4020, 0xc000, 0x8080,
        0xc080, 0x8080, 0x6000, 0xe080, 0x8080, 0x8080, 0xe000, 0x0000, 0x40a0, 0xa0a0, 0x4000,
        0x2020, 0x2060, 0xa0a0, 0x6000, 0x0000, 0x60a0, 0xc080, 0x6000, 0x0000, 0x0060, 0x4040,
        0x5000, 0x0000, 0x0000, 0x0000,
    ];

    pub const RNG_TEST: [u16; 17] = [
        0x00e0, 0xc0ff, 0xa224, 0xf033, 0xf265, 0xf029, 0x6000, 0x6300, 0xd035, 0xf129, 0x6005,
        0xd035, 0xf229, 0x600a, 0xd035, 0xf00a, 0x1200,
    ];

    pub const JUMPING_X_O: [u16; 41] = [
        0xa24c, 0x6530, 0x6604, 0xd566, 0xa240, 0x2212, 0xa246, 0x2212, 0x1208, 0x611e, 0x620d,
        0xd125, 0x630c, 0xf315, 0xf407, 0x3400, 0x121c, 0x4f01, 0x122e, 0xd125, 0xc13f, 0xc21f,
        0x1216, 0xd125, 0x00ee, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0x8850,
        0x2050, 0x8800, 0xf888, 0x8888, 0xf800, 0xfcfc, 0xfcfc, 0xfcfc,
    ];
}

pub mod games {
    pub const BREAKOUT: [u16; 140] = [
        0x6e05, 0x6500, 0x6b06, 0x6a00, 0xa30c, 0xdab1, 0x7a04, 0x3a40, 0x1208, 0x7b02, 0x3b12,
        0x1206, 0x6c20, 0x6d1f, 0xa310, 0xdcd1, 0x22f6, 0x6000, 0x6100, 0xa312, 0xd011, 0x7008,
        0xa30e, 0xd011, 0x6040, 0xf015, 0xf007, 0x3000, 0x1234, 0xc60f, 0x671e, 0x6801, 0x69ff,
        0xa30e, 0xd671, 0xa310, 0xdcd1, 0x6004, 0xe0a1, 0x7cfe, 0x6006, 0xe0a1, 0x7c02, 0x603f,
        0x8c02, 0xdcd1, 0xa30e, 0xd671, 0x8684, 0x8794, 0x603f, 0x8602, 0x611f, 0x8712, 0x471f,
        0x12ac, 0x4600, 0x6801, 0x463f, 0x68ff, 0x4700, 0x6901, 0xd671, 0x3f01, 0x12aa, 0x471f,
        0x12aa, 0x6005, 0x8075, 0x3f00, 0x12aa, 0x6001, 0xf018, 0x8060, 0x61fc, 0x8012, 0xa30c,
        0xd071, 0x60fe, 0x8903, 0x22f6, 0x7501, 0x22f6, 0x4560, 0x12de, 0x1246, 0x69ff, 0x8060,
        0x80c5, 0x3f01, 0x12ca, 0x6102, 0x8015, 0x3f01, 0x12e0, 0x8015, 0x3f01, 0x12ee, 0x8015,
        0x3f01, 0x12e8, 0x6020, 0xf018, 0xa30e, 0x7eff, 0x80e0, 0x8004, 0x6100, 0xd011, 0x3e00,
        0x1230, 0x12de, 0x78ff, 0x48fe, 0x68ff, 0x12ee, 0x7801, 0x4802, 0x6801, 0x6004, 0xf018,
        0x69ff, 0x1270, 0xa314, 0xf533, 0xf265, 0xf129, 0x6337, 0x6400, 0xd345, 0x7305, 0xf229,
        0xd345, 0x00ee, 0xf000, 0x8000, 0xfc00, 0xaa00, 0x0000, 0x0000,
    ];
}

use convert_case::{Case, Casing};
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum Colors {
    #[strum(ascii_case_insensitive)]
    Black = 0,
    #[strum(ascii_case_insensitive)]
    Red = 1,
    #[strum(ascii_case_insensitive)]
    Green = 2,
    #[strum(ascii_case_insensitive)]
    Yellow = 3,
    #[strum(ascii_case_insensitive)]
    Blue = 4,
    #[strum(ascii_case_insensitive)]
    Magenta = 5,
    #[strum(ascii_case_insensitive)]
    Cyan = 6,
    #[strum(ascii_case_insensitive)]
    White = 7,
    #[strum(ascii_case_insensitive)]
    BrightBlack = 8,
    #[strum(ascii_case_insensitive)]
    BrightRed = 9,
    #[strum(ascii_case_insensitive)]
    BrightGreen = 10,
    #[strum(ascii_case_insensitive)]
    BrightYellow = 11,
    #[strum(ascii_case_insensitive)]
    BrightBlue = 12,
    #[strum(ascii_case_insensitive)]
    BrightMagenta = 13,
    #[strum(ascii_case_insensitive)]
    BrightCyan = 14,
    #[strum(ascii_case_insensitive)]
    BrightWhite = 15,
    #[strum(ascii_case_insensitive)]
    Grey0 = 16,
    #[strum(ascii_case_insensitive)]
    NavyBlue = 17,
    #[strum(ascii_case_insensitive)]
    DarkBlue = 18,
    #[strum(ascii_case_insensitive)]
    Blue3 = 20,
    #[strum(ascii_case_insensitive)]
    Blue1 = 21,
    #[strum(ascii_case_insensitive)]
    DarkGreen = 22,
    #[strum(ascii_case_insensitive)]
    DeepSkyBlue4 = 25,
    #[strum(ascii_case_insensitive)]
    DodgerBlue3 = 26,
    #[strum(ascii_case_insensitive)]
    DodgerBlue2 = 27,
    #[strum(ascii_case_insensitive)]
    Green4 = 28,
    #[strum(ascii_case_insensitive)]
    SpringGreen4 = 29,
    #[strum(ascii_case_insensitive)]
    Turquoise4 = 30,
    #[strum(ascii_case_insensitive)]
    DeepSkyBlue3 = 32,
    #[strum(ascii_case_insensitive)]
    DodgerBlue1 = 33,
    #[strum(ascii_case_insensitive)]
    Green3 = 40,
    #[strum(ascii_case_insensitive)]
    SpringGreen3 = 41,
    #[strum(ascii_case_insensitive)]
    DarkCyan = 36,
    #[strum(ascii_case_insensitive)]
    LightSeaGreen = 37,
    #[strum(ascii_case_insensitive)]
    DeepSkyBlue2 = 38,
    #[strum(ascii_case_insensitive)]
    DeepSkyBlue1 = 39,
    #[strum(ascii_case_insensitive)]
    SpringGreen2 = 47,
    #[strum(ascii_case_insensitive)]
    Cyan3 = 43,
    #[strum(ascii_case_insensitive)]
    DarkTurquoise = 44,
    #[strum(ascii_case_insensitive)]
    Turquoise2 = 45,
    #[strum(ascii_case_insensitive)]
    Green1 = 46,
    #[strum(ascii_case_insensitive)]
    SpringGreen1 = 48,
    #[strum(ascii_case_insensitive)]
    MediumSpringGreen = 49,
    #[strum(ascii_case_insensitive)]
    Cyan2 = 50,
    #[strum(ascii_case_insensitive)]
    Cyan1 = 51,
    #[strum(ascii_case_insensitive)]
    DarkRed = 88,
    #[strum(ascii_case_insensitive)]
    DeepPink4 = 125,
    #[strum(ascii_case_insensitive)]
    Purple4 = 55,
    #[strum(ascii_case_insensitive)]
    Purple3 = 56,
    #[strum(ascii_case_insensitive)]
    BlueViolet = 57,
    #[strum(ascii_case_insensitive)]
    Orange4 = 94,
    #[strum(ascii_case_insensitive)]
    Grey37 = 59,
    #[strum(ascii_case_insensitive)]
    MediumPurple4 = 60,
    #[strum(ascii_case_insensitive)]
    SlateBlue3 = 62,
    #[strum(ascii_case_insensitive)]
    RoyalBlue1 = 63,
    #[strum(ascii_case_insensitive)]
    Chartreuse4 = 64,
    #[strum(ascii_case_insensitive)]
    DarkSeaGreen4 = 71,
    #[strum(ascii_case_insensitive)]
    PaleTurquoise4 = 66,
    #[strum(ascii_case_insensitive)]
    SteelBlue = 67,
    #[strum(ascii_case_insensitive)]
    SteelBlue3 = 68,
    #[strum(ascii_case_insensitive)]
    CornflowerBlue = 69,
    #[strum(ascii_case_insensitive)]
    Chartreuse3 = 76,
    #[strum(ascii_case_insensitive)]
    CadetBlue = 73,
    #[strum(ascii_case_insensitive)]
    SkyBlue3 = 74,
    #[strum(ascii_case_insensitive)]
    SteelBlue1 = 81,
    #[strum(ascii_case_insensitive)]
    PaleGreen3 = 114,
    #[strum(ascii_case_insensitive)]
    SeaGreen3 = 78,
    #[strum(ascii_case_insensitive)]
    Aquamarine3 = 79,
    #[strum(ascii_case_insensitive)]
    MediumTurquoise = 80,
    #[strum(ascii_case_insensitive)]
    Chartreuse2 = 112,
    #[strum(ascii_case_insensitive)]
    SeaGreen2 = 83,
    #[strum(ascii_case_insensitive)]
    SeaGreen1 = 85,
    #[strum(ascii_case_insensitive)]
    Aquamarine1 = 122,
    #[strum(ascii_case_insensitive)]
    DarkSlateGray2 = 87,
    #[strum(ascii_case_insensitive)]
    DarkMagenta = 91,
    #[strum(ascii_case_insensitive)]
    DarkViolet = 128,
    #[strum(ascii_case_insensitive)]
    Purple = 129,
    #[strum(ascii_case_insensitive)]
    LightPink4 = 95,
    #[strum(ascii_case_insensitive)]
    Plum4 = 96,
    #[strum(ascii_case_insensitive)]
    MediumPurple3 = 98,
    #[strum(ascii_case_insensitive)]
    SlateBlue1 = 99,
    #[strum(ascii_case_insensitive)]
    Yellow4 = 106,
    #[strum(ascii_case_insensitive)]
    Wheat4 = 101,
    #[strum(ascii_case_insensitive)]
    Grey53 = 102,
    #[strum(ascii_case_insensitive)]
    LightSlateGrey = 103,
    #[strum(ascii_case_insensitive)]
    MediumPurple = 104,
    #[strum(ascii_case_insensitive)]
    LightSlateBlue = 105,
    #[strum(ascii_case_insensitive)]
    DarkOliveGreen3 = 149,
    #[strum(ascii_case_insensitive)]
    DarkSeaGreen = 108,
    #[strum(ascii_case_insensitive)]
    LightSkyBlue3 = 110,
    #[strum(ascii_case_insensitive)]
    SkyBlue2 = 111,
    #[strum(ascii_case_insensitive)]
    DarkSeaGreen3 = 150,
    #[strum(ascii_case_insensitive)]
    DarkSlateGray3 = 116,
    #[strum(ascii_case_insensitive)]
    SkyBlue1 = 117,
    #[strum(ascii_case_insensitive)]
    Chartreuse1 = 118,
    #[strum(ascii_case_insensitive)]
    LightGreen = 120,
    #[strum(ascii_case_insensitive)]
    PaleGreen1 = 156,
    #[strum(ascii_case_insensitive)]
    DarkSlateGray1 = 123,
    #[strum(ascii_case_insensitive)]
    Red3 = 160,
    #[strum(ascii_case_insensitive)]
    MediumVioletRed = 126,
    #[strum(ascii_case_insensitive)]
    Magenta3 = 164,
    #[strum(ascii_case_insensitive)]
    DarkOrange3 = 166,
    #[strum(ascii_case_insensitive)]
    IndianRed = 167,
    #[strum(ascii_case_insensitive)]
    HotPink3 = 168,
    #[strum(ascii_case_insensitive)]
    MediumOrchid3 = 133,
    #[strum(ascii_case_insensitive)]
    MediumOrchid = 134,
    #[strum(ascii_case_insensitive)]
    MediumPurple2 = 140,
    #[strum(ascii_case_insensitive)]
    DarkGoldenrod = 136,
    #[strum(ascii_case_insensitive)]
    LightSalmon3 = 173,
    #[strum(ascii_case_insensitive)]
    RosyBrown = 138,
    #[strum(ascii_case_insensitive)]
    Grey63 = 139,
    #[strum(ascii_case_insensitive)]
    MediumPurple1 = 141,
    #[strum(ascii_case_insensitive)]
    Gold3 = 178,
    #[strum(ascii_case_insensitive)]
    DarkKhaki = 143,
    #[strum(ascii_case_insensitive)]
    NavajoWhite3 = 144,
    #[strum(ascii_case_insensitive)]
    Grey69 = 145,
    #[strum(ascii_case_insensitive)]
    LightSteelBlue3 = 146,
    #[strum(ascii_case_insensitive)]
    LightSteelBlue = 147,
    #[strum(ascii_case_insensitive)]
    Yellow3 = 184,
    #[strum(ascii_case_insensitive)]
    DarkSeaGreen2 = 157,
    #[strum(ascii_case_insensitive)]
    LightCyan3 = 152,
    #[strum(ascii_case_insensitive)]
    LightSkyBlue1 = 153,
    #[strum(ascii_case_insensitive)]
    GreenYellow = 154,
    #[strum(ascii_case_insensitive)]
    DarkOliveGreen2 = 155,
    #[strum(ascii_case_insensitive)]
    DarkSeaGreen1 = 193,
    #[strum(ascii_case_insensitive)]
    PaleTurquoise1 = 159,
    #[strum(ascii_case_insensitive)]
    DeepPink3 = 162,
    #[strum(ascii_case_insensitive)]
    Magenta2 = 200,
    #[strum(ascii_case_insensitive)]
    HotPink2 = 169,
    #[strum(ascii_case_insensitive)]
    Orchid = 170,
    #[strum(ascii_case_insensitive)]
    MediumOrchid1 = 207,
    #[strum(ascii_case_insensitive)]
    Orange3 = 172,
    #[strum(ascii_case_insensitive)]
    LightPink3 = 174,
    #[strum(ascii_case_insensitive)]
    Pink3 = 175,
    #[strum(ascii_case_insensitive)]
    Plum3 = 176,
    #[strum(ascii_case_insensitive)]
    Violet = 177,
    #[strum(ascii_case_insensitive)]
    LightGoldenrod3 = 179,
    #[strum(ascii_case_insensitive)]
    Tan = 180,
    #[strum(ascii_case_insensitive)]
    MistyRose3 = 181,
    #[strum(ascii_case_insensitive)]
    Thistle3 = 182,
    #[strum(ascii_case_insensitive)]
    Plum2 = 183,
    #[strum(ascii_case_insensitive)]
    Khaki3 = 185,
    #[strum(ascii_case_insensitive)]
    LightGoldenrod2 = 222,
    #[strum(ascii_case_insensitive)]
    LightYellow3 = 187,
    #[strum(ascii_case_insensitive)]
    Grey84 = 188,
    #[strum(ascii_case_insensitive)]
    LightSteelBlue1 = 189,
    #[strum(ascii_case_insensitive)]
    Yellow2 = 190,
    #[strum(ascii_case_insensitive)]
    DarkOliveGreen1 = 192,
    #[strum(ascii_case_insensitive)]
    Honeydew2 = 194,
    #[strum(ascii_case_insensitive)]
    LightCyan1 = 195,
    #[strum(ascii_case_insensitive)]
    Red1 = 196,
    #[strum(ascii_case_insensitive)]
    DeepPink2 = 197,
    #[strum(ascii_case_insensitive)]
    DeepPink1 = 199,
    #[strum(ascii_case_insensitive)]
    Magenta1 = 201,
    #[strum(ascii_case_insensitive)]
    OrangeRed1 = 202,
    #[strum(ascii_case_insensitive)]
    IndianRed1 = 204,
    #[strum(ascii_case_insensitive)]
    HotPink = 206,
    #[strum(ascii_case_insensitive)]
    DarkOrange = 208,
    #[strum(ascii_case_insensitive)]
    Salmon1 = 209,
    #[strum(ascii_case_insensitive)]
    LightCoral = 210,
    #[strum(ascii_case_insensitive)]
    PaleVioletRed1 = 211,
    #[strum(ascii_case_insensitive)]
    Orchid2 = 212,
    #[strum(ascii_case_insensitive)]
    Orchid1 = 213,
    #[strum(ascii_case_insensitive)]
    Orange1 = 214,
    #[strum(ascii_case_insensitive)]
    SandyBrown = 215,
    #[strum(ascii_case_insensitive)]
    LightSalmon1 = 216,
    #[strum(ascii_case_insensitive)]
    LightPink1 = 217,
    #[strum(ascii_case_insensitive)]
    Pink1 = 218,
    #[strum(ascii_case_insensitive)]
    Plum1 = 219,
    #[strum(ascii_case_insensitive)]
    Gold1 = 220,
    #[strum(ascii_case_insensitive)]
    NavajoWhite1 = 223,
    #[strum(ascii_case_insensitive)]
    MistyRose1 = 224,
    #[strum(ascii_case_insensitive)]
    Thistle1 = 225,
    #[strum(ascii_case_insensitive)]
    Yellow1 = 226,
    #[strum(ascii_case_insensitive)]
    LightGoldenrod1 = 227,
    #[strum(ascii_case_insensitive)]
    Khaki1 = 228,
    #[strum(ascii_case_insensitive)]
    Wheat1 = 229,
    #[strum(ascii_case_insensitive)]
    Cornsilk1 = 230,
    #[strum(ascii_case_insensitive)]
    Grey100 = 231,
    #[strum(ascii_case_insensitive)]
    Grey3 = 232,
    #[strum(ascii_case_insensitive)]
    Grey7 = 233,
    #[strum(ascii_case_insensitive)]
    Grey11 = 234,
    #[strum(ascii_case_insensitive)]
    Grey15 = 235,
    #[strum(ascii_case_insensitive)]
    Grey19 = 236,
    #[strum(ascii_case_insensitive)]
    Grey23 = 237,
    #[strum(ascii_case_insensitive)]
    Grey27 = 238,
    #[strum(ascii_case_insensitive)]
    Grey30 = 239,
    #[strum(ascii_case_insensitive)]
    Grey35 = 240,
    #[strum(ascii_case_insensitive)]
    Grey39 = 241,
    #[strum(ascii_case_insensitive)]
    Grey42 = 242,
    #[strum(ascii_case_insensitive)]
    Grey46 = 243,
    #[strum(ascii_case_insensitive)]
    Grey50 = 244,
    #[strum(ascii_case_insensitive)]
    Grey54 = 245,
    #[strum(ascii_case_insensitive)]
    Gray58 = 246,
    #[strum(ascii_case_insensitive)]
    Gray62 = 247,
    #[strum(ascii_case_insensitive)]
    Gray66 = 248,
    #[strum(ascii_case_insensitive)]
    Gray70 = 249,
    #[strum(ascii_case_insensitive)]
    Gray74 = 250,
    #[strum(ascii_case_insensitive)]
    Gray78 = 251,
    #[strum(ascii_case_insensitive)]
    Gray82 = 252,
    #[strum(ascii_case_insensitive)]
    Gray85 = 253,
    #[strum(ascii_case_insensitive)]
    Gray89 = 254,
    #[strum(ascii_case_insensitive)]
    Gray93 = 255,
}
pub fn find_color(color: &str) -> i32 {
    let color_variant = Colors::from_str(color.to_case(Case::Pascal).as_str()).unwrap();
    color_variant as i32
}

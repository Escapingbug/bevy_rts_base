#![allow(unused)]
use bevy::prelude::Color;

pub struct Colors;
impl Colors {
    pub fn print_color(color: Color) {
        print!(
            "\x1b[38;2;{};{};{}m",
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8
        );
        print!(
            "\n{};{};{}\n",
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8
        );
        print!(
            "RGBA: {}, {}, {}, {}",
            color.r(),
            color.g(),
            color.b(),
            color.a()
        );
        println!("\x1b[0m");
    }
}

pub struct Tailwind;
impl Tailwind {
    pub const BLACK: Color = Color::rgb_linear(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::rgb_linear(1.0, 1.0, 1.0);

    pub const GRAY100: Color = Color::rgb_linear(247.0 / 255.0, 250.0 / 255.0, 252.0 / 255.0);
    pub const GRAY200: Color = Color::rgb_linear(237.0 / 255.0, 242.0 / 255.0, 247.0 / 255.0);
    pub const GRAY300: Color = Color::rgb_linear(226.0 / 255.0, 232.0 / 255.0, 240.0 / 255.0);
    pub const GRAY400: Color = Color::rgb_linear(203.0 / 255.0, 213.0 / 255.0, 224.0 / 255.0);
    pub const GRAY500: Color = Color::rgb_linear(160.0 / 255.0, 174.0 / 255.0, 192.0 / 255.0);
    pub const GRAY600: Color = Color::rgb_linear(113.0 / 255.0, 128.0 / 255.0, 150.0 / 255.0);
    pub const GRAY700: Color = Color::rgb_linear(74.0 / 255.0, 85.0 / 255.0, 104.0 / 255.0);
    pub const GRAY800: Color = Color::rgb_linear(45.0 / 255.0, 55.0 / 255.0, 72.0 / 255.0);
    pub const GRAY900: Color = Color::rgb_linear(26.0 / 255.0, 32.0 / 255.0, 44.0 / 255.0);

    pub const RED100: Color = Color::rgb_linear(255.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0);
    pub const RED200: Color = Color::rgb_linear(254.0 / 255.0, 215.0 / 255.0, 215.0 / 255.0);
    pub const RED300: Color = Color::rgb_linear(254.0 / 255.0, 178.0 / 255.0, 178.0 / 255.0);
    pub const RED400: Color = Color::rgb_linear(252.0 / 255.0, 129.0 / 255.0, 129.0 / 255.0);
    pub const RED500: Color = Color::rgb_linear(245.0 / 255.0, 101.0 / 255.0, 101.0 / 255.0);
    pub const RED600: Color = Color::rgb_linear(229.0 / 255.0, 62.0 / 255.0, 62.0 / 255.0);
    pub const RED700: Color = Color::rgb_linear(197.0 / 255.0, 48.0 / 255.0, 48.0 / 255.0);
    pub const RED800: Color = Color::rgb_linear(155.0 / 255.0, 44.0 / 255.0, 44.0 / 255.0);
    pub const RED900: Color = Color::rgb_linear(116.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0);

    pub const ORANGE100: Color = Color::rgb_linear(255.0 / 255.0, 250.0 / 255.0, 240.0 / 255.0);
    pub const ORANGE200: Color = Color::rgb_linear(254.0 / 255.0, 235.0 / 255.0, 200.0 / 255.0);
    pub const ORANGE300: Color = Color::rgb_linear(251.0 / 255.0, 211.0 / 255.0, 141.0 / 255.0);
    pub const ORANGE400: Color = Color::rgb_linear(246.0 / 255.0, 173.0 / 255.0, 85.0 / 255.0);
    pub const ORANGE500: Color = Color::rgb_linear(237.0 / 255.0, 137.0 / 255.0, 54.0 / 255.0);
    pub const ORANGE600: Color = Color::rgb_linear(221.0 / 255.0, 107.0 / 255.0, 32.0 / 255.0);
    pub const ORANGE700: Color = Color::rgb_linear(192.0 / 255.0, 86.0 / 255.0, 33.0 / 255.0);
    pub const ORANGE800: Color = Color::rgb_linear(156.0 / 255.0, 66.0 / 255.0, 33.0 / 255.0);
    pub const ORANGE900: Color = Color::rgb_linear(123.0 / 255.0, 52.0 / 255.0, 30.0 / 255.0);

    pub const YELLOW100: Color = Color::rgb_linear(255.0 / 255.0, 255.0 / 255.0, 240.0 / 255.0);
    pub const YELLOW200: Color = Color::rgb_linear(254.0 / 255.0, 252.0 / 255.0, 191.0 / 255.0);
    pub const YELLOW300: Color = Color::rgb_linear(250.0 / 255.0, 240.0 / 255.0, 137.0 / 255.0);
    pub const YELLOW400: Color = Color::rgb_linear(246.0 / 255.0, 224.0 / 255.0, 94.0 / 255.0);
    pub const YELLOW500: Color = Color::rgb_linear(236.0 / 255.0, 201.0 / 255.0, 75.0 / 255.0);
    pub const YELLOW600: Color = Color::rgb_linear(214.0 / 255.0, 158.0 / 255.0, 46.0 / 255.0);
    pub const YELLOW700: Color = Color::rgb_linear(183.0 / 255.0, 121.0 / 255.0, 31.0 / 255.0);
    pub const YELLOW800: Color = Color::rgb_linear(151.0 / 255.0, 90.0 / 255.0, 22.0 / 255.0);
    pub const YELLOW900: Color = Color::rgb_linear(116.0 / 255.0, 66.0 / 255.0, 16.0 / 255.0);

    pub const GREEN100: Color = Color::rgb_linear(240.0 / 255.0, 255.0 / 255.0, 244.0 / 255.0);
    pub const GREEN200: Color = Color::rgb_linear(198.0 / 255.0, 246.0 / 255.0, 213.0 / 255.0);
    pub const GREEN300: Color = Color::rgb_linear(154.0 / 255.0, 230.0 / 255.0, 180.0 / 255.0);
    pub const GREEN400: Color = Color::rgb_linear(104.0 / 255.0, 211.0 / 255.0, 145.0 / 255.0);
    pub const GREEN500: Color = Color::rgb_linear(72.0 / 255.0, 187.0 / 255.0, 120.0 / 255.0);
    pub const GREEN600: Color = Color::rgb_linear(56.0 / 255.0, 161.0 / 255.0, 105.0 / 255.0);
    pub const GREEN700: Color = Color::rgb_linear(47.0 / 255.0, 133.0 / 255.0, 90.0 / 255.0);
    pub const GREEN800: Color = Color::rgb_linear(39.0 / 255.0, 103.0 / 255.0, 73.0 / 255.0);
    pub const GREEN900: Color = Color::rgb_linear(34.0 / 255.0, 84.0 / 255.0, 61.0 / 255.0);

    pub const TEAL100: Color = Color::rgb_linear(230.0 / 255.0, 255.0 / 255.0, 250.0 / 255.0);
    pub const TEAL200: Color = Color::rgb_linear(178.0 / 255.0, 245.0 / 255.0, 234.0 / 255.0);
    pub const TEAL300: Color = Color::rgb_linear(129.0 / 255.0, 230.0 / 255.0, 217.0 / 255.0);
    pub const TEAL400: Color = Color::rgb_linear(79.0 / 255.0, 209.0 / 255.0, 197.0 / 255.0);
    pub const TEAL500: Color = Color::rgb_linear(56.0 / 255.0, 178.0 / 255.0, 172.0 / 255.0);
    pub const TEAL600: Color = Color::rgb_linear(49.0 / 255.0, 151.0 / 255.0, 149.0 / 255.0);
    pub const TEAL700: Color = Color::rgb_linear(44.0 / 255.0, 122.0 / 255.0, 123.0 / 255.0);
    pub const TEAL800: Color = Color::rgb_linear(40.0 / 255.0, 94.0 / 255.0, 97.0 / 255.0);
    pub const TEAL900: Color = Color::rgb_linear(35.0 / 255.0, 78.0 / 255.0, 82.0 / 255.0);

    pub const BLUE100: Color = Color::rgb_linear(235.0 / 255.0, 248.0 / 255.0, 255.0 / 255.0);
    pub const BLUE200: Color = Color::rgb_linear(190.0 / 255.0, 227.0 / 255.0, 248.0 / 255.0);
    pub const BLUE300: Color = Color::rgb_linear(144.0 / 255.0, 205.0 / 255.0, 244.0 / 255.0);
    pub const BLUE400: Color = Color::rgb_linear(99.0 / 255.0, 179.0 / 255.0, 237.0 / 255.0);
    pub const BLUE500: Color = Color::rgb_linear(66.0 / 255.0, 153.0 / 255.0, 225.0 / 255.0);
    pub const BLUE600: Color = Color::rgb_linear(49.0 / 255.0, 130.0 / 255.0, 206.0 / 255.0);
    pub const BLUE700: Color = Color::rgb_linear(43.0 / 255.0, 108.0 / 255.0, 176.0 / 255.0);
    pub const BLUE800: Color = Color::rgb_linear(44.0 / 255.0, 82.0 / 255.0, 130.0 / 255.0);
    pub const BLUE900: Color = Color::rgb_linear(42.0 / 255.0, 67.0 / 255.0, 101.0 / 255.0);

    pub const INDIGO100: Color = Color::rgb_linear(235.0 / 255.0, 244.0 / 255.0, 255.0 / 255.0);
    pub const INDIGO200: Color = Color::rgb_linear(195.0 / 255.0, 218.0 / 255.0, 254.0 / 255.0);
    pub const INDIGO300: Color = Color::rgb_linear(163.0 / 255.0, 191.0 / 255.0, 250.0 / 255.0);
    pub const INDIGO400: Color = Color::rgb_linear(127.0 / 255.0, 156.0 / 255.0, 245.0 / 255.0);
    pub const INDIGO500: Color = Color::rgb_linear(102.0 / 255.0, 126.0 / 255.0, 234.0 / 255.0);
    pub const INDIGO600: Color = Color::rgb_linear(90.0 / 255.0, 103.0 / 255.0, 216.0 / 255.0);
    pub const INDIGO700: Color = Color::rgb_linear(76.0 / 255.0, 81.0 / 255.0, 191.0 / 255.0);
    pub const INDIGO800: Color = Color::rgb_linear(67.0 / 255.0, 65.0 / 255.0, 144.0 / 255.0);
    pub const INDIGO900: Color = Color::rgb_linear(60.0 / 255.0, 54.0 / 255.0, 107.0 / 255.0);

    pub const PURPLE100: Color = Color::rgb_linear(250.0 / 255.0, 245.0 / 255.0, 255.0 / 255.0);
    pub const PURPLE200: Color = Color::rgb_linear(233.0 / 255.0, 216.0 / 255.0, 253.0 / 255.0);
    pub const PURPLE300: Color = Color::rgb_linear(214.0 / 255.0, 188.0 / 255.0, 250.0 / 255.0);
    pub const PURPLE400: Color = Color::rgb_linear(183.0 / 255.0, 148.0 / 255.0, 244.0 / 255.0);
    pub const PURPLE500: Color = Color::rgb_linear(159.0 / 255.0, 122.0 / 255.0, 234.0 / 255.0);
    pub const PURPLE600: Color = Color::rgb_linear(128.0 / 255.0, 90.0 / 255.0, 213.0 / 255.0);
    pub const PURPLE700: Color = Color::rgb_linear(107.0 / 255.0, 70.0 / 255.0, 193.0 / 255.0);
    pub const PURPLE800: Color = Color::rgb_linear(85.0 / 255.0, 60.0 / 255.0, 154.0 / 255.0);
    pub const PURPLE900: Color = Color::rgb_linear(68.0 / 255.0, 51.0 / 255.0, 122.0 / 255.0);

    pub const PINK100: Color = Color::rgb_linear(255.0 / 255.0, 245.0 / 255.0, 247.0 / 255.0);
    pub const PINK200: Color = Color::rgb_linear(254.0 / 255.0, 215.0 / 255.0, 226.0 / 255.0);
    pub const PINK300: Color = Color::rgb_linear(251.0 / 255.0, 182.0 / 255.0, 206.0 / 255.0);
    pub const PINK400: Color = Color::rgb_linear(246.0 / 255.0, 135.0 / 255.0, 179.0 / 255.0);
    pub const PINK500: Color = Color::rgb_linear(237.0 / 255.0, 100.0 / 255.0, 166.0 / 255.0);
    pub const PINK600: Color = Color::rgb_linear(213.0 / 255.0, 63.0 / 255.0, 140.0 / 255.0);
    pub const PINK700: Color = Color::rgb_linear(184.0 / 255.0, 50.0 / 255.0, 128.0 / 255.0);
    pub const PINK800: Color = Color::rgb_linear(151.0 / 255.0, 38.0 / 255.0, 109.0 / 255.0);
    pub const PINK900: Color = Color::rgb_linear(112.0 / 255.0, 36.0 / 255.0, 89.0 / 255.0);
}

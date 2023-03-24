#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub struct RgbaF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub sRGB: bool,
}

#[allow(non_snake_case, dead_code)]
impl RgbaF {
    pub fn new(r: f32, g: f32, b: f32, a: f32, sRGB: bool) -> RgbaF {
        RgbaF { r, g, b, a, sRGB }
    }
    fn sRGB(value: f32, inverse: bool) -> f32 {
        if inverse {
            if value <= 0.04045 {
                (25.0 * value) / 323.0
            } else {
                (((200.0 * value) + 11.0) / 211.0).powf(12.0 / 5.0)
            }
        } else {
            if value <= 0.0031308 {
                (323.0 * value) / 25.0
            } else {
                (211.0 * value.powf(5.0 / 12.0) - 11.0) / 200.0
            }
        }
    }

    // from https://en.wikipedia.org/wiki/HSL_and_HSV
    pub fn f_hsv(h: f32, s: f32, v: f32, n: f32) -> f32 {
        let k = (n + (h / 60.0)) % 6.0;
        v - (v * s * (0.0f32).max((k).min((4.0 - k).min(1.0))))
    }
    pub fn from_hsv(h: f32, s: f32, v: f32, a: f32) -> RgbaF {
        let h = h % 360.0;
        RgbaF {
            r: RgbaF::f_hsv(h, s, v, 5.0),
            g: RgbaF::f_hsv(h, s, v, 3.0),
            b: RgbaF::f_hsv(h, s, v, 1.0),
            a: a,
            sRGB: false,
        }
    }

    pub fn f_hsl(h: f32, s: f32, l: f32, n: f32) -> f32 {
        let k = (n + (h / 30.0)) % 12.0;
        let a = s * l.min(1.0 - l);
        l - (a * (-1.0f32).max((k - 3.0).min((1.0f32).min(9.0 - k))))
    }
    pub fn from_hsl(h: f32, s: f32, l: f32, a: f32) -> RgbaF {
        let h = h % 360.0;
        RgbaF {
            r: RgbaF::f_hsl(h, s, l, 0.0),
            g: RgbaF::f_hsl(h, s, l, 8.0),
            b: RgbaF::f_hsl(h, s, l, 4.0),
            a: a,
            sRGB: false,
        }
    }

    pub fn to_sRGB(&self) -> RgbaF {
        RgbaF {
            r: RgbaF::sRGB(self.r, false),
            g: RgbaF::sRGB(self.g, false),
            b: RgbaF::sRGB(self.b, false),
            a: RgbaF::sRGB(self.a, true),
            sRGB: true,
        }
    }
    pub fn to_RGB(&self) -> RgbaF {
        RgbaF {
            r: RgbaF::sRGB(self.r, true),
            g: RgbaF::sRGB(self.g, true),
            b: RgbaF::sRGB(self.b, true),
            a: RgbaF::sRGB(self.a, true),
            sRGB: false,
        }
    }
    pub fn to_arr(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    pub fn to_arr16(&self) -> [u16; 4] {
        [
            (self.r * u16::MAX as f32) as u16,
            (self.g * u16::MAX as f32) as u16,
            (self.b * u16::MAX as f32) as u16,
            (self.a * u16::MAX as f32) as u16,
        ]
    }
    pub fn to_arr8(&self) -> [u8; 4] {
        [
            (self.r * u8::MAX as f32) as u8,
            (self.g * u8::MAX as f32) as u8,
            (self.b * u8::MAX as f32) as u8,
            (self.a * u8::MAX as f32) as u8,
        ]
    }
}

// RgbaF operators
impl std::ops::Add<RgbaF> for RgbaF {
    type Output = RgbaF;
    fn add(self, mut _rhs: RgbaF) -> RgbaF {
        if self.sRGB != _rhs.sRGB {
            match self.sRGB {
                true => _rhs = _rhs.to_sRGB(),
                false => _rhs = _rhs.to_RGB(),
            }
        }
        RgbaF {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
            a: self.a + _rhs.a,
            sRGB: self.sRGB,
        }
    }
}
impl std::ops::Sub<RgbaF> for RgbaF {
    type Output = RgbaF;
    fn sub(self, mut _rhs: RgbaF) -> RgbaF {
        if self.sRGB != _rhs.sRGB {
            match self.sRGB {
                true => _rhs = _rhs.to_sRGB(),
                false => _rhs = _rhs.to_RGB(),
            }
        }
        RgbaF {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b,
            a: self.a - _rhs.a,
            sRGB: self.sRGB,
        }
    }
}
impl std::ops::Mul<RgbaF> for RgbaF {
    type Output = RgbaF;
    fn mul(self, mut _rhs: RgbaF) -> RgbaF {
        if self.sRGB != _rhs.sRGB {
            match self.sRGB {
                true => _rhs = _rhs.to_sRGB(),
                false => _rhs = _rhs.to_RGB(),
            }
        }
        RgbaF {
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b,
            a: self.a * _rhs.a,
            sRGB: self.sRGB,
        }
    }
}

// f32 operators
impl std::ops::Add<f32> for RgbaF {
    type Output = RgbaF;
    fn add(self, _rhs: f32) -> RgbaF {
        RgbaF {
            r: self.r + _rhs,
            g: self.g + _rhs,
            b: self.b + _rhs,
            a: self.a + _rhs,
            sRGB: self.sRGB,
        }
    }
}
impl std::ops::Sub<f32> for RgbaF {
    type Output = RgbaF;
    fn sub(self, _rhs: f32) -> RgbaF {
        RgbaF {
            r: self.r - _rhs,
            g: self.g - _rhs,
            b: self.b - _rhs,
            a: self.a - _rhs,
            sRGB: self.sRGB,
        }
    }
}
impl std::ops::Mul<f32> for RgbaF {
    type Output = RgbaF;
    fn mul(self, _rhs: f32) -> RgbaF {
        RgbaF {
            r: self.r * _rhs,
            g: self.g * _rhs,
            b: self.b * _rhs,
            a: self.a * _rhs,
            sRGB: self.sRGB,
        }
    }
}
impl std::ops::Div<f32> for RgbaF {
    type Output = RgbaF;
    fn div(self, _rhs: f32) -> RgbaF {
        RgbaF {
            r: self.r / _rhs,
            g: self.g / _rhs,
            b: self.b / _rhs,
            a: self.a / _rhs,
            sRGB: self.sRGB,
        }
    }
}

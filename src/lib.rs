fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055)/1.055).powf(2.4) }
}

fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0/2.4) - 0.055 }
}

pub fn get_table() -> [u8;65536] {
    let mut table = [0;65536];

    let mut s2l = [0;65536];
    let mut l2s = [0;65536];
    for c in 0..65536 {
        s2l[c] = (srgb_to_linear(c as f32 / 65535.0) * 65535.0).round() as u16;
        l2s[c] = (linear_to_srgb(c as f32 / 65535.0) * 65535.0).round() as u16;
    }

    let startbg: i32 = 0;
    let endbg: i32 = 256 * 0x101;
    let mut index = 0;

    for fg in (0..65536).step_by(0x101) {
        let mut startac = 0_i32;
        for a in 0..256 {
            let mut besterror = 0xffff_ffff_u32;
            let mut bestac = 0_i32;

            let ca = a as i32;
            for ac in startac..256 {
                let mut error = 0_u32;
                for bg in (startbg..endbg).step_by(0x101) {
                    let linear_blended: i32 = (ac * fg + (255 - ac) * bg + 128) / 255;
                    let srgb_blended: i32 = l2s[((ca * s2l[fg as usize] as i32
                                                     + (255 - ca) * s2l[bg as usize] as i32 + 128) / 255) as usize] as _;

                    let difference: i32 = (linear_blended - srgb_blended) >> 4;
                    error += (difference * difference) as u32;
                }

                if error <= besterror {
                    besterror = error;
                    bestac = ac;
                } else {
                    break
                }
            }

            startac = bestac;
            table[index] = bestac as u8;
            index += 1;
        }
    }
    table
}

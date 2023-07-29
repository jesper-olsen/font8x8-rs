// font8x8: 606 8x8 bitmaps with unicode mappings

// The bitmaps in this file are ported from
// https://github.com/dhepper/font8x8

pub const UNICODE_ALL: std::ops::Range<u16> = 0x00..0xe55a;
pub const UNICODE_ASCII: std::ops::Range<u16> = 0x0..0x7f;
pub const UNICODE_LATIN: std::ops::Range<u16> = 0x00..0xff;
pub const UNICODE_GREEK: std::ops::Range<u16> = 0x390..0x3c9;
pub const UNICODE_HIRAGANA: std::ops::Range<u16> = 0x3040..0x309f;
pub const UNICODE_SGA: std::ops::Range<u16> = 0xe541..0xe55a;
pub const UNICODE_BOX: std::ops::Range<u16> = 0x2500..0x259f;

pub fn index2unicode(x: usize) -> Option<usize> {
    match x {
        0x0..=0xff => Some(0x0 + x), // latin
        0x100..=0x100 => Some(0x92 + x),
        0x101..=0x101 => Some(0x1ff + x),
        0x102..=0x13b => Some(0x28e + x), // greek
        0x13c..=0x13d => Some(0x1db6 + x),
        0x13e..=0x13e => Some(0x1f69 + x),
        0x13f..=0x140 => Some(0x2126 + x),
        0x141..=0x141 => Some(0x21cf + x),
        0x142..=0x1e1 => Some(0x23be + x), // box
        0x1e2..=0x241 => Some(0x2e5e + x), // hiragana
        0x242..=0x25b => Some(0xe2ff + x), // SGA
        604 => Some(0x00aa),               // duplicate unicode bitmap
        605 => Some(0x00ba),               // duplicate unicode bitmap
        _ => None,
    }
}

pub fn unicode2index(x: usize) -> Option<usize> {
    match x {
        0x0..=0xff => Some(x), // latin
        0x192..=0x192 => Some(x - 0x92),
        0x300..=0x300 => Some(x - 0x1ff),
        0x390..=0x3c9 => Some(x - 0x28e), // greek
        0x1ef2..=0x1ef3 => Some(x - 0x1db6),
        0x20a7..=0x20a7 => Some(x - 0x1f69),
        0x2265..=0x2266 => Some(x - 0x2126),
        0x2310..=0x2310 => Some(x - 0x21cf),
        0x2500..=0x259f => Some(x - 0x23be),
        0x3040..=0x309f => Some(x - 0x2e5e), // hiragana
        0xe541..=0xe55a => Some(x - 0xe2ff),
        _ => None,
    }
}

pub fn unicode2bitmap(y: u16) -> u64 {
    if let Some(i) = unicode2index(y.into()) {
        FONT[i as usize]
    } else {
        0x0
    }
}

pub fn display(bm: u64) {
    for x in (0..8).rev() {
        for y in 0..8 {
            if bm & 1 << (x * 8 + y) != 0 {
                print!("\u{1f7e6}"); // blue
            } else {
                print!("\u{1f7e8}"); // yellow
            }
        }
        println!()
    }
    println!()
}

pub const FONT: [u64; 606] = [
    0x0000000000000000, // U+0000 (nul)
    0x0000000000000000, // U+0001
    0x0000000000000000, // U+0002
    0x0000000000000000, // U+0003
    0x0000000000000000, // U+0004
    0x0000000000000000, // U+0005
    0x0000000000000000, // U+0006
    0x0000000000000000, // U+0007
    0x0000000000000000, // U+0008
    0x0000000000000000, // U+0009
    0x0000000000000000, // U+000A
    0x0000000000000000, // U+000B
    0x0000000000000000, // U+000C
    0x0000000000000000, // U+000D
    0x0000000000000000, // U+000E
    0x0000000000000000, // U+000F
    0x0000000000000000, // U+0010
    0x0000000000000000, // U+0011
    0x0000000000000000, // U+0012
    0x0000000000000000, // U+0013
    0x0000000000000000, // U+0014
    0x0000000000000000, // U+0015
    0x0000000000000000, // U+0016
    0x0000000000000000, // U+0017
    0x0000000000000000, // U+0018
    0x0000000000000000, // U+0019
    0x0000000000000000, // U+001A
    0x0000000000000000, // U+001B
    0x0000000000000000, // U+001C
    0x0000000000000000, // U+001D
    0x0000000000000000, // U+001E
    0x0000000000000000, // U+001F
    0x0000000000000000, // U+0020 (space)
    0x183C3C1818001800, // U+0021 (!)
    0x3636000000000000, // U+0022 (")
    0x36367F367F363600, // U+0023 (#)
    0x0C3E031E301F0C00, // U+0024 ($)
    0x006333180C666300, // U+0025 (%)
    0x1C361C6E3B336E00, // U+0026 (&)
    0x0606030000000000, // U+0027 (')
    0x180C0606060C1800, // U+0028 (()
    0x060C1818180C0600, // U+0029 ())
    0x00663CFF3C660000, // U+002A (*)
    0x000C0C3F0C0C0000, // U+002B (+)
    0x00000000000C0C06, // U+002C (,)
    0x0000003F00000000, // U+002D (-)
    0x00000000000C0C00, // U+002E (.)
    0x6030180C06030100, // U+002F (/)
    0x3E63737B6F673E00, // U+0030 (0)
    0x0C0E0C0C0C0C3F00, // U+0031 (1)
    0x1E33301C06333F00, // U+0032 (2)
    0x1E33301C30331E00, // U+0033 (3)
    0x383C36337F307800, // U+0034 (4)
    0x3F031F3030331E00, // U+0035 (5)
    0x1C06031F33331E00, // U+0036 (6)
    0x3F3330180C0C0C00, // U+0037 (7)
    0x1E33331E33331E00, // U+0038 (8)
    0x1E33333E30180E00, // U+0039 (9)
    0x000C0C00000C0C00, // U+003A (:)
    0x000C0C00000C0C06, // U+003B (;)
    0x180C0603060C1800, // U+003C (<)
    0x00003F00003F0000, // U+003D (=)
    0x060C1830180C0600, // U+003E (>)
    0x1E3330180C000C00, // U+003F (?)
    0x3E637B7B7B031E00, // U+0040 (@)
    0x0C1E33333F333300, // U+0041 (A)
    0x3F66663E66663F00, // U+0042 (B)
    0x3C66030303663C00, // U+0043 (C)
    0x1F36666666361F00, // U+0044 (D)
    0x7F46161E16467F00, // U+0045 (E)
    0x7F46161E16060F00, // U+0046 (F)
    0x3C66030373667C00, // U+0047 (G)
    0x3333333F33333300, // U+0048 (H)
    0x1E0C0C0C0C0C1E00, // U+0049 (I)
    0x7830303033331E00, // U+004A (J)
    0x6766361E36666700, // U+004B (K)
    0x0F06060646667F00, // U+004C (L)
    0x63777F7F6B636300, // U+004D (M)
    0x63676F7B73636300, // U+004E (N)
    0x1C36636363361C00, // U+004F (O)
    0x3F66663E06060F00, // U+0050 (P)
    0x1E3333333B1E3800, // U+0051 (Q)
    0x3F66663E36666700, // U+0052 (R)
    0x1E33070E38331E00, // U+0053 (S)
    0x3F2D0C0C0C0C1E00, // U+0054 (T)
    0x3333333333333F00, // U+0055 (U)
    0x33333333331E0C00, // U+0056 (V)
    0x6363636B7F776300, // U+0057 (W)
    0x6363361C1C366300, // U+0058 (X)
    0x3333331E0C0C1E00, // U+0059 (Y)
    0x7F6331184C667F00, // U+005A (Z)
    0x1E06060606061E00, // U+005B ()
    0x03060C1830604000, // U+005C (\)
    0x1E18181818181E00, // U+005D ()
    0x081C366300000000, // U+005E (^)
    0x00000000000000FF, // U+005F (_)
    0x0C0C180000000000, // U+0060 (`)
    0x00001E303E336E00, // U+0061 (a)
    0x0706063E66663B00, // U+0062 (b)
    0x00001E3303331E00, // U+0063 (c)
    0x3830303e33336E00, // U+0064 (d)
    0x00001E333f031E00, // U+0065 (e)
    0x1C36060f06060F00, // U+0066 (f)
    0x00006E33333E301F, // U+0067 (g)
    0x0706366E66666700, // U+0068 (h)
    0x0C000E0C0C0C1E00, // U+0069 (i)
    0x300030303033331E, // U+006A (j)
    0x070666361E366700, // U+006B (k)
    0x0E0C0C0C0C0C1E00, // U+006C (l)
    0x0000337F7F6B6300, // U+006D (m)
    0x00001F3333333300, // U+006E (n)
    0x00001E3333331E00, // U+006F (o)
    0x00003B66663E060F, // U+0070 (p)
    0x00006E33333E3078, // U+0071 (q)
    0x00003B6E66060F00, // U+0072 (r)
    0x00003E031E301F00, // U+0073 (s)
    0x080C3E0C0C2C1800, // U+0074 (t)
    0x0000333333336E00, // U+0075 (u)
    0x00003333331E0C00, // U+0076 (v)
    0x0000636B7F7F3600, // U+0077 (w)
    0x000063361C366300, // U+0078 (x)
    0x00003333333E301F, // U+0079 (y)
    0x00003F190C263F00, // U+007A (z)
    0x380C0C070C0C3800, // U+007B ()
    0x1818180018181800, // U+007C (|)
    0x070C0C380C0C0700, // U+007D ()
    0x6E3B000000000000, // U+007E (~)
    0x0000000000000000, // U+007F
    0x0000000000000000, // U+0080
    0x0000000000000000, // U+0081
    0x0000000000000000, // U+0082
    0x0000000000000000, // U+0083
    0x0000000000000000, // U+0084
    0x0000000000000000, // U+0085
    0x0000000000000000, // U+0086
    0x0000000000000000, // U+0087
    0x0000000000000000, // U+0088
    0x0000000000000000, // U+0089
    0x0000000000000000, // U+008A
    0x0000000000000000, // U+008B
    0x0000000000000000, // U+008C
    0x0000000000000000, // U+008D
    0x0000000000000000, // U+008E
    0x0000000000000000, // U+008F
    0x0000000000000000, // U+0090
    0x0000000000000000, // U+0091
    0x0000000000000000, // U+0092
    0x0000000000000000, // U+0093
    0x0000000000000000, // U+0094
    0x0000000000000000, // U+0095
    0x0000000000000000, // U+0096
    0x0000000000000000, // U+0097
    0x0000000000000000, // U+0098
    0x0000000000000000, // U+0099
    0x0000000000000000, // U+009A
    0x0000000000000000, // U+009B
    0x0000000000000000, // U+009C
    0x0000000000000000, // U+009D
    0x0000000000000000, // U+009E
    0x0000000000000000, // U+009F
    0x0000000000000000, // U+00A0 (no break space)
    0x1818001818181800, // U+00A1 (inverted !)
    0x18187E03037E1818, // U+00A2 (dollarcents)
    0x1C36260F06673F00, // U+00A3 (pound sterling)
    0x0000633E363E6300, // U+00A4 (currency mark)
    0x33331E3F0C3F0C0C, // U+00A5 (yen)
    0x1818180018181800, // U+00A6 (broken pipe)
    0x7CC61C36361C331E, // U+00A7 (paragraph)
    0x3300000000000000, // U+00A8 (diaeresis)
    0x3C4299858599423C, // U+00A9 (copyright symbol)
    0x3C36367C00000000, // U+00AA (superscript a)
    0x00CC663366CC0000, // U+00AB (<<)
    0x0000003F30300000, // U+00AC (gun pointing left)
    0x0000000000000000, // U+00AD (soft hyphen)
    0x3C429DA59DA5423C, // U+00AE (registered symbol)
    0x7E00000000000000, // U+00AF (macron)
    0x1C36361C00000000, // U+00B0 (degree)
    0x18187E1818007E00, // U+00B1 (plusminus)
    0x1C30180C3C000000, // U+00B2 (superscript 2)
    0x1C3018301C000000, // U+00B3 (superscript 3)
    0x180C000000000000, // U+00B4 (aigu)
    0x00006666663E0603, // U+00B5 (mu)
    0xFEDBDBDED8D8D800, // U+00B6 (pilcrow)
    0x0000001818000000, // U+00B7 (central dot)
    0x000000000018301E, // U+00B8 (cedille)
    0x080C081C00000000, // U+00B9 (superscript 1)
    0x1C36361C00000000, // U+00BA (superscript 0)
    0x003366CC66330000, // U+00BB (>>)
    0xC36333BDECF6F303, // U+00BC (1/4)
    0xC363337BCC6633F0, // U+00BD (1/2)
    0x03C463B4DBACE680, // U+00BE (3/4)
    0x0C000C0603331E00, // U+00BF (inverted ?)
    0x07001C36637F6300, // U+00C0 (A grave)
    0x70001C36637F6300, // U+00C1 (A aigu)
    0x1C36003E637F6300, // U+00C2 (A circumflex)
    0x6E3B003E637F6300, // U+00C3 (A ~)
    0x631C36637F636300, // U+00C4 (A umlaut)
    0x0C0C001E333F3300, // U+00C5 (A ring)
    0x7C36337F33337300, // U+00C6 (AE)
    0x1E3303331E18301E, // U+00C7 (C cedille)
    0x07003F061E063F00, // U+00C8 (E grave)
    0x38003F061E063F00, // U+00C9 (E aigu)
    0x0C123F061E063F00, // U+00CA (E circumflex)
    0x36003F061E063F00, // U+00CB (E umlaut)
    0x07001E0C0C0C1E00, // U+00CC (I grave)
    0x38001E0C0C0C1E00, // U+00CD (I aigu)
    0x0C12001E0C0C1E00, // U+00CE (I circumflex)
    0x33001E0C0C0C1E00, // U+00CF (I umlaut)
    0x3F666F6F66663F00, // U+00D0 (Eth)
    0x3F0033373F3B3300, // U+00D1 (N ~)
    0x0E00183C663C1800, // U+00D2 (O grave)
    0x7000183C663C1800, // U+00D3 (O aigu)
    0x3C66183C663C1800, // U+00D4 (O circumflex)
    0x6E3B003E63633E00, // U+00D5 (O ~)
    0xC3183C66663C1800, // U+00D6 (O umlaut)
    0x00361C081C360000, // U+00D7 (multiplicative x)
    0x5C36737B6F361D00, // U+00D8 (O stroke)
    0x0E00666666663C00, // U+00D9 (U grave)
    0x7000666666663C00, // U+00DA (U aigu)
    0x3C66006666663C00, // U+00DB (U circumflex)
    0x3300333333331E00, // U+00DC (U umlaut)
    0x700066663C181800, // U+00DD (Y aigu)
    0x0F063E66663E060F, // U+00DE (Thorn)
    0x001E331F331F0303, // U+00DF (beta)
    0x07001E303E337E00, // U+00E0 (a grave)
    0x38001E303E337E00, // U+00E1 (a aigu)
    0x7EC33C607C66FC00, // U+00E2 (a circumflex)
    0x6E3B1E303E337E00, // U+00E3 (a ~)
    0x33001E303E337E00, // U+00E4 (a umlaut)
    0x0C0C1E303E337E00, // U+00E5 (a ring)
    0x0000FE30FE33FE00, // U+00E6 (ae)
    0x00001E03031E301C, // U+00E7 (c cedille)
    0x07001E333F031E00, // U+00E8 (e grave)
    0x38001E333F031E00, // U+00E9 (e aigu)
    0x7EC33C667E063C00, // U+00EA (e circumflex)
    0x33001E333F031E00, // U+00EB (e umlaut)
    0x07000E0C0C0C1E00, // U+00EC (i grave)
    0x1C000E0C0C0C1E00, // U+00ED (i augu)
    0x3E631C1818183C00, // U+00EE (i circumflex)
    0x33000E0C0C0C1E00, // U+00EF (i umlaut)
    0x1B0E1B303E331E00, // U+00F0 (eth)
    0x001F001F33333300, // U+00F1 (n ~)
    0x0007001E33331E00, // U+00F2 (o grave)
    0x0038001E33331E00, // U+00F3 (o aigu)
    0x1E33001E33331E00, // U+00F4 (o circumflex)
    0x6E3B001E33331E00, // U+00F5 (o ~)
    0x0033001E33331E00, // U+00F6 (o umlaut)
    0x1818007E00181800, // U+00F7 (division)
    0x00603C767E6E3C06, // U+00F8 (o stroke)
    0x0007003333337E00, // U+00F9 (u grave)
    0x0038003333337E00, // U+00FA (u aigu)
    0x1E33003333337E00, // U+00FB (u circumflex)
    0x0033003333337E00, // U+00FC (u umlaut)
    0x00380033333E301F, // U+00FD (y aigu)
    0x0000063E663E0600, // U+00FE (thorn)
    0x00330033333E301F, // U+00FF (y umlaut)
    0x70D8183C18181B0E, // U+0192 (dutch florijn)
    0x0C18000000000000, // U+0300 (grave)
    0x2D000C0C0C2C1800, // U+0390 (iota with tonos and diaeresis)
    0x0C1E33333F333300, // U+0391 (Alpha)
    0x3F66663E66663F00, // U+0392 (Beta)
    0x3F33030303030300, // U+0393 (Gamma)
    0x081C1C3636637F00, // U+0394 (Delta)
    0x7F46161E16467F00, // U+0395 (Epsilon)
    0x7F6331184C667F00, // U+0396 (Zeta)
    0x3333333F33333300, // U+0397 (Eta)
    0x1C36637F63361C00, // U+0398 (Theta)
    0x1E0C0C0C0C0C1E00, // U+0399 (Iota)
    0x6766361E36666700, // U+039A (Kappa)
    0x081C1C3636636300, // U+039B (Lambda)
    0x63777F7F6B636300, // U+039C (Mu)
    0x63676F7B73636300, // U+039D (Nu)
    0x7F63003E00637F00, // U+039E (Xi)
    0x1C36636363361C00, // U+039F (Omikron)
    0x7F36363636363600, // U+03A0 (Pi)
    0x3F66663E06060F00, // U+03A1 (Rho)
    0x000102044F90A040, // U+03A2
    0x7F63060C06637F00, // U+03A3 (Sigma 2)
    0x3F2D0C0C0C0C1E00, // U+03A4 (Tau)
    0x3333331E0C0C1E00, // U+03A5 (Upsilon)
    0x187EDBDBDB7E1800, // U+03A6 (Phi)
    0x6363361C36636300, // U+03A7 (Chi)
    0xDBDBDB7E18183C00, // U+03A8 (Psi)
    0x3E63636336367700, // U+03A9 (Omega)
    0x33001E0C0C0C1E00, // U+03AA (Iota with diaeresis)
    0x330033331E0C1E00, // U+03AB (Upsilon with diaeresis)
    0x70006E3B133B6E00, // U+03AC (alpha aigu)
    0x38001E030E031E00, // U+03AD (epsilon aigu)
    0x38001F3333333330, // U+03AE (eta aigu)
    0x38000C0C0C2C1800, // U+03AF (iota aigu)
    0x2D00333333331E00, // U+03B0 (upsilon with tonos and diaeresis)
    0x00006E3B133B6E00, // U+03B1 (alpha)
    0x001E331F331F0303, // U+03B2 (beta)
    0x000033331E0C0C00, // U+03B3 (gamma)
    0x380C183E33331E00, // U+03B4 (delta)
    0x00001E030E031E00, // U+03B5 (epsilon)
    0x003F0603031E301C, // U+03B6 (zeta)
    0x00001F3333333330, // U+03B7 (eta)
    0x00001E333F331E00, // U+03B8 (theta)
    0x00000C0C0C2C1800, // U+03B9 (iota)
    0x0000331B0F1B3300, // U+03BA (kappa)
    0x0003060C1C366300, // U+03BB (lambda)
    0x00006666663E0603, // U+03BC (mu)
    0x00003333331E0C00, // U+03BD (nu)
    0x1E030E03031E301C, // U+03BE (xi)
    0x00001E3333331E00, // U+03BF (omikron)
    0x00007F3636363600, // U+03C0 (pi)
    0x00003C6666360606, // U+03C1 (rho)
    0x00003E03031E301C, // U+03C2 (sigma 1)
    0x00007E1B1B1B0E00, // U+03C3 (sigma 2)
    0x00007E1818583000, // U+03C4 (tau)
    0x0000333333331E00, // U+03C5 (upsilon)
    0x000076DBDB7E1800, // U+03C6 (phi)
    0x0063361C1C366300, // U+03C7 (chi)
    0x0000DBDBDB7E1800, // U+03C8 (psi)
    0x000036636B7F3600, // U+03C9 (omega)
    0x0E0066663C181800, // U+1EF2 (Y grave)
    0x00070033333E301F, // U+1EF3 (y grave)
    0x1F33335F63F363E3, // U+20A7 (Spanish Pesetas/Pt)
    0x0C1830180C007E00, // U+2265 (greater than or equal)
    0x30180C1830007E00, // U+2266 (less than or equal)
    0x0000003F03030000, // U+2310 (gun pointing right)
    0x00000000FF000000, // U+2500 (thin horizontal)
    0x000000FFFF000000, // U+2501 (thick horizontal)
    0x0808080808080808, // U+2502 (thin vertical)
    0x1818181818181818, // U+2503 (thich vertical)
    0x00000000BB000000, // U+2504 (thin horizontal dashed)
    0x000000BBBB000000, // U+2505 (thick horizontal dashed)
    0x0800080808000808, // U+2506 (thin vertical dashed)
    0x1800181818001818, // U+2507 (thich vertical dashed)
    0x0000000055000000, // U+2508 (thin horizontal dotted)
    0x0000005555000000, // U+2509 (thick horizontal dotted)
    0x0008000800080008, // U+250A (thin vertical dotted)
    0x0018001800180018, // U+250B (thich vertical dotted)
    0x00000000f8080808, // U+250C (down L, right L)
    0x000000f8f8080808, // U+250D (down L, right H)
    0x00000000f8181818, // U+250E (down H, right L)
    0x000000f8f8181818, // U+250F (down H, right H)
    0x000000000f080808, // U+2510 (down L, left L)
    0x0000000f0f080808, // U+2511 (down L, left H)
    0x000000001f181818, // U+2512 (down H, left L)
    0x0000001f1f181818, // U+2513 (down H, left H)
    0x08080808f8000000, // U+2514 (up L, right L)
    0x080808f8f8000000, // U+2515 (up L, right H)
    0x18181818f8000000, // U+2516 (up H, right L)
    0x181818f8f8000000, // U+2517 (up H, right H)
    0x080808080f000000, // U+2518 (up L, left L)
    0x0808080f0f000000, // U+2519 (up L, left H)
    0x181818181f000000, // U+251A (up H, left L)
    0x1818181f1f000000, // U+251B (up H, left H)
    0x08080808f8080808, // U+251C (down L, right L, up L)
    0x080808f8f8080808, // U+251D (down L, right H, up L)
    0x18181818f8080808, // U+251E (down L, right L, up H)
    0x08080808f8181818, // U+251F (down H, right L, up L)
    0x18181818f8181818, // U+2520 (down H, right L, up H)
    0x181818f8f8080808, // U+2521 (down L, right H, up H)
    0x080808f8f8181818, // U+2522 (down H, right H, up L)
    0x181818f8f8181818, // U+2523 (down H, right H, up H)
    0x080808080f080808, // U+2524 (down L, left L, up L)
    0x0808080f0f080808, // U+2525 (down L, left H, up L)
    0x181818181f080808, // U+2526 (down L, left L, up H)
    0x080808081f181818, // U+2527 (down H, left L, up L)
    0x181818181f181818, // U+2528 (down H, left L, up H)
    0x1818181f1f080808, // U+2529 (down L, left H, up H)
    0x0808081f1f181818, // U+252A (down H, left H, up L)
    0x1818181f1f181818, // U+252B (down H, left H, up H)
    0x00000000ff080808, // U+252C (down L, right L, left L)
    0x0000000fff080808, // U+252D (down L, right L, left H)
    0x000000f8ff080808, // U+252E (down L, right H, left L)
    0x000000ffff080808, // U+252F (down L, right H, left H)
    0x00000000ff181818, // U+2530 (down H, right L, left L)
    0x0000001fff181818, // U+2531 (down H, right L, left H)
    0x000000f8ff181818, // U+2532 (down H, right H, left L)
    0x000000ffff181818, // U+2533 (down H, right H, left H)
    0x08080808ff000000, // U+2534 (up L, right L, left L)
    0x0808080fff000000, // U+2535 (up L, right L, left H)
    0x080808f8ff000000, // U+2536 (up L, right H, left L)
    0x080808ffff000000, // U+2537 (up L, right H, left H)
    0x18181818ff000000, // U+2538 (up H, right L, left L)
    0x1818181fff000000, // U+2539 (up H, right L, left H)
    0x181818f8ff000000, // U+253A (up H, right H, left L)
    0x181818ffff000000, // U+253B (up H, right H, left H)
    0x08080808ff080808, // U+253C (up L, right L, left L, down L)
    0x0808080fff080808, // U+253D (up L, right L, left H, down L)
    0x080808f8ff080808, // U+253E (up L, right H, left L, down L)
    0x080808ffff080808, // U+253F (up L, right H, left H, down L)
    0x18181818ff080808, // U+2540 (up H, right L, left L, down L)
    0x08080808ff181818, // U+2541 (up L, right L, left L, down H)
    0x18181818ff181818, // U+2542 (up H, right L, left L, down H)
    0x1818181fff080808, // U+2543 (up H, right L, left H, down L)
    0x181818f8ff080808, // U+2544 (up H, right H, left L, down L)
    0x0808081fff181818, // U+2545 (up L, right L, left H, down H)
    0x080808f8ff181818, // U+2546 (up L, right H, left L, down H)
    0x181818ffff181818, // U+2547 (up H, right H, left H, down H)
    0x080808ffff181818, // U+2548 (up L, right H, left H, down H)
    0x1818181fff181818, // U+2549 (up H, right L, left H, down H)
    0x181818f8ff181818, // U+254A (up H, right H, left L, down H)
    0x181818ffff080808, // U+254B (up H, right H, left H, down L)
    0x00000000E7000000, // U+254C (thin horizontal broken)
    0x000000E7E7000000, // U+254D (thick horizontal broken)
    0x0808080000080808, // U+254E (thin vertical broken)
    0x1818180000181818, // U+254F (thich vertical broken)
    0x000000FF00FF0000, // U+2550 (double horizontal)
    0x1414141414141414, // U+2551 (double vertical)
    0x000000F808F80808, // U+2552 (down L, right D)
    0x00000000FC141414, // U+2553 (down D, right L)
    0x000000FC04F41414, // U+2554 (down D, right D)
    0x0000000F080F0808, // U+2555 (down L, left D)
    0x000000001F141414, // U+2556 (down D, left L)
    0x0000001F10171414, // U+2557 (down D, left D)
    0x080808F808F80000, // U+2558 (up L, right D)
    0x14141414FC000000, // U+2559 (up D, right L)
    0x141414F404FC0000, // U+255A (up D, right D)
    0x0808080F080F0000, // U+255B (up L, left D)
    0x141414141F000000, // U+255C (up D, left L)
    0x14141417101F0000, // U+255D (up D, left D)
    0x080808F808F80808, // U+255E (up L, down L, right D)
    0x14141414F4141414, // U+255F (up D, down D, right L)
    0x141414F404F41414, // U+2560 (up D, down D, right D)
    0x0808080F080F0808, // U+2561 (up L, down L, left D)
    0x1414141417141414, // U+2562 (up D, down D, left L)
    0x1414141710171414, // U+2563 (up D, down D, left D)
    0x000000FF00FF0808, // U+2564 (left D, right D, down L)
    0x00000000FF141414, // U+2565 (left L, right L, down D)
    0x000000FF00F71414, // U+2566 (left D, right D, down D)
    0x080808FF00FF0000, // U+2567 (left D, right D, up L)
    0x14141414FF000000, // U+2568 (left L, right L, up D)
    0x141414F700FF0000, // U+2569 (left D, right D, up D)
    0x080808FF08FF0808, // U+256A (left D, right D, down L, up L)
    0x14141414FF141414, // U+256B (left L, right L, down D, up D)
    0x141414F700F71414, // U+256C (left D, right D, down D, up D)
    0x00000000E0100808, // U+256D (curve down-right)
    0x0000000003040808, // U+256E (curve down-left)
    0x0808080403000000, // U+256F (curve up-left)
    0x08080810E0000000, // U+2570 (curve up-right)
    0x8040201008040201, // U+2571 (diagonal bottom-left to top-right)
    0x0102040810204080, // U+2572 (diagonal bottom-right to top-left)
    0x8142241818244281, // U+2573 (diagonal cross)
    0x000000000F000000, // U+2574 (left L)
    0x0808080800000000, // U+2575 (up L)
    0x00000000F8000000, // U+2576 (right L)
    0x0000000008080808, // U+2577 (down L)
    0x0000000F0F000000, // U+2578 (left H)
    0x1818181800000000, // U+2579 (up H)
    0x000000F8F8000000, // U+257A (right H)
    0x0000000018181818, // U+257B (down H)
    0x000000F8FF000000, // U+257C (right H, left L)
    0x0808080818181818, // U+257D (up L, down H)
    0x0000000FFF000000, // U+257E (right L, left H)
    0x1818181808080808, // U+257F (up H, down L)
    0xFFFFFFFF00000000, // U+2580 (top half)
    0x00000000000000FF, // U+2581 (box 1/8)
    0x000000000000FFFF, // U+2582 (box 2/8)
    0x0000000000FFFFFF, // U+2583 (box 3/8)
    0x00000000FFFFFFFF, // U+2584 (bottom half)
    0x000000FFFFFFFFFF, // U+2585 (box 5/8)
    0x0000FFFFFFFFFFFF, // U+2586 (box 6/8)
    0x00FFFFFFFFFFFFFF, // U+2587 (box 7/8)
    0xFFFFFFFFFFFFFFFF, // U+2588 (solid)
    0x7F7F7F7F7F7F7F7F, // U+2589 (box 7/8)
    0x3F3F3F3F3F3F3F3F, // U+258A (box 6/8)
    0x1F1F1F1F1F1F1F1F, // U+258B (box 5/8)
    0x0F0F0F0F0F0F0F0F, // U+258C (left half)
    0x0707070707070707, // U+258D (box 3/8)
    0x0303030303030303, // U+258E (box 2/8)
    0x0101010101010101, // U+258F (box 1/8)
    0xF0F0F0F0F0F0F0F0, // U+2590 (right half)
    0x5500AA005500AA00, // U+2591 (25% solid)
    0x55AA55AA55AA55AA, // U+2592 (50% solid)
    0xFFAAFF55FFAAFF55, // U+2593 (75% solid)
    0xFF00000000000000, // U+2594 (box 1/8)
    0x8080808080808080, // U+2595 (box 1/8)
    0x000000000F0F0F0F, // U+2596 (box bottom left)
    0x00000000F0F0F0F0, // U+2597 (box bottom right)
    0x0F0F0F0F00000000, // U+2598 (box top left)
    0x0F0F0F0FFFFFFFFF, // U+2599 (boxes left and bottom)
    0x0F0F0F0FF0F0F0F0, // U+259A (boxes top-left and bottom right)
    0xFFFFFFFF0F0F0F0F, // U+259B (boxes top and left)
    0xFFFFFFFFF0F0F0F0, // U+259C (boxes top and right)
    0xF0F0F0F000000000, // U+259D (box top right)
    0xF0F0F0F00F0F0F0F, // U+259E (boxes top right and bottom left)
    0xF0F0F0F0FFFFFFFF, // U+259F (boxes right and bottom)
    0x0000000000000000, // U+3040
    0x043F043C564D2600, // U+3041 (Hiragana a)
    0x043F043C564D2600, // U+3042 (Hiragana A)
    0x0000001121250200, // U+3043 (Hiragana i)
    0x0001112121250200, // U+3044 (Hiragana I)
    0x001C001C22201800, // U+3045 (Hiragana u)
    0x3C003C4240201800, // U+3046 (Hiragana U)
    0x1C003E1038246200, // U+3047 (Hiragana e)
    0x1C003E1038246200, // U+3048 (Hiragana E)
    0x244F043C46452200, // U+3049 (Hiragana o)
    0x244F043C46452200, // U+304A (Hiragana O)
    0x04244F5452120900, // U+304B (Hiragana KA)
    0x44240F5452520900, // U+304C (Hiragana GA)
    0x081F083F1C023C00, // U+304D (Hiragana KI)
    0x442F041F0E011E00, // U+304E (Hiragana GI)
    0x1008040204081000, // U+304F (Hiragana KU)
    0x2844122102040800, // U+3050 (Hiragana GU)
    0x0022792121221000, // U+3051 (Hiragana KE)
    0x4022113D11120800, // U+3052 (Hiragana GE)
    0x00003C0002023C00, // U+3053 (Hiragana KO)
    0x2040162001010E00, // U+3054 (Hiragana GO)
    0x107E103C02021C00, // U+3055 (Hiragana SA)
    0x244F142E01010E00, // U+3056 (Hiragana ZA)
    0x0002020242221C00, // U+3057 (Hiragana SI)
    0x2042122202221C00, // U+3058 (Hiragana ZI)
    0x107E181418100C00, // U+3059 (Hiragana SU)
    0x442F060506040300, // U+305A (Hiragana ZU)
    0x20722F221A021C00, // U+305B (Hiragana SE)
    0x80503A171A021C00, // U+305C (Hiragana ZE)
    0x1E08047F08043800, // U+305D (Hiragana SO)
    0x4F24027F08043800, // U+305E (Hiragana ZO)
    0x020F027202097100, // U+305F (Hiragana TA)
    0x422F027202097100, // U+3060 (Hiragana DA)
    0x087E083C40403800, // U+3061 (Hiragana TI)
    0x442F041E20201C00, // U+3062 (Hiragana DI)
    0x0000001C22201C00, // U+3063 (Hiragana tu)
    0x001C224140201C00, // U+3064 (Hiragana TU)
    0x40201E2120201C00, // U+3065 (Hiragana DU)
    0x003E080404043800, // U+3066 (Hiragana TE)
    0x003E482404043800, // U+3067 (Hiragana DE)
    0x0404083C02023C00, // U+3068 (Hiragana TO)
    0x4424083C02023C00, // U+3069 (Hiragana DO)
    0x3202272272291100, // U+306A (Hiragana NA)
    0x00027A020A720200, // U+306B (Hiragana NI)
    0x08093E4B65552200, // U+306C (Hiragana NU)
    0x0407344C66542400, // U+306D (Hiragana NE)
    0x00003C4A49452200, // U+306E (Hiragana NO)
    0x00227A22722A1200, // U+306F (Hiragana HA)
    0x80511D1139150900, // U+3070 (Hiragana BA)
    0x40B15D1139150900, // U+3071 (Hiragana PA)
    0x0000133251110E00, // U+3072 (Hiragana HI)
    0x4020033251110E00, // U+3073 (Hiragana BI)
    0x40A0433251110E00, // U+3074 (Hiragana PI)
    0x1C00082A49100C00, // U+3075 (Hiragana HU)
    0x4C20082A49100C00, // U+3076 (Hiragana BU)
    0x4CA0480A29480C00, // U+3077 (Hiragana PU)
    0x0000040A11204000, // U+3078 (Hiragana HE)
    0x2040142A11204000, // U+3079 (Hiragana BE)
    0x2050240A11204000, // U+307A (Hiragana PE)
    0x7D117D1139550900, // U+307B (Hiragana HO)
    0x9D511D1139550900, // U+307C (Hiragana BO)
    0x5DB15D1139550900, // U+307D (Hiragana PO)
    0x7E083E081C2A0400, // U+307E (Hiragana MA)
    0x000724247E251200, // U+307F (Hiragana MI)
    0x040F640605263C00, // U+3080 (Hiragana MU)
    0x00093D4A4B452A00, // U+3081 (Hiragana ME)
    0x020F020F62423C00, // U+3082 (Hiragana MO)
    0x0000121F22120400, // U+3083 (Hiragana ya)
    0x00123F4242340400, // U+3084 (Hiragana YA)
    0x0000113D53391100, // U+3085 (Hiragana yu)
    0x00113D5351391100, // U+3086 (Hiragana YU)
    0x000838081C2A0400, // U+3087 (Hiragana yo)
    0x080838081C2A0400, // U+3088 (Hiragana YO)
    0x1E00023A46423000, // U+3089 (Hiragana RA)
    0x002022222A241000, // U+308A (Hiragana RI)
    0x1F083C4249543800, // U+308B (Hiragana RU)
    0x0407040C16552400, // U+308C (Hiragana RE)
    0x3F10083C42413000, // U+308D (Hiragana RO)
    0x0000080E384C2A00, // U+308E (Hiragana wa)
    0x0407043C46452400, // U+308F (Hiragana WA)
    0x0E083C4A69553200, // U+3090 (Hiragana WI)
    0x063C423904364900, // U+3091 (Hiragana WE)
    0x040F046E11087000, // U+3092 (Hiragana WO)
    0x0808040C56522100, // U+3093 (Hiragana N)
    0x402E003C42403800, // U+3094 (Hiragana VU)
    0x0000000000000000, // U+3095
    0x0000000000000000, // U+3096
    0x0000000000000000, // U+3097
    0x0000000000000000, // U+3098
    0x0000000000000000, // U+3099 (voiced combinator mark)
    0x0000000000000000, // U+309A (semivoiced combinator mark)
    0x4080204000000000, // U+309B (Hiragana voiced mark)
    0x40A0400000000000, // U+309C (Hiragana semivoiced mark)
    0x0000080810300C00, // U+309D (Hiragana iteration mark)
    0x2040142408180600, // U+309E (Hiragana voiced iteration mark)
    0x0000000000000000, // U+309F missing? ゟ
    0x0000386606060700, // U+E541 (SGA A)
    0x00000C0C18307F00, // U+E542 (SGA B)
    0x00000C000C303000, // U+E543 (SGA C)
    0x00007F00031C6000, // U+E544 (SGA D)
    0x0000630303037F00, // U+E545 (SGA E)
    0x000000FF00DB0000, // U+E546 (SGA F)
    0x000030303E303000, // U+E547 (SGA G)
    0x00007E007E181800, // U+E548 (SGA H)
    0x0000181800181800, // U+E549 (SGA I)
    0x0000180018001800, // U+E54A (SGA J)
    0x000018185A181800, // U+E54B (SGA K)
    0x0000033303330300, // U+E54C (SGA L)
    0x0000636060607F00, // U+E54D (SGA M)
    0x0000666030180C00, // U+E54E (SGA N)
    0x00003C6030180C00, // U+E54F (SGA O)
    0x0000666066066600, // U+E550 (SGA P)
    0x000018007E607E00, // U+E551 (SGA Q)
    0x0000006600660000, // U+E552 (SGA R)
    0x00000C0C3C303000, // U+E553 (SGA S)
    0x00003C3030003000, // U+E554 (SGA T)
    0x00000036007F0000, // U+E555 (SGA U)
    0x000018187E007E00, // U+E556 (SGA V)
    0x0000001800660000, // U+E557 (SGA W)
    0x00006630180C0600, // U+E558 (SGA X)
    0x0000363636363600, // U+E559 (SGA Y)
    0x0000183C66666600, // U+E55A (SGA Z)
    0x3C36367C007E0000, // U+00AA (duplicate - underlined f. nominal)
    0x1C36361C003E0000, // U+00BA (duplicate - underlined m. nominal)
];

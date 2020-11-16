const CBits = 8;

export const Sizes = {
    u8: 8 / CBits,
    u16: 16 / CBits,
    u32: 32 / CBits,
    u64: 64 / CBits,
    i8: 8 / CBits,
    i16: 16 / CBits,
    i32: 32 / CBits,
    i64: 64 / CBits,
    f32: 32 / CBits,
    f64: 64 / CBits,
    bool: 1,
}

export const Borders = {
    u8: { min: 0, max: 255 },
    u16: { min: 0, max: 65535 },
    u32: { min: 0, max: 4294967295 },
    u64: { min: 0, max: 18446744073709551615 },
    i8: { min: -128, max: 127 },
    i16: { min: -32768, max: 32767 },
    i32: { min: -2147483648, max: 2147483647 },
    i64: { min: -9223372036854775808, max: 9223372036854775807 },
    f32: 32 / CBits,
    f64: 64 / CBits,
    bool: 1,
}
import { u8 } from './protocol.primitives.u8';
import { u16 } from './protocol.primitives.u16';
import { u32 } from './protocol.primitives.u32';
import { u64 } from './protocol.primitives.u64';
import { i8 } from './protocol.primitives.i8';
import { i16 } from './protocol.primitives.i16';
import { i32 } from './protocol.primitives.i32';
import { i64 } from './protocol.primitives.i64';
import { f32 } from './protocol.primitives.f32';
import { f64 } from './protocol.primitives.f64';
import { StrUTF8 } from './protocol.primitives.string.utf8';
import { ArrayU8 } from './protocol.primitives.array.u8';
import { ArrayU16 } from './protocol.primitives.array.u16';
import { ArrayU32 } from './protocol.primitives.array.u32';
import { ArrayU64 } from './protocol.primitives.array.u64';
import { ArrayI8 } from './protocol.primitives.array.i8';
import { ArrayI16 } from './protocol.primitives.array.i16';
import { ArrayI32 } from './protocol.primitives.array.i32';
import { ArrayI64 } from './protocol.primitives.array.i64';
import { ArrayF32 } from './protocol.primitives.array.f32';
import { ArrayF64 } from './protocol.primitives.array.f64';
import { ArrayStrUTF8 } from './protocol.primitives.array.string.utf8';

export enum ETypes {
    u8 = 'u8',
    u16 = 'u16',
    u32 = 'u32',
    u64 = 'u64',
    i8 = 'i8',
    i16 = 'i16',
    i32 = 'i32',
    i64 = 'i64',
    f32 = 'f32',
    f64 = 'f64',
    StrUTF8 = 'StrUTF8',
    ArrayU8 = 'ArrayU8',
    ArrayU16 = 'ArrayU16',
    ArrayU32 = 'ArrayU32',
    ArrayU64 = 'ArrayU64',
    ArrayI8 = 'ArrayI8',
    ArrayI16 = 'ArrayI16',
    ArrayI32 = 'ArrayI32',
    ArrayI64 = 'ArrayI64',
    ArrayF32 = 'ArrayF32',
    ArrayF64 = 'ArrayF64',
    ArrayStrUTF8 = 'ArrayStrUTF8',
}

export const CTypesRefs = {
    [ETypes.u8]: u8,
    [ETypes.u16]: u16,
    [ETypes.u32]: u32,
    [ETypes.u64]: u64,
    [ETypes.i8]: i8,
    [ETypes.i16]: i16,
    [ETypes.i32]: i32,
    [ETypes.i64]: i64,
    [ETypes.f32]: f32,
    [ETypes.f64]: f64,
    [ETypes.StrUTF8]: StrUTF8,
    [ETypes.ArrayU8]: ArrayU8,
    [ETypes.ArrayU16]: ArrayU16,
    [ETypes.ArrayU32]: ArrayU32,
    [ETypes.ArrayU64]: ArrayU64,
    [ETypes.ArrayI8]: ArrayI8,
    [ETypes.ArrayI16]: ArrayI16,
    [ETypes.ArrayI32]: ArrayI32,
    [ETypes.ArrayI64]: ArrayI64,
    [ETypes.ArrayF32]: ArrayF32,
    [ETypes.ArrayF64]: ArrayF64,
    [ETypes.ArrayStrUTF8]: ArrayStrUTF8,
}

/*
| ID | Size rate | Size  | Value |
| 2  | 1         | 1 - 8 | -     |
*/
/*
fn get_value_buffer(id: u16, size: ESize, mut value: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = vec!();
    buffer.append(&mut id.to_le_bytes().to_vec());
    match size {
        ESize::U8(size) => {
            buffer.append(&mut (8 as u8).to_le_bytes().to_vec());
            buffer.append(&mut size.to_le_bytes().to_vec());
        },
        ESize::U16(size) => {
            buffer.append(&mut (16 as u8).to_le_bytes().to_vec());
            buffer.append(&mut size.to_le_bytes().to_vec());
        },
        ESize::U32(size) => {
            buffer.append(&mut (32 as u8).to_le_bytes().to_vec());
            buffer.append(&mut size.to_le_bytes().to_vec());
        },
        ESize::U64(size) => {
            buffer.append(&mut (64 as u8).to_le_bytes().to_vec());
            buffer.append(&mut size.to_le_bytes().to_vec());
        },
    };
    buffer.append(&mut value);
    Ok(buffer)
}
*/

import * as Primitives from './protocol.primitives';
import * as Tools from '../tools/index';

import { ESize, CBits } from './protocol.sizes';

export abstract class Encode {

    public getBuffer(id: number, esize: ESize, size: number | bigint, value: ArrayBufferLike | Error): ArrayBufferLike | Error {
        if (value instanceof Error) {
            return value;
        }
        const idBuf: ArrayBufferLike | Error = Primitives.u16.encode(id);
        if (idBuf instanceof Error) {
            return idBuf;
        }
        let sizeType: ArrayBufferLike | Error;
        let sizeValue: ArrayBufferLike | Error;
        if (esize === ESize.u64 && typeof size !== 'bigint') {
            return new Error(`For size ${ESize.u64}, size should be defined as BigInt`);
        } else if ((esize === ESize.u8 || esize === ESize.u16 || esize === ESize.u32) && typeof size === 'bigint') {
            return new Error(`For sizes ${ESize.u8}, ${ESize.u16}, ${ESize.u32}, size should be defined as Number`);
        }
        switch(esize) {
            case ESize.u8:
                sizeType = Primitives.u8.encode(Primitives.u8.getSize() * CBits);
                sizeValue = Primitives.u16.encode(size as number);
                break;
            case ESize.u16:
                sizeType = Primitives.u8.encode(Primitives.u16.getSize() * CBits);
                sizeValue = Primitives.u16.encode(size as number);
                break;
            case ESize.u32:
                sizeType = Primitives.u8.encode(Primitives.u32.getSize() * CBits);
                sizeValue = Primitives.u32.encode(size as number);
                break;
            case ESize.u64:
                sizeType = Primitives.u8.encode(Primitives.u64.getSize() * CBits);
                sizeValue = Primitives.u64.encode(size as bigint);
                break;
        }
        if (sizeType instanceof Error) {
            return sizeType;
        }
        if (sizeValue instanceof Error) {
            return sizeValue;
        }
        if (sizeType === undefined || sizeValue === undefined) {
            return new Error(`Size type or size value aren't defined`);
        }
        return Tools.append([idBuf, sizeType, sizeValue, value]);
    }

    public abstract getId(): number;
    public abstract encode(): ArrayBufferLike;

}
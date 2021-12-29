import { ProducerIdentificationStrategy, Logger } from "clibri";
import * as Protocol from "../protocol";

export class Identification {
    private readonly _uuid: string;
    private readonly _strategy: ProducerIdentificationStrategy;
    private _discredited: boolean = false;
    private _key: Protocol.StructA | undefined;
    private _assigned: Protocol.StructC | undefined;
    private _logger: Logger;

    constructor(
        uuid: string,
        strategy: ProducerIdentificationStrategy,
        logger: Logger
    ) {
        this._uuid = uuid;
        this._strategy = strategy;
        this._logger = logger.clone(`[${uuid}][Identification]`);
    }

    public uuid(): string {
        return this._uuid;
    }

    public key(
        key: Protocol.StructA,
        overwrite: boolean
    ): string {
        if (this._key === undefined || overwrite) {
            this._key = key;
        } else {            
            if (key.field_str !== undefined) {
                this._key.field_str = key.field_str;
            }
            if (key.field_str_empty !== undefined) {
                this._key.field_str_empty = key.field_str_empty;
            }
            if (key.field_u8 !== undefined) {
                this._key.field_u8 = key.field_u8;
            }
            if (key.field_u16 !== undefined) {
                this._key.field_u16 = key.field_u16;
            }
            if (key.field_u32 !== undefined) {
                this._key.field_u32 = key.field_u32;
            }
            if (key.field_u64 !== undefined) {
                this._key.field_u64 = key.field_u64;
            }
            if (key.field_i8 !== undefined) {
                this._key.field_i8 = key.field_i8;
            }
            if (key.field_i16 !== undefined) {
                this._key.field_i16 = key.field_i16;
            }
            if (key.field_i32 !== undefined) {
                this._key.field_i32 = key.field_i32;
            }
            if (key.field_i64 !== undefined) {
                this._key.field_i64 = key.field_i64;
            }
            if (key.field_f32 !== undefined) {
                this._key.field_f32 = key.field_f32;
            }
            if (key.field_f64 !== undefined) {
                this._key.field_f64 = key.field_f64;
            }
            if (key.field_bool !== undefined) {
                this._key.field_bool = key.field_bool;
            }
        }
        return this._uuid;
    }

    public assign(
        key: Protocol.StructC,
        overwrite: boolean
    ) {
        if (this._assigned === undefined || overwrite) {
            this._assigned = key;
        } else {            
            if (key.field_str !== undefined) {
                    this._assigned.field_str = key.field_str;
            }
            if (key.field_u8 !== undefined) {
                    this._assigned.field_u8 = key.field_u8;
            }
            if (key.field_u16 !== undefined) {
                    this._assigned.field_u16 = key.field_u16;
            }
            if (key.field_u32 !== undefined) {
                    this._assigned.field_u32 = key.field_u32;
            }
            if (key.field_u64 !== undefined) {
                    this._assigned.field_u64 = key.field_u64;
            }
            if (key.field_i8 !== undefined) {
                    this._assigned.field_i8 = key.field_i8;
            }
            if (key.field_i16 !== undefined) {
                    this._assigned.field_i16 = key.field_i16;
            }
            if (key.field_i32 !== undefined) {
                    this._assigned.field_i32 = key.field_i32;
            }
            if (key.field_i64 !== undefined) {
                    this._assigned.field_i64 = key.field_i64;
            }
            if (key.field_f32 !== undefined) {
                    this._assigned.field_f32 = key.field_f32;
            }
            if (key.field_f64 !== undefined) {
                    this._assigned.field_f64 = key.field_f64;
            }
            if (key.field_bool !== undefined) {
                    this._assigned.field_bool = key.field_bool;
            }
        }
    }

    public assigned(): boolean {
        if (this.assign === undefined) {
            switch (this._strategy) {
                case ProducerIdentificationStrategy.Ignore:
                    return true;
                case ProducerIdentificationStrategy.Log:
                    this._logger.warn(`Consumer ${this._uuid} isn't assigned`);
                    return true;
                default:
                    return false;
            }
        } else {
            return true;
        }
    }

    public hasKey(): boolean {
        return this._key !== undefined;
    }

    public discredited() {
        this._discredited = true;
    }

    public isDiscredited(): boolean {
        return this._discredited;
    }
}
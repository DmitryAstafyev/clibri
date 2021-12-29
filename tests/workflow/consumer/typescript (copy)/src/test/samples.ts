import { Protocol } from "../consumer/index";

// tslint:disable-next-line: no-namespace
export namespace StructA {
	export function get(): Protocol.IStructA {
		return {
			field_bool: true,
			field_f32: 0.1,
			field_f64: 0.2,
			field_i8: -1,
			field_i16: -2,
			field_i32: -3,
			field_i64: -BigInt(4),
			field_u8: 1,
			field_u16: 2,
			field_u32: 3,
			field_u64: BigInt(4),
			field_str: "",
			field_str_empty: "",
		};
	}
}

// tslint:disable:no-namespace
// tslint:disable:no-shadowed-variable

import * as Primitives from "./protocol.primitives";
import { Convertor } from "./protocol.convertor";
import { validate, IPropScheme } from "./protocol.validator";
import { ESize } from "./protocol.sizes";

export { ESize } from "./protocol.sizes";
export { Primitives };
export { Convertor } from "./protocol.convertor";
export { validate, IPropScheme } from "./protocol.validator";
export { BufferReader, IAvailableMessage } from "./packing";
export { MessageHeader } from "./packing.header";

// injectable
type ESizeAlias = ESize;
const ESizeAlias = ESize;
type ConvertorAlias<T> = Convertor<T>;
const ConvertorAlias = Convertor;
type IPropSchemeAlias = IPropScheme;
const PrimitivesAlias = Primitives;
const validateAlias = validate;

export namespace Protocol {
	export const ESize = ESizeAlias;
	export type ESize = ESizeAlias;
	export const Convertor = ConvertorAlias;
	export type Convertor<T> = ConvertorAlias<T>;
	export type IPropScheme = IPropSchemeAlias;
	export const Primitives = PrimitivesAlias;
	export const validate = validateAlias;
}

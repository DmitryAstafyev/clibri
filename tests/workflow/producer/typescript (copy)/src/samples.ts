// tslint:disable: no-namespace

import * as Protocol from "./producer/implementation/protocol";

export namespace StructA {
	export function get(): Protocol.IStructA {
		return Protocol.StructA.defaults();
	}

	export function instance(): Protocol.StructA {
		return new Protocol.StructA(get());
	}
}

export namespace StructB {
	export function get(): Protocol.IStructB {
		return Protocol.StructB.defaults();
	}

	export function instance(): Protocol.StructB {
		return new Protocol.StructB(get());
	}
}

export namespace StructC {
	export function get(): Protocol.IStructC {
		return Protocol.StructC.defaults();
	}

	export function instance(): Protocol.StructC {
		return new Protocol.StructC(get());
	}
}

export namespace StructD {
	export function get(): Protocol.IStructD {
		return Protocol.StructB.defaults();
	}

	export function instance(): Protocol.StructD {
		return new Protocol.StructD(get());
	}
}

export namespace StructE {
	export function get(): Protocol.IStructE {
		return Protocol.StructE.defaults();
	}

	export function instance(): Protocol.StructE {
		return new Protocol.StructE(get());
	}
}


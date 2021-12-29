import * as Protocol from "./implementation/protocol";
import { Stat } from "../stat";
import { panic } from "../tools";

export class Context {
	private _tests: Map<string, Stat> = new Map();

	public connected(uuid: string) {
		this._tests.set(uuid, new Stat());
	}

	public getStat(uuid: string): Stat {
		const stat = this._tests.get(uuid);
		if (stat === undefined) {
			panic(`Fail to get Stat for ${uuid}`);
		}
		return stat as Stat;
	}
}

export const expectations = {
	BeaconA: 1,
	BeaconsBeaconA: 1,
	BeaconsBeaconB: 1,
	BeaconsSubBeaconA: 1,
	GroupAStructA: 1,
	GroupAStructB: 4,
	GroupBGroupCStructA: 4,
	GroupBGroupCStructB: 4,
	GroupBStructA: 3,
	StructA: 3,
	StructB: 6,
	StructC: 5,
	StructD: 6,
	StructE: 3,
	StructF: 4,
	StructJ: 2,
	TriggerBeacons: 0,
	FinishConsumerTestBroadcast: 1,
	Connected: 1,
	Disconnected: 1,
	StructEmpty: 0,
	StructEmptyA: 1,
	StructEmptyB: 1,
	Error: 0,
};

export enum Alias {
	BeaconA,
	BeaconsBeaconA,
	BeaconsBeaconB,
	BeaconsSubBeaconA,
	GroupAStructA,
	GroupAStructB,
	GroupBGroupCStructA,
	GroupBGroupCStructB,
	GroupBStructA,
	StructA,
	StructB,
	StructC,
	StructD,
	StructE,
	StructF,
	StructJ,
	TriggerBeacons,
	FinishConsumerTestBroadcast,
	Connected,
	Disconnected,
	StructEmpty,
	StructEmptyA,
	StructEmptyB,
	Error,
}

const all: Alias[] = [
	Alias.BeaconA,
	Alias.BeaconsBeaconA,
	Alias.BeaconsBeaconB,
	Alias.BeaconsSubBeaconA,
	Alias.GroupAStructA,
	Alias.GroupAStructB,
	Alias.GroupBGroupCStructA,
	Alias.GroupBGroupCStructB,
	Alias.GroupBStructA,
	Alias.StructA,
	Alias.StructB,
	Alias.StructC,
	Alias.StructD,
	Alias.StructE,
	Alias.StructF,
	Alias.StructJ,
	Alias.TriggerBeacons,
	Alias.FinishConsumerTestBroadcast,
	Alias.Connected,
	Alias.Disconnected,
	Alias.StructEmpty,
	Alias.StructEmptyA,
	Alias.StructEmptyB,
	Alias.Error,
];

export interface TestData {
	done: number;
	expectation: number;
}

export class Stat {
	private _tests: Map<Alias, TestData> = new Map();
	private _indexes: Map<Alias, number> = new Map();

	constructor() {
		all.forEach((alias) => {
			this._indexes.set(alias, 0);
		});
		this._tests.set(Alias.BeaconA, {
			done: 0,
			expectation: expectations.BeaconA,
		});
		this._tests.set(Alias.BeaconsBeaconA, {
			done: 0,
			expectation: expectations.BeaconsBeaconA,
		});
		this._tests.set(Alias.BeaconsBeaconB, {
			done: 0,
			expectation: expectations.BeaconsBeaconB,
		});
		this._tests.set(Alias.BeaconsSubBeaconA, {
			done: 0,
			expectation: expectations.BeaconsSubBeaconA,
		});
		this._tests.set(Alias.GroupAStructA, {
			done: 0,
			expectation: expectations.GroupAStructA,
		});
		this._tests.set(Alias.GroupAStructB, {
			done: 0,
			expectation: expectations.GroupAStructB,
		});
		this._tests.set(Alias.GroupBGroupCStructA, {
			done: 0,
			expectation: expectations.GroupBGroupCStructA,
		});
		this._tests.set(Alias.GroupBGroupCStructB, {
			done: 0,
			expectation: expectations.GroupBGroupCStructB,
		});
		this._tests.set(Alias.GroupBStructA, {
			done: 0,
			expectation: expectations.GroupBStructA,
		});
		this._tests.set(Alias.StructA, {
			done: 0,
			expectation: expectations.StructA,
		});
		this._tests.set(Alias.StructB, {
			done: 0,
			expectation: expectations.StructB,
		});
		this._tests.set(Alias.StructC, {
			done: 0,
			expectation: expectations.StructC,
		});
		this._tests.set(Alias.StructD, {
			done: 0,
			expectation: expectations.StructD,
		});
		this._tests.set(Alias.StructE, {
			done: 0,
			expectation: expectations.StructE,
		});
		this._tests.set(Alias.StructF, {
			done: 0,
			expectation: expectations.StructF,
		});
		this._tests.set(Alias.StructJ, {
			done: 0,
			expectation: expectations.StructJ,
		});
		this._tests.set(Alias.TriggerBeacons, {
			done: 0,
			expectation: expectations.TriggerBeacons,
		});
		this._tests.set(Alias.FinishConsumerTestBroadcast, {
			done: 0,
			expectation: expectations.FinishConsumerTestBroadcast,
		});
		this._tests.set(Alias.Connected, {
			done: 0,
			expectation: expectations.Connected,
		});
		this._tests.set(Alias.Disconnected, {
			done: 0,
			expectation: expectations.Disconnected,
		});
		this._tests.set(Alias.StructEmpty, {
			done: 0,
			expectation: expectations.StructEmpty,
		});
		this._tests.set(Alias.StructEmptyA, {
			done: 0,
			expectation: expectations.StructEmptyA,
		});
		this._tests.set(Alias.StructEmptyB, {
			done: 0,
			expectation: expectations.StructEmptyB,
		});
		this._tests.set(Alias.Error, {
			done: 0,
			expectation: expectations.Error,
		});
	}

	public index(alias: Alias): number {
		const index = this._indexes.get(alias) as number;
		this._indexes.set(alias, index + 1);
		return index;
	}

	public case(alias: Alias) {
		console.log(`>>>>>>>>>>>>>>> request ${alias} has been gotten`);
		const data = this._tests.get(alias) as TestData;
		data.done += 1;
		this._tests.set(alias, data);
	}
}

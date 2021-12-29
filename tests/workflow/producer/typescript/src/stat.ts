export const expectations = {
	BeaconA: 1,
	BeaconsBeaconA: 1,
	BeaconsBeaconB: 1,
	BeaconsSubBeaconA: 1,
	GroupAStructA: 4,
	GroupAStructB: 4,
	GroupBGroupCStructA: 3,
	GroupBGroupCStructB: 5,
	GroupBStructA: 4,
	StructA: 5,
	StructB: 2,
	StructC: 6,
	StructD: 2,
	StructE: 3,
	StructF: 2,
	StructJ: 2,
	TriggerBeacons: 1,
	FinishConsumerTestBroadcast: 1,
	Connected: 1,
	Disconnected: 1,
	StructEmpty: 2,
	StructEmptyA: 1,
	StructEmptyB: 1,
	Error: 0,
};

export enum Alias {
	BeaconA = "BeaconA",
	BeaconsBeaconA = "BeaconsBeaconA",
	BeaconsBeaconB = "BeaconsBeaconB",
	BeaconsSubBeaconA = "BeaconsSubBeaconA",
	GroupAStructA = "GroupAStructA",
	GroupAStructB = "GroupAStructB",
	GroupBGroupCStructA = "GroupBGroupCStructA",
	GroupBGroupCStructB = "GroupBGroupCStructB",
	GroupBStructA = "GroupBStructA",
	StructA = "StructA",
	StructB = "StructB",
	StructC = "StructC",
	StructD = "StructD",
	StructF = "StructF",
	StructJ = "StructJ",
	StructE = "StructE",
	TriggerBeacons = "TriggerBeacons",
	FinishConsumerTestBroadcast = "FinishConsumerTestBroadcast",
	Connected = "Connected",
	Disconnected = "Disconnected",
	StructEmpty = "StructEmpty",
	StructEmptyA = "StructEmptyA",
	StructEmptyB = "StructEmptyB",
	Error = "Error",
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
	static MIN_BEACONS_COUNT: number = 4;
	private _tests: Map<Alias, TestData> = new Map();
	private _indexes: Map<Alias, number> = new Map();

	constructor(noExpectations?: boolean) {
		all.forEach((alias) => {
			this._indexes.set(alias, 0);
		});
		this._tests.set(Alias.BeaconA, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.BeaconA,
		});
		this._tests.set(Alias.BeaconsBeaconA, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.BeaconsBeaconA,
		});
		this._tests.set(Alias.BeaconsBeaconB, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.BeaconsBeaconB,
		});
		this._tests.set(Alias.BeaconsSubBeaconA, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.BeaconsSubBeaconA,
		});
		this._tests.set(Alias.GroupAStructA, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.GroupAStructA,
		});
		this._tests.set(Alias.GroupAStructB, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.GroupAStructB,
		});
		this._tests.set(Alias.GroupBGroupCStructA, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.GroupBGroupCStructA,
		});
		this._tests.set(Alias.GroupBGroupCStructB, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.GroupBGroupCStructB,
		});
		this._tests.set(Alias.GroupBStructA, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.GroupBStructA,
		});
		this._tests.set(Alias.StructA, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructA,
		});
		this._tests.set(Alias.StructB, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructB,
		});
		this._tests.set(Alias.StructC, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructC,
		});
		this._tests.set(Alias.StructD, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructD,
		});
		this._tests.set(Alias.StructE, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructE,
		});
		this._tests.set(Alias.StructF, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructF,
		});
		this._tests.set(Alias.StructJ, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructJ,
		});
		this._tests.set(Alias.TriggerBeacons, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.TriggerBeacons,
		});
		this._tests.set(Alias.FinishConsumerTestBroadcast, {
			done: 0,
			expectation: noExpectations
				? 0
				: expectations.FinishConsumerTestBroadcast,
		});
		this._tests.set(Alias.Connected, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.Connected,
		});
		this._tests.set(Alias.Disconnected, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.Disconnected,
		});
		this._tests.set(Alias.StructEmpty, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructEmpty,
		});
		this._tests.set(Alias.StructEmptyA, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructEmptyA,
		});
		this._tests.set(Alias.StructEmptyB, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.StructEmptyB,
		});
		this._tests.set(Alias.Error, {
			done: 0,
			expectation: noExpectations ? 0 : expectations.Error,
		});
	}

	public index(alias: Alias): number {
		const index = this._indexes.get(alias) as number;
		this._indexes.set(alias, index + 1);
		return index;
	}

	public case(alias: Alias) {
		const data = this._tests.get(alias) as TestData;
		data.done += 1;
		this._tests.set(alias, data);
	}

	public getBeaconsCount(): number {
		return (
			(this._tests.get(Alias.BeaconA) as TestData).done +
			(this._tests.get(Alias.BeaconsBeaconA) as TestData).done +
			(this._tests.get(Alias.BeaconsBeaconB) as TestData).done +
			(this._tests.get(Alias.BeaconsSubBeaconA) as TestData).done
		);
	}

	public merge(stat: Stat) {
		this._tests.forEach((data: TestData, alias: Alias) => {
			const income = stat.getCase(alias);
			data.done += income.done;
			data.expectation += income.expectation;
		});
	}

	public print() {
		const LEN: number = 70;
		this._tests.forEach((data: TestData, alias: Alias) => {
			const title = `${alias}`;
			const correct = data.done === data.expectation;
			console.log(
				`${correct ? "☑" : "☐"} ${title}${".".repeat(
					LEN - title.length
				)}: ${data.done} / ${data.expectation}`
			);
		});
	}

	public getCase(alias: Alias): TestData {
		return this._tests.get(alias) as TestData;
	}

	public getErrors(): string[] {
		const errors: string[] = [];
		this._tests.forEach((data: TestData, alias: Alias) => {
			if (data.done !== data.expectation) {
				errors.push(
					`Test for usecase "${alias}" is failed; ${data.done} / ${data.expectation}`
				);
			}
		});
		return errors;
	}
}

import * as Protocol from "./implementation/protocol";
import { Stat } from "../stat";
import { panic } from "../tools";
import { Producer } from "./index";

export class Context {
	private _tests: Map<string, Stat> = new Map();
	private _summary: Stat = new Stat(true);
	private _ignore: string | undefined;

	public ignore(uuid: string) {
		this._ignore = uuid;
	}

	public connected(uuid: string) {
		this._tests.set(uuid, new Stat());
	}

	public disconnected(uuid: string) {
		this._ignore !== uuid && this._summary.merge(this.getStat(uuid));
		this._tests.delete(uuid);
		if (this._tests.size === 0) {
			this._summary.print();
			const errors = this._summary.getErrors();
			errors.forEach((err: string) => {
				console.error(err);
			});
			if (errors.length > 0) {
				panic("Test results are negative");
			}
		}
	}

	public getStat(uuid: string): Stat {
		const stat = this._tests.get(uuid);
		if (stat === undefined) {
			panic(`Fail to get Stat for ${uuid}`);
		}
		return stat as Stat;
	}

	public checkBeacons(uuid: string, producer: Producer) {
		const stat = this._tests.get(uuid);
		if (stat === undefined) {
			panic(`Fail to get Stat for ${uuid}`);
			return;
		}
		if (stat.getBeaconsCount() >= Stat.MIN_BEACONS_COUNT) {
			const event = Protocol.FinishConsumerTest.defaults();
			event.uuid = uuid;
			producer.events.finishConsumerTest.emit(event);
		}
	}
}

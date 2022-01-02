import { Consumer } from "./index";
import { Identification } from "./identification";

export class Filter {
	private readonly _identifications: Map<string, Identification> = new Map();

	constructor(consumers: Map<string, Consumer>) {
		consumers.forEach((consumer: Consumer, uuid: string) => {
			this._identifications.set(uuid, consumer.getIdentification());
		});
	}

	public filter(cb: (identification: Identification) => boolean): string[] {
		return Array.from(this._identifications.values())
			.filter((identification: Identification) => {
				return cb(identification);
			})
			.map((identification: Identification) => {
				return identification.uuid();
			});
	}

	public exclude(uuids: string[]): string[] {
		return Array.from(this._identifications.values())
			.filter((identification: Identification) => {
				return uuids.indexOf(identification.uuid()) === -1;
			})
			.map((identification: Identification) => {
				return identification.uuid();
			});
	}

	public except(uuid: string): string[] {
		return Array.from(this._identifications.values())
			.filter((identification: Identification) => {
				return identification.uuid() !== uuid;
			})
			.map((identification: Identification) => {
				return identification.uuid();
			});
	}

	public all(): string[] {
		return Array.from(this._identifications.values()).map(
			(identification: Identification) => {
				return identification.uuid();
			}
		);
	}
}

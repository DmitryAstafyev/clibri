import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { emit } from "../../events/triggerbeaconsemitter";

export class Output {
    static REQUIRED = [    
        Protocol.TriggerBeacons,
    ];
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    public broadcast(uuids: string[]): {        
        TriggerBeacons(msg: Protocol.TriggerBeacons): Output;
    } {
        const self = this;
        return {            
            TriggerBeacons(msg: Protocol.TriggerBeacons): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getSignature() ===
                            Protocol.TriggerBeacons.getSignature()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.TriggerBeacons already has been defined.`
                    );
                }
                self._broadcasts.push([uuids, msg]);
                return self;
            },
        };
    }

	public error(): Error | undefined {
		let error: Error | undefined;
		Output.REQUIRED.forEach((ref) => {
			if (error !== undefined) {
				return;
			}
			if (
				this._broadcasts.find((msg) => {
					return msg[1].getSignature() === ref.getSignature();
				}) === undefined
			) {
				error = new Error(
					`Broadcast ${ref.getSignature()} is required, but hasn't been found`
				);
			}
		});
		return error;
	}

    public broadcasts(): Array<[string[], Protocol.Convertor<any>]> {
        return this._broadcasts;
    }
}

export function handler(
    event: Protocol.TriggerBeaconsEmitter,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return emit(event, filter, context, producer).then((output) => {
        const error: Error | undefined = output.error();
        if (error instanceof Error) {
            return Promise.reject(error);
        }
        return broadcastAll(producer, output.broadcasts());
    });
}
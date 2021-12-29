import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { emit } from "../../events/events.eventa";

export class Output {
    static REQUIRED = [    
        Protocol.StructA,
        Protocol.StructB,
    ];
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    public broadcast(uuids: string[]): {        
        StructA(msg: Protocol.StructA): Output;
        StructB(msg: Protocol.StructB): Output;
    } {
        const self = this;
        return {            
            StructA(msg: Protocol.StructA): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.StructA.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.StructA already has been defined.`
                    );
                }
                self._broadcasts.push([uuids, msg]);
                return self;
            },
            StructB(msg: Protocol.StructB): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.StructB.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.StructB already has been defined.`
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
					return msg[1].getId() === ref.getId();
				}) === undefined
			) {
				error = new Error(
					`Broadcast ${ref.getSignature()}/${ref.getId()} is required, but hasn't been found`
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
    event: Protocol.Events.EventA,
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
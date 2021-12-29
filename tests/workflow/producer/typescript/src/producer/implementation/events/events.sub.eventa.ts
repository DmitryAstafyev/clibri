import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { emit } from "../../events/events.sub.eventa";

export class Output {
    static REQUIRED = [    
        Protocol.GroupB.GroupC.StructA,
        Protocol.GroupB.GroupC.StructB,
    ];
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    public broadcast(uuids: string[]): {        
        GroupBGroupCStructA(msg: Protocol.GroupB.GroupC.StructA): Output;
        GroupBGroupCStructB(msg: Protocol.GroupB.GroupC.StructB): Output;
    } {
        const self = this;
        return {            
            GroupBGroupCStructA(msg: Protocol.GroupB.GroupC.StructA): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.GroupB.GroupC.StructA.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.GroupB.GroupC.StructA already has been defined.`
                    );
                }
                self._broadcasts.push([uuids, msg]);
                return self;
            },
            GroupBGroupCStructB(msg: Protocol.GroupB.GroupC.StructB): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.GroupB.GroupC.StructB.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.GroupB.GroupC.StructB already has been defined.`
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
    event: Protocol.Events.Sub.EventA,
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
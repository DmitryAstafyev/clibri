import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { emit } from "../../events/events.eventb";

export class Output {
    static REQUIRED = [    
        Protocol.GroupA.StructA,
        Protocol.GroupA.StructB,
        Protocol.GroupB.StructA,
    ];
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    public broadcast(uuids: string[]): {        
        GroupAStructA(msg: Protocol.GroupA.StructA): Output;
        GroupAStructB(msg: Protocol.GroupA.StructB): Output;
        GroupBStructA(msg: Protocol.GroupB.StructA): Output;
    } {
        const self = this;
        return {            
            GroupAStructA(msg: Protocol.GroupA.StructA): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getSignature() ===
                            Protocol.GroupA.StructA.getSignature()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.GroupA.StructA already has been defined.`
                    );
                }
                self._broadcasts.push([uuids, msg]);
                return self;
            },
            GroupAStructB(msg: Protocol.GroupA.StructB): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getSignature() ===
                            Protocol.GroupA.StructB.getSignature()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.GroupA.StructB already has been defined.`
                    );
                }
                self._broadcasts.push([uuids, msg]);
                return self;
            },
            GroupBStructA(msg: Protocol.GroupB.StructA): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getSignature() ===
                            Protocol.GroupB.StructA.getSignature()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.GroupB.StructA already has been defined.`
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
    event: Protocol.Events.EventB,
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
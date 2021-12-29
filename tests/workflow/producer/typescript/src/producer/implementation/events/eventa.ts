import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { emit } from "../../events/eventa";

export class Output {
    static REQUIRED = [    
        Protocol.StructB,
        Protocol.StructC,
    ];
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    public broadcast(uuids: string[]): {        
        StructB(msg: Protocol.StructB): Output;
        StructC(msg: Protocol.StructC): Output;
    } {
        const self = this;
        return {            
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
            StructC(msg: Protocol.StructC): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.StructC.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.StructC already has been defined.`
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
    event: Protocol.EventA,
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
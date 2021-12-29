import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { emit } from "../../events/finishconsumertest";

export class Output {
    static REQUIRED = [    
        Protocol.FinishConsumerTestBroadcast,
    ];
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    public broadcast(uuids: string[]): {        
        FinishConsumerTestBroadcast(msg: Protocol.FinishConsumerTestBroadcast): Output;
    } {
        const self = this;
        return {            
            FinishConsumerTestBroadcast(msg: Protocol.FinishConsumerTestBroadcast): Output {
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getSignature() ===
                            Protocol.FinishConsumerTestBroadcast.getSignature()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.FinishConsumerTestBroadcast already has been defined.`
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
    event: Protocol.FinishConsumerTest,
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
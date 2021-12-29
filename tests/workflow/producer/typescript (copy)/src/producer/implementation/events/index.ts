import * as Protocol from "../../implementation/protocol";
import { Producer } from "../index";

export { Identification, Filter } from "../consumer";
export { Context } from "../../context";
export { Producer } from "../index";
export { ProducerError, ProducerErrorType } from "./error";
export { Protocol };
export { handler as eventAHandler } from "./eventa";
export { handler as eventBHandler } from "./eventb";
export { handler as eventsEventAHandler } from "./events.eventa";
export { handler as eventsEventBHandler } from "./events.eventb";
export { handler as eventsSubEventAHandler } from "./events.sub.eventa";
export { handler as triggerBeaconsEmitterHandler } from "./triggerbeaconsemitter";
export { handler as finishConsumerTestHandler } from "./finishconsumertest";
export { handler as disconnectedHandler } from "./disconnected";
export { handler as connectedHandler } from "./connected";
export { handler as errorHandler } from "./error";
export { handler as readyHandler } from "./ready";
export { handler as shutdownHandler } from "./shutdown";

export function broadcastAll(
    producer: Producer,
    broadcasts: Array<[string[], Protocol.Convertor<any>]>
): Promise<void> {
    if (broadcasts.length === 0) {
        return Promise.resolve();
    }
    return new Promise((resolve, reject) => {
        let error: Error | undefined;
        Promise.all(
            broadcasts.map((broadcast) => {
                return producer
                    .broadcast(broadcast[0], broadcast[1].pack(0, undefined))
                    .catch((err: Error) => {
                        error = err;
                    });
            })
        )
            .then(() => {
                if (error !== undefined) {
                    reject(error);
                } else {
                    resolve();
                }
            })
            .catch(reject);
    });
}
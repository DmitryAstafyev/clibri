import * as Protocol from "implementation/protocol";
import { Producer } from "../index";

export { Identification, Filter } from "../consumer";
export { Context } from "../../context";
export { Producer } from "../index";
export { ProducerError, ProducerErrorType } from "./error";

export { handler as disconnectedHandler } from "./disconnected";
export { handler as connectedHandler } from "./connected";
export { handler as errorHandler } from "./error";
export { handler as readyHandler } from "./ready";
export { handler as shutdownHandler } from "./shutdown";
export { handler as servereventsUseralertHandler } from "./serverevents.useralert";
export { handler as servereventsUserkickoffHandler } from "./serverevents.userkickoff";

export function broadcastAll<C>(
	producer: Producer<C>,
	broadcasts: Array<[string[], Protocol.Convertor<any>]>
): Promise<void> {
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

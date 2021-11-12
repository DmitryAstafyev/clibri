import * as Protocol from "../../implementation/protocol";
import { Producer } from "../index";

export { handler as userLoginRequestHandler } from "./userlogin.request";
export { handler as usersRequestHandler } from "./users.request";
export { handler as messageRequestHandler } from "./message.request";
export { handler as messagesRequestHandler } from "./messages.request";

export { Identification, Filter } from "../consumer";
export { Context } from "../../context";
export { Producer } from "../index";
export { Protocol };

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
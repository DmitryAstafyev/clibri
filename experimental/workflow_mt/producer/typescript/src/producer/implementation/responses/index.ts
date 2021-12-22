import * as Protocol from "../../implementation/protocol";
import { Producer } from "../index";

export { handler as structAHandler } from "./structa";
export { handler as structCHandler } from "./structc";
export { handler as structDHandler } from "./structd";
export { handler as structFHandler } from "./structf";
export { handler as groupAStructAHandler } from "./groupa.structa";
export { handler as groupAStructBHandler } from "./groupa.structb";
export { handler as groupBGroupCStructAHandler } from "./groupb.groupc.structa";
export { handler as groupBStructAHandler } from "./groupb.structa";
export { handler as groupBGroupCStructBHandler } from "./groupb.groupc.structb";

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
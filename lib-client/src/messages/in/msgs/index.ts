import { ClientDisconnect, IClientDisconnect } from './msg.client.disconnect';
import { Message } from './message';
import { IMessage } from '../message.holder';

export * from './message';
export * from './msg.client.disconnect';

export type TMessage =  typeof ClientDisconnect |
                        typeof Message;

const refs: { [key: number]: TMessage } = {
    [ClientDisconnect.id]: ClientDisconnect,
};

export function getMsgClass(msg: IMessage): TMessage | Error {
    if (refs[msg.id] === undefined) {
        return new Error(`Fail to find implementation for message ID="${msg.id}"`);
    }
    return refs[msg.id];
}
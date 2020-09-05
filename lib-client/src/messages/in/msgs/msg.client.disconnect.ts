import { IMessage } from '../message.holder';
import { Message } from './message';

export interface IClientDisconnect {
    uuid: string;
}

export class ClientDisconnect extends Message<IClientDisconnect> {

    static id: number = 4;
    public readonly id: number = 4;

    constructor(msg: IMessage) {
        super(msg);
    }

    public validate(): Error | undefined {
        const err: Error | undefined = this.getErr();
        if (err instanceof Error) {
            return err;
        }
        const message: IClientDisconnect = this.getMsg().struct as IClientDisconnect;
        if (typeof message.uuid !== 'string' || message.uuid.trim() === '') {
            return new Error(`Expecting field "uuid" would be not empty string.`);
        }
        return undefined;
    }

}
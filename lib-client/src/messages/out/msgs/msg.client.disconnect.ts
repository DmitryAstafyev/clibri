import { Message } from './message';

export interface IClientDisconnect {
    uuid: string;
}

export class ClientDisconnect extends Message<IClientDisconnect> {

    public readonly id: number = 4;

}
import { Message } from './message';

export interface IPing {
    guid: string;
}

export class Ping extends Message<IPing> {

    public readonly id: number = 1;

}
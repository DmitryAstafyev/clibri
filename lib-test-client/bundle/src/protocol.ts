// tslint:disable:max-classes-per-file

import { Protocol, In, Out } from '../../../lib-client/src/index';

export interface PingInMsgBody {
    uuid: string;
}

export class PingIn extends In.Message<PingInMsgBody> {

    public static readonly id: number = 1;

    public readonly id: number = PingIn.id;

    public validate(): Error | undefined {
        if (typeof this.getMsg().struct.uuid !== 'string') {
            return new Error(`Expecting "uuid" be a string`);
        }
        if (this.getMsg().struct.uuid.trim() === '') {
            return new Error(`Expecting "uuid" would not be empty`);
        }
        return undefined;
    }
}

export class PingOut extends Out.Message<PingInMsgBody> {

    public static readonly id: number = 1;

    public readonly id: number = PingOut.id;

}

export type IncomeMessages = typeof PingIn;

export class ProtocolImpl extends Protocol<IncomeMessages> {

    public getMsgRefs(): { [key: number]: IncomeMessages } {
        return {
            [PingIn.id]: PingIn,
        }
    }

}

import * as Protocol from '../protocol/protocol';

import { Consumer } from '../consumer';

export abstract class UserJoin extends Protocol.UserJoin.Request {

    constructor(request: Protocol.UserJoin.IRequest) {
        super(request);
    }

    public send(): Error | undefined {
        const consumer: Consumer | Error = Consumer.get();
        if (consumer instanceof Error) {
            return consumer;
        }
        
    }

    public abstract accept(response: Protocol.UserJoin.Accepted);
    public abstract deny(response: Protocol.UserJoin.Denied);
    public abstract error(response: Protocol.UserJoin.Err);

}
/*
export class UserJoin<UCX> extends IUserJoin<UCX> {

    constructor(client: Client) {

    }

    public send(request: Protocol.UserJoin.Request): Promise<
        Protocol.UserJoin.Accepted |
        Protocol.UserJoin.Denied |
        Protocol.UserJoin.Err> {
        return new Promise((resolve, reject) => {

        });
    }
    public accept(response: Protocol.UserJoin.Accepted, ucx: UCX) {
        throw new Error(`Response "accept" for "UserJoin" doesn't have implementation`);
    }
    public deny(response: Protocol.UserJoin.Denied, ucx: UCX) {
        throw new Error(`Response "accept" for "UserJoin" doesn't have implementation`);
    }
    public error(response: Protocol.UserJoin.Err, ucx: UCX) {
        throw new Error(`Response "accept" for "UserJoin" doesn't have implementation`);
    }
}
*/
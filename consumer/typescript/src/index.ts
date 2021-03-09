import { UserJoin } from './declarations/observer.UserJoin';
import { UserLogout } from './declarations/observer.UserLogout';
import { UserSignIn } from './declarations/observer.UserSignIn';
import { UserConnected } from './declarations/observer.UserConnected';
import { UserDisconnected } from './declarations/observer.UserDisconnected';

/*

class DummyClient extends Client {
    public send(buffer: Buffer): Error | undefined {
        return undefined;
    }
    public connect(): Promise<void> {
        return Promise.resolve();
    }
    public disconnect(): Promise<void> {
        return Promise.resolve();
    }
    public destroy(): Promise<void> {
        return Promise.resolve();
    }
}

const consumer: Consumer = new Consumer(new DummyClient());

*/
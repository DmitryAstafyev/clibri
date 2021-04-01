import { Client, Subject, IClientSubjects } from 'fiber';
import { Consumer, Protocol, UserLogin } from './consumer/index';
import { Connection } from 'fiber-websocket';
import { SpinnerComponent } from './components/spinner/component';
import { LoginComponent } from './components/login/component';
import { UsersComponent } from './components/users/component';

class Application {

    private _components: {
        spinner: SpinnerComponent;
        login: LoginComponent;
        users: UsersComponent;
    } | undefined;
    private _connection: Connection;
    private _consumer: Consumer;

    constructor() {
        this._init = this._init.bind(this);
        [document, window].forEach(v => v.addEventListener('load', this._init));
    }

    private _init() {
        [document, window].forEach(v => v.removeEventListener('load', this._init));
        this._connection = new Connection(`ws://127.0.0.1:8080`);
        this._consumer = new Consumer(this._connection, {
            id: BigInt(123),
            uuid: 'Some UUID',
            location: 'London'
        });
        this._consumer.connected.subscribe(this._onConnected.bind(this));
        this._consumer.ready.subscribe(this._onReady.bind(this));
        this._components = {
            spinner: new SpinnerComponent(),
            login: new LoginComponent(),
            users: new UsersComponent(this._consumer),
        };
        this._components.login.input.subscribe(this._onLoginInput.bind(this));
        this._components.spinner.mount();
    }

    private _onConnected() {
        console.log(`Consumer is connected!`);
    }

    private _onReady() {
        this._components.spinner.umount();
        this._components.login.mount();
    }

    private _onLoginInput(username: string) {
        this._components.login.umount();
        const login: UserLogin = new UserLogin({ username: username });
        login.accept((response: Protocol.UserLogin.Accepted) => {
            this._components.users.mount();
            // console.log(response);
        }).deny((response: Protocol.UserLogin.Denied) => {
            // console.log(response);
        }).err((response: Protocol.UserLogin.Err) => {
            // console.log(response);
        });
        login.send().catch((err: Error) => {
            console.error(err);
        });
    }


}

const app: Application = new Application();



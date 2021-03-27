import { Client, Subject, IClientSubjects } from 'fiber';
import { Consumer, Protocol } from './consumer/index';
import { Connection } from 'fiber-websocket';
import { SpinnerComponent } from './components/spinner/component';
import { LoginComponent } from './components/login/component';

class Application {

    private _components: {
        spinner: SpinnerComponent;
        login: LoginComponent;
    } = {
        spinner: new SpinnerComponent(),
        login: new LoginComponent(),
    };
    private _connection: Connection;
    private _consumer: Consumer;

    constructor() {
        this._init = this._init.bind(this);
        document.addEventListener('load', this._init);
        window.addEventListener('load', this._init);
    }

    private _init() {
        document.removeEventListener('load', this._init);
        window.removeEventListener('load', this._init);
        this._components.spinner.mount();
        this._connection = new Connection(`ws://127.0.0.1:8080`);
        this._consumer = new Consumer(this._connection, {
            id: BigInt(123),
            uuid: 'Some UUID',
            location: 'London'
        });
        this._consumer.connected.subscribe(() => {
            console.log(`Consumer is connected!`);
        });
        this._consumer.ready.subscribe(() => {
            console.log(`Consumer is ready!`);
            this._components.spinner.umount();
            this._components.login.mount();
        });
    }
}

const app: Application = new Application();



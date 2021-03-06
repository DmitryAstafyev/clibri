// tslint:disable: max-classes-per-file

abstract class Client {
}

class Consumer {

    private readonly _client: Client;

    constructor(client: Client) {
        this._client = client;
    }

}

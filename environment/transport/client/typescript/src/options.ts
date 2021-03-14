import { Logger, DefaultLogger } from 'fiber';

export class IConnectionOptions {
    autoconnect?: boolean;
    reconnect?: number;
    logger?: Logger;
}

export class ConnectionOptions {

    public autoconnect: boolean = true;
    public reconnect: number = 2000;
    public logger: Logger;
    
    constructor(alias: string, options: IConnectionOptions = {}) {
        if (typeof options.autoconnect === 'boolean') {
            this.autoconnect = options.autoconnect;
        }
        if (typeof options.reconnect === 'number' && !isNaN(options.reconnect) && isFinite(options.reconnect)) {
            this.reconnect = options.reconnect;
        }
        if (options.logger !== undefined) {
            this.logger = options.logger;
        } else {
            this.logger = new DefaultLogger(alias);
        }
    }

}
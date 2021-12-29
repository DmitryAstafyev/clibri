import { Logger, DefaultLogger } from 'clibri';

export interface IOptions {
    logger?: Logger;
    autoconnect?: boolean;
    reconnect?: number;
    global?: boolean;
}

export class Options {

    public autoconnect: boolean = true;
    public global: boolean = true;
    public reconnect: number = 2000;
    public logger: Logger;

    constructor(alias: string, options: IOptions = {}) {
        if (options.logger !== undefined) {
            this.logger = options.logger;
        } else {
            this.logger = new DefaultLogger(alias);
        }
        options.autoconnect !== undefined && (this.autoconnect = options.autoconnect);
        options.reconnect !== undefined && (this.reconnect = options.reconnect);
        options.global !== undefined && (this.global = options.global);
    }

}
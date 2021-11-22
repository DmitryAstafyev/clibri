import { Logger, DefaultLogger } from 'clibri';

export interface IOptions {
    logger?: Logger;
}

export class Options {

    public autoconnect: boolean = true;
    public reconnect: number = 2000;
    public logger: Logger;

    constructor(alias: string, options: IOptions = {}) {
        if (options.logger !== undefined) {
            this.logger = options.logger;
        } else {
            this.logger = new DefaultLogger(alias);
        }
    }

}
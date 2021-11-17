import { ProducerIdentificationStrategy, Logger } from "clibri";
import * as Protocol from "../protocol";

export class Identification {
    private readonly _uuid: string;
    private readonly _strategy: ProducerIdentificationStrategy;
    private _discredited: boolean = false;
    private _key: Protocol.Identification.SelfKey | undefined;
    private _assigned: Protocol.Identification.AssignedKey | undefined;
    private _logger: Logger;

    constructor(
        uuid: string,
        strategy: ProducerIdentificationStrategy,
        logger: Logger
    ) {
        this._uuid = uuid;
        this._strategy = strategy;
        this._logger = logger.clone(`[${uuid}][Identification]`);
    }

    public uuid(): string {
        return this._uuid;
    }

    public key(
        key: Protocol.Identification.SelfKey,
        overwrite: boolean
    ): string {
        if (this._key === undefined || overwrite) {
            this._key = key;
        } else {            
            if (key.uuid !== undefined) {
                this._key.uuid = key.uuid;
            }
            if (key.id !== undefined) {
                this._key.id = key.id;
            }
            if (key.location !== undefined) {
                this._key.location = key.location;
            }
        }
        return this._uuid;
    }

    public assign(
        key: Protocol.Identification.AssignedKey,
        overwrite: boolean
    ) {
        if (this._assigned === undefined || overwrite) {
            this._assigned = key;
        } else {            
            if (key.uuid !== undefined) {
                    this._assigned.uuid = key.uuid;
            }
            if (key.auth !== undefined) {
                    this._assigned.auth = key.auth;
            }
        }
    }

    public assigned(): boolean {
        if (this.assign === undefined) {
            switch (this._strategy) {
                case ProducerIdentificationStrategy.Ignore:
                    return true;
                case ProducerIdentificationStrategy.Log:
                    this._logger.warn(`Consumer ${this._uuid} isn't assigned`);
                    return true;
                default:
                    return false;
            }
        } else {
            return true;
        }
    }

    public hasKey(): boolean {
        return this._key !== undefined;
    }

    public discredited() {
        this._discredited = true;
    }

    public isDiscredited(): boolean {
        return this._discredited;
    }
}
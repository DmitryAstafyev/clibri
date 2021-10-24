// tslint:disable: max-classes-per-file

import globals from "../tools/tools.globals";

export enum ELogLevel {
	verb = 5,
	info = 4,
	debug = 3,
	warn = 2,
	err = 1,
}

const LogLevelLabels = {
	1: "Error",
	2: "Warning",
	3: "Debug",
	4: "Info",
	5: "Verb",
};

export abstract class Logger {
	public static readonly GLOBAL_ALIAS: string =
		"___FIBER_GLOBAL_LOGS_ALIAS___";
	public static setGlobalLevel(level: ELogLevel) {
		const global: any = globals();
		global[Logger.GLOBAL_ALIAS].level = level;
	}

	private readonly _alias: string | undefined;

	constructor(alias?: string) {
		this._alias = typeof alias !== "string" ? undefined : alias;
	}

	public getSignature(level: number): string {
		return `[${this._setLastTimeStamp()}ms\t][${
			(LogLevelLabels as any)[level]
		}]${this._alias === undefined ? "" : `[${this._alias}]`}`;
	}

	public getLevel(): ELogLevel {
		const global: any = globals();
		return global[Logger.GLOBAL_ALIAS].level;
	}

	public abstract warn(msg: string): string;
	public abstract debug(msg: string): string;
	public abstract verb(msg: string): string;
	public abstract err(msg: string): string;
	public abstract info(msg: string): string;
	public abstract clone(alias: string | undefined): Logger;

	private _setLastTimeStamp(): number {
		const global: any = globals();
		const prev = global[Logger.GLOBAL_ALIAS].ts;
		global[Logger.GLOBAL_ALIAS].ts = Date.now();
		return global[Logger.GLOBAL_ALIAS].ts - prev;
	}
}

export class DefaultLogger extends Logger {
	public warn(msg: string): string {
		return this._log(msg, ELogLevel.warn);
	}

	public debug(msg: string): string {
		return this._log(msg, ELogLevel.debug);
	}

	public verb(msg: string): string {
		return this._log(msg, ELogLevel.verb);
	}

	public err(msg: string): string {
		return this._log(msg, ELogLevel.err);
	}

	public info(msg: string): string {
		return this._log(msg, ELogLevel.info);
	}

	public clone(alias: string | undefined): Logger {
		return new DefaultLogger(alias);
	}

	private _log(msg: string, level: number): string {
		if (level > this.getLevel()) {
			return msg;
		}
		const message: string = `${this.getSignature(level)}: ${msg}`;
		console.log(message);
		return msg;
	}
}

function setDefaults() {
	const global: any = globals();
	if (global[Logger.GLOBAL_ALIAS] === undefined) {
		global[Logger.GLOBAL_ALIAS] = { ts: Date.now(), level: ELogLevel.warn };
	}
}

setDefaults();

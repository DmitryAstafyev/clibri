// tslint:disable: max-classes-per-file
import {
	Consumer,
	BeaconBeaconsShutdownServer,
	Protocol,
} from "./consumer/index";
import { Subscription, Logger, ELogLevel } from "clibri";
import { Stat, Alias } from "./stat";
import { Client } from "./client";
import { Connection } from "clibri-websocket-client-node";
import { panic } from "./tools";

Logger.setGlobalLevel(ELogLevel.warn);

class Options {
	public connections: number = 100;
	constructor() {
		process.argv.forEach((arg: string) => {
			if (arg.toLowerCase().includes("--connections")) {
				const connections = parseInt(arg.split("=")[1], 10);
				if (isFinite(connections) && !isNaN(connections)) {
					this.connections = connections;
				}
			}
		});
	}
}

class Application {
	private _subscriptions: { [key: string]: Subscription } = {};
	private _stat: Stat = new Stat(true);
	private _started = Date.now();

	constructor() {
		const options = new Options();
		let done: number = 0;
		for (let i = 0; i < options.connections; i += 1) {
			const client = new Client(`ws://127.0.0.1:8080`);
			client.done.subscribe((stat: Stat) => {
				done += 1;
				this._stat.merge(stat);
				if (done === options.connections) {
					this._finish();
				}
			});
		}
	}

	private _finish() {
		const consumer = new Consumer(new Connection(`ws://127.0.0.1:8080`), {
			field_bool: true,
			field_f32: 0.1,
			field_f64: 0.2,
			field_i8: -1,
			field_i16: -2,
			field_i32: -3,
			field_i64: -BigInt(4),
			field_u8: 1,
			field_u16: 2,
			field_u32: 3,
			field_u64: BigInt(4),
			field_str: "",
			field_str_empty: "",
		});
		consumer.ready.subscribe(() => {
			const shutdown = new BeaconBeaconsShutdownServer(
				Protocol.Beacons.ShutdownServer.defaults()
			);
			shutdown
				.send()
				.catch((error: Error) => {
					panic(
						`Fail to send a shutdown signal to server: ${error.message}`
					);
				})
				.finally(() => {
					consumer
						.destroy()
						.catch((error: Error) => {
							panic(`Fail to destroy consumer: ${error.message}`);
						})
						.finally(() => {
							const finished = Date.now() - this._started;
							console.log(`=`.repeat(90));
							this._stat.print();
							console.log(`=`.repeat(90));
							console.log(
								`Done in ${finished / 1000}s (${finished}ms)`
							);
							console.log(`=`.repeat(90));
							const errors = this._stat.getErrors();
							errors.forEach((err: string) => {
								console.error(err);
							});
							if (errors.length > 0) {
								panic("Test results are negative");
							}
						});
				});
		});
	}
}

const app: Application = new Application();

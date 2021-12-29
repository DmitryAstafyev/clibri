import { Consumer, Protocol, StructA } from "./consumer/index";
import { Connection } from "clibri-websocket-client-node";
import { Test } from "./test";
class Application {
	private _connection!: Connection;
	private _consumer!: Consumer;

	constructor() {
		this._connection = new Connection(`ws://127.0.0.1:8080`);
		this._consumer = new Consumer(this._connection, {
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
		this._consumer.connected.subscribe(this._onConnected.bind(this));
		this._consumer.disconnected.subscribe(this._onDisconnected.bind(this));
		this._consumer.ready.subscribe(this._onReady.bind(this));
	}

	private _onConnected() {
		console.log(`>>>>>>>>>>>>>> connected`);
	}

	private _onDisconnected() {}

	private _onReady() {
		console.log(`>>>>>>>>>>>>>> ready`);
		const test = new Test();
		test.run();
	}
}

const app: Application = new Application();

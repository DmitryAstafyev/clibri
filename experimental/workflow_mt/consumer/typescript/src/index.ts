import { Consumer, Protocol, StructA } from "./consumer/index";
import { Connection } from "clibri-websocket-client";

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
		const structA = new StructA({
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
		structA.caseb((response: Protocol.StructD) => {
			console.log(`DONE`);
			this._consumer
				.destroy()
				.then(() => {
					console.log(`Consumer has been destroyed`);
				})
				.catch((err: Error) => {
					console.log(`Fail to destroy: ${err.message}`);
				});
		});
		structA.send().catch((err: Error) => {
			console.log(err);
		});
	}

	private _onDisconnected() {}

	private _onReady() {}

	// private _onLoginInput(username: string) {
	// 	this._components.login.umount();
	// 	const login: UserLoginRequest = new UserLoginRequest({
	// 		username: username,
	// 	});
	// 	login
	// 		.accept((response: Protocol.UserLogin.Accepted) => {
	// 			this._components.users.mount();
	// 			this._components.messages.setUuid(response.uuid);
	// 			this._components.messages.mount();
	// 			this._components.sender.setUsername(username);
	// 			this._components.sender.setMessagesRef(
	// 				this._components.messages
	// 			);
	// 			this._components.sender.setUuid(response.uuid);
	// 			this._components.sender.mount();
	// 			this._components.stat.mount();
	// 		})
	// 		.deny((response: Protocol.UserLogin.Denied) => {
	// 			// console.log(response);
	// 		})
	// 		.err((response: Protocol.UserLogin.Err) => {
	// 			// console.log(response);
	// 		});
	// 	login.send().catch((err: Error) => {
	// 		console.error(err);
	// 	});
	// }
}

const app: Application = new Application();

// tslint:disable: variable-name
import { Protocol, StructC } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_StructB = false;
		let case_StructF = false;
		let case_StructD = false;
		let case_StructE = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new StructC(
					Protocol.StructC.defaults(),
					consumer
				);
				request.send().then((response) => {
					if (response instanceof Protocol.StructB) {
						if (!case_StructB) {
							case_StructB = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructB`
								)
							);
						}
					} else if (response instanceof Protocol.StructF) {
						if (!case_StructF) {
							case_StructF = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructF`
								)
							);
						}
					} else if (response instanceof Protocol.StructD) {
						if (!case_StructD) {
							case_StructD = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructD`
								)
							);
						}
					} else if (response instanceof Protocol.StructE) {
						if (!case_StructE) {
							case_StructE = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructE`
								)
							);
						}
					}
				});
			});
		}
		function trigger() {
			send()
				.then(() => {
					if (
						case_StructB &&
						case_StructF &&
						case_StructD &&
						case_StructE
					) {
						resolve(undefined);
					} else {
						trigger();
					}
				})
				.catch(reject);
		}
		trigger();
	});
}

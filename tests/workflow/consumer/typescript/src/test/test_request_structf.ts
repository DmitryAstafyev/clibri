// tslint:disable: variable-name
import { Protocol, StructF } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_StructE = false;
		let case_StructF = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new StructF(
					Protocol.StructF.defaults(),
					consumer
				);
				request.send().then((response) => {
					if (response instanceof Protocol.StructF) {
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
					if (case_StructF && case_StructE) {
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

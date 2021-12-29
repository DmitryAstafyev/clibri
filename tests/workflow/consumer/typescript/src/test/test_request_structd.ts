// tslint:disable: variable-name
import { Protocol, StructD } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_StructA = false;
		let case_StructC = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new StructD(
					Protocol.StructD.defaults(),
					consumer
				);
				request.send().then((response) => {
					if (response instanceof Protocol.StructC) {
						if (!case_StructC) {
							case_StructC = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructC`
								)
							);
						}
					} else if (response instanceof Protocol.StructA) {
						if (!case_StructA) {
							case_StructA = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructA`
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
					if (case_StructA && case_StructC) {
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

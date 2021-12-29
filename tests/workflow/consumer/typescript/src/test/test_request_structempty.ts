// tslint:disable: variable-name
import { Protocol, StructEmpty } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_StructEmptyA = false;
		let case_StructEmptyB = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new StructEmpty(
					Protocol.StructEmpty.defaults(),
					consumer
				);
				request.send().then((response) => {
					if (response instanceof Protocol.StructEmptyB) {
						if (!case_StructEmptyB) {
							case_StructEmptyB = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructEmptyB`
								)
							);
						}
					} else if (response instanceof Protocol.StructEmptyA) {
						if (!case_StructEmptyA) {
							case_StructEmptyA = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructEmptyA`
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
					if (case_StructEmptyB && case_StructEmptyA) {
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

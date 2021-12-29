// tslint:disable: variable-name
import { Protocol, GroupAStructA } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_StructA = false;
		let case_StructB = false;
		let case_GroupAStructB = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new GroupAStructA(
					Protocol.GroupA.StructA.defaults(),
					consumer
				);
				request.send().then((response) => {
					if (response instanceof Protocol.StructA) {
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
					} else if (response instanceof Protocol.StructB) {
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
					} else if (response instanceof Protocol.GroupA.StructB) {
						if (!case_GroupAStructB) {
							case_GroupAStructB = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.GroupA.StructB`
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
					if (case_StructB && case_StructA && case_GroupAStructB) {
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

// tslint:disable: variable-name
import { Protocol, GroupBStructA } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_GroupBGroupCStructB = false;
		let case_GroupBStructA = false;
		let case_GroupBGroupCStructA = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new GroupBStructA(
					Protocol.GroupB.StructA.defaults(),
					consumer
				);
				request.send().then((response) => {
					if (response instanceof Protocol.GroupB.GroupC.StructB) {
						if (!case_GroupBGroupCStructB) {
							case_GroupBGroupCStructB = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.GroupB.GroupC.StructB`
								)
							);
						}
					} else if (response instanceof Protocol.GroupB.StructA) {
						if (!case_GroupBStructA) {
							case_GroupBStructA = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.GroupB.StructA`
								)
							);
						}
					} else if (
						response instanceof Protocol.GroupB.GroupC.StructA
					) {
						if (!case_GroupBGroupCStructA) {
							case_GroupBGroupCStructA = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.GroupB.GroupC.StructA`
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
						case_GroupBGroupCStructB &&
						case_GroupBStructA &&
						case_GroupBGroupCStructA
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

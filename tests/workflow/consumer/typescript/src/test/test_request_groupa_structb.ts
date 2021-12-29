// tslint:disable: variable-name
import { Protocol, GroupAStructB } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_GroupBStructA = false;
		let case_GroupBGroupCStructA = false;
		let case_GroupAStructB = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new GroupAStructB(
					Protocol.GroupA.StructB.defaults(),
					consumer
				);
				request.send().then((response) => {
					if (response instanceof Protocol.GroupA.StructB) {
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
						case_GroupAStructB &&
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

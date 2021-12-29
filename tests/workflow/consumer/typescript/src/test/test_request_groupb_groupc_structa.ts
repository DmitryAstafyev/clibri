// tslint:disable: variable-name
import { Protocol, GroupBGroupCStructA } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_GroupAStructB = false;
		let case_GroupBGroupCStructB = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new GroupBGroupCStructA(
					Protocol.GroupB.GroupC.StructA.defaults(),
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
					} else if (
						response instanceof Protocol.GroupB.GroupC.StructB
					) {
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
					}
				});
			});
		}
		function trigger() {
			send()
				.then(() => {
					if (case_GroupBGroupCStructB && case_GroupAStructB) {
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

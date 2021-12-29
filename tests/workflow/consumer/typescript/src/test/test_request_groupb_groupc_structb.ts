// tslint:disable: variable-name
import { Protocol, GroupBGroupCStructB } from "../consumer/index";
import { Consumer } from "../consumer";

export function test(consumer: Consumer): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_GroupBGroupCStructA = false;
		let case_StructB = false;
		let case_StructC = false;
		let case_StructD = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new GroupBGroupCStructB(
					Protocol.GroupB.GroupC.StructB.defaults(),
					consumer
				);
				request.send().then((response) => {
					if (response instanceof Protocol.GroupB.GroupC.StructA) {
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
					} else if (response instanceof Protocol.StructC) {
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
					}
				});
			});
		}
		function trigger() {
			send()
				.then(() => {
					if (
						case_GroupBGroupCStructA &&
						case_StructB &&
						case_StructC &&
						case_StructD
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

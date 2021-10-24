import * as Protocol from "./implementation/protocol";

export class Context {
	private _messages: Map<number, Protocol.Messages.Message> = new Map();
	private _users: Map<string, Protocol.Users.User> = new Map();
	private _sequence: number = 0;

	public addUser(uuid: string, username: string) {
		this._users.set(
			uuid,
			new Protocol.Users.User({
				uuid: uuid,
				name: username,
			})
		);
	}

	public removeUser(uuid: string): Protocol.Users.User | Error {
		const user = this._users.get(uuid);
		if (user === undefined) {
			return new Error(`User ${uuid} isn't found`);
		}
		this._users.delete(uuid);
		return user;
	}

	public getUsers(): Protocol.Users.User[] {
		return Array.from(this._users.values());
	}

	public addMessage(
		msg: Protocol.Messages.Message
	): Protocol.Messages.Message {
		this._messages.set(this._sequence++, msg);
		return msg;
	}

	public getMessages(): Protocol.Messages.Message[] {
		return Array.from(this._messages.values());
	}
}

import { Component } from "../component";
import {
	Consumer,
	Protocol,
	UsersRequest,
	BeaconBeaconsLikeUser,
} from "../../consumer/index";
import { Subscription } from "clibri";
import { StatComponent } from "../stat/component";

interface IUser {
	name: string;
	uuid: string;
}

const USER_ELEMENT_SELECTOR = `li[data-uuid]`;

export class UsersComponent extends Component {
	private _instance: HTMLElement | undefined;
	private _users: IUser[] = [];
	private _consumer: Consumer;
	private _subscriptions: { [key: string]: Subscription } = {};
	private _stat: StatComponent;
	private _refs: Array<() => void> = [];

	constructor(consumer: Consumer, stat: StatComponent) {
		super();
		this._consumer = consumer;
		this._stat = stat;
	}

	public mount(): Error | undefined {
		if (this._instance !== undefined) {
			return new Error(`Already mount`);
		}
		this._subscriptions.UserConnected =
			this._consumer.broadcast.EventsUserConnected.subscribe(
				this._onUserConnected.bind(this)
			);
		this._subscriptions.UserDisconnected =
			this._consumer.broadcast.EventsUserDisconnected.subscribe(
				this._onUserDisconnected.bind(this)
			);
		this.link(`./components/users/style.css`);
		this._instance = this.element();
		const holder: HTMLElement | null =
			document.body.querySelector('aside[id="users"]');
		if (holder === null) {
			return new Error(`Fail find holder DOM element`);
		}
		holder.appendChild(this._instance);
		this._bind();
		this._request();
	}

	public umount(): Error | undefined {
		if (
			this._instance === undefined ||
			this._instance.parentNode === null ||
			this._instance.parentNode === undefined
		) {
			return new Error(`Already umount`);
		}
		Object.keys(this._subscriptions).forEach((key: string) => {
			this._subscriptions[key].destroy();
		});
		this._instance.parentNode.removeChild(this._instance);
		this._instance = undefined;
	}

	public element(): HTMLElement {
		if (this._users.length === 0) {
			const holder: HTMLElement = document.createElement("p");
			holder.className = "t-normal";
			holder.innerHTML = "No users are online";
			return holder;
		} else {
			const holder: HTMLElement = document.createElement("ul");
			holder.className = "users";
			holder.innerHTML = this._users
				.map((user: IUser) => {
					return `<li data-uuid="${user.uuid}">${user.name}</li>`;
				})
				.join("");
			return holder;
		}
	}

	public destroy() {
		if (this._instance === undefined) {
			return;
		}
		this.umount();
		this.unlink();
		this._unbind();
		this._instance = undefined;
	}

	public update(users: IUser[]) {
		if (this._instance === undefined) {
			return;
		}
		this._users = users;
		this._stat.setUsers(this._users.length);
		if (this._instance.nodeName.toLowerCase() === "p") {
			const parent = this._instance.parentNode;
			if (parent !== null) {
				parent.removeChild(this._instance);
				this._instance = this.element();
				parent.appendChild(this._instance);
			}
		} else {
			this._instance.innerHTML = this.element().innerHTML;
		}
		this._bind();
	}

	private _bind() {
		this._unbind();
		document
			.querySelectorAll<HTMLElement>(USER_ELEMENT_SELECTOR)
			.forEach((user: HTMLElement) => {
				const uuid = user.getAttribute("data-uuid");
				if (uuid === null) {
					return;
				}
				const handler = this._onUserClick.bind(this, uuid);
				this._refs.push(
					// tslint:disable-next-line: only-arrow-functions
					function (
						cb: (uuid: string) => void,
						node: HTMLElement
					): void {
						node.removeEventListener("click", cb as any);
					}.bind(this, handler, user)
				);
				user.addEventListener("click", handler);
			});
	}

	private _unbind() {
		this._refs.forEach((ref: () => void) => {
			ref();
		});
		this._refs = [];
	}

	private _onUserClick(uuid: string) {
		new BeaconBeaconsLikeUser(new Protocol.Beacons.LikeUser({ uuid }))
			.send()
			.then(() => {
				console.log(`Beacon LikeUser has been deliveried`);
			})
			.catch((err: Error) => {
				console.log(err);
			});
	}

	private _onUserConnected(event: Protocol.Events.UserConnected) {
		this._request();
	}

	private _onUserDisconnected(event: Protocol.Events.UserDisconnected) {
		this._request();
	}

	private _request() {
		new UsersRequest(new Protocol.Users.Request({}))
			.send()
			.then((response: Protocol.Users.Response | Protocol.Users.Err) => {
				if (response instanceof Protocol.Users.Err) {
					return console.log(`Error: ${response.error}`);
				}
				this.update(response.users);
			})
			.catch((err: Error) => {
				console.log(err);
			});
	}
}

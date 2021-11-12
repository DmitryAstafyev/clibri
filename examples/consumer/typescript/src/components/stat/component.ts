import { Component } from "../component";

interface IStat {
	messages: number;
	users: number;
}

export class StatComponent extends Component {
	private _instance: HTMLElement | undefined;
	private _stat: IStat = {
		messages: 0,
		users: 0,
	};

	constructor() {
		super();
	}

	public mount(): Error | undefined {
		if (this._instance !== undefined) {
			return new Error(`Already mount`);
		}
		this.link(`./components/stat/style.css`);
		this._instance = this.element();
		const holder: HTMLElement | null =
			document.body.querySelector("footer");
		if (holder === null) {
			return new Error(`Fail find holder DOM element`);
		}
		holder.appendChild(this._instance);
	}

	public umount(): Error | undefined {
		if (
			this._instance === undefined ||
			this._instance.parentNode === null ||
			this._instance.parentNode === undefined
		) {
			return new Error(`Already umount`);
		}
		this._instance.parentNode.removeChild(this._instance);
		this._instance = undefined;
	}

	public element(): HTMLElement {
		const holder: HTMLElement = document.createElement("p");
		holder.className = "stat";
		holder.innerHTML = `<span>users online: ${this._stat.users}</span><span>messages: ${this._stat.messages}</span>`;
		return holder;
	}

	public destroy() {
		if (this._instance === undefined) {
			return;
		}
		this.umount();
		this.unlink();
		this._instance = undefined;
	}

	public setUsers(count: number) {
		this._stat.users = count;
		this._update();
	}

	public setMessages(count: number) {
		this._stat.messages = count;
		this._update();
	}

	private _update() {
		this.umount();
		this.mount();
	}
}

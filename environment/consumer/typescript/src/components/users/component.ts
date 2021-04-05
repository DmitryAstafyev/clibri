import { Component } from '../component';
import { Consumer, Protocol, Users } from '../../consumer/index';
import { Subscription } from 'fiber';

interface IUser {
    name: string;
    uuid: string;
}

export class UsersComponent extends Component {

    private _instance: HTMLElement | undefined;
    private _users: IUser[] = [];
    private _consumer: Consumer;
    private _subscriptions: { [key: string]: Subscription } = {};

    constructor(consumer: Consumer) {
        super();
        this._consumer = consumer;
    }

    public mount(): Error | undefined {
        if (this._instance !== undefined) {
            return new Error(`Already mount`);
        }
        this._subscriptions.UserConnected = this._consumer.broadcast.UserConnected.subscribe(this._onUserConnected.bind(this));
        this._subscriptions.UserDisconnected = this._consumer.broadcast.UserDisconnected.subscribe(this._onUserDisconnected.bind(this));
        this.link(`./components/users/style.css`);
        this._instance = this.element();
        const holder: HTMLElement | null = document.body.querySelector('aside[id="users"]');
        if (holder === null) {
            return new Error(`Fail find holder DOM element`);
        }
        holder.appendChild(this._instance);
        this._request();
    }

    public umount(): Error | undefined {
        if (this._instance.parentNode === null || this._instance.parentNode === undefined) {
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
            const holder: HTMLElement = document.createElement('p');
            holder.className = 't-normal';
            holder.innerHTML = 'No users are online';
            return holder;
        } else {
            const holder: HTMLElement = document.createElement('ul');
            holder.className = 'users';
            holder.innerHTML = this._users.map((user: IUser) => {
                return `<li data-uuid="${user.uuid}">${user.name}</li>`;
            }).join('');
            return holder;
        }
    }

    public destroy() {
        if (this._instance === undefined) {
            return;
        }
        this.umount();
        this.unlink();
        this._instance = undefined;
    }

    public update(users: IUser[]) {
        if (this._instance === undefined) {
            return;
        }
        this._users = users;
        if (this._instance.nodeName.toLowerCase() === 'p') {
            const parent = this._instance.parentNode;
            parent.removeChild(this._instance);
            this._instance = this.element();
            parent.appendChild(this._instance);
        } else {
            this._instance.innerHTML = this.element().innerHTML;
        }
    }

    private _onUserConnected(event: Protocol.Events.UserConnected) {
        this._request();
    }

    private _onUserDisconnected(event: Protocol.Events.UserDisconnected) {
        this._request();
    }

    private _request() {
        (new Users(new Protocol.Users.Request({}))).send().then((response: Protocol.Users.Response | Protocol.Users.Err) => {
            if (response instanceof Protocol.Users.Err) {
                return console.log(`Error: ${response.error}`);
            }
            this.update(response.users)
        }).catch((err: Error) => {
            console.log(err);
        });
    }

}
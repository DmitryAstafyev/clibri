import { Component } from '../component';
import { Consumer, Protocol, MessagesRequest } from '../../consumer/index';
import { Subscription } from 'fiber';
import { StatComponent } from '../stat/component';

interface IMessage {
    datetime: string;
    message: string;
    user: string;
    uuid: string;
}

export class MessagesComponent extends Component {

    private _instance: HTMLElement | undefined;
    private _messages: IMessage[] = [];
    private _uuid: string | undefined;
    private _consumer: Consumer;
    private _subscriptions: { [key: string]: Subscription } = {};
    private _stat: StatComponent;

    constructor(consumer: Consumer, stat: StatComponent) {
        super();
        this._consumer = consumer;
        this._stat = stat;
    }

    public mount(): Error | undefined {
        if (this._instance !== undefined) {
            return new Error(`Already mount`);
        }
        this._subscriptions.Message = this._consumer.broadcast.EventsMessage.subscribe(this._onMessage.bind(this));
        this.link(`./components/messages/style.css`);
        this._instance = this.element();
        const holder: HTMLElement | null = document.body.querySelector('div[id="messages"]');
        if (holder === null) {
            return new Error(`Fail find holder DOM element`);
        }
        holder.appendChild(this._instance);
        this._request();
    }

    public umount(): Error | undefined {
        if (this._instance === undefined || this._instance.parentNode === null || this._instance.parentNode === undefined) {
            return new Error(`Already umount`);
        }
        Object.keys(this._subscriptions).forEach((key: string) => {
            this._subscriptions[key].destroy();
        });
        this._instance.parentNode.removeChild(this._instance);
        this._instance = undefined;
    }

    public element(): HTMLElement {
        if (this._messages.length === 0) {
            const element: HTMLElement = document.createElement('p');
            element.className = 't-normal messages-info';
            element.innerHTML = 'No messages';
            return element;
        } else {
            const element: HTMLElement = document.createElement('ul');
            element.className = 'messages';
            element.innerHTML = this._messages.map((msg: IMessage) => {
                return `<li class="${msg.uuid === this._uuid ? 'own' : ''}${msg.user === '' ? ' system' : ''}">
                    <span class="username">${msg.user}</span>
                    <p class="message">${msg.message.replace(/\n/gi, '</br>')}</p>
                    <span class="datetime">${msg.datetime}</span>
                </li>`;
            }).join('');
            return element;
        }
    }

    public update(messages: IMessage[]) {
        if (this._instance === undefined) {
            return;
        }
        this._messages = messages;
        this._stat.setMessages(this._messages.length);
        if (this._instance.nodeName.toLowerCase() === 'p') {
            const parent = this._instance.parentNode;
            if (parent !== null) {
                parent.removeChild(this._instance);
                this._instance = this.element();
                parent.appendChild(this._instance);    
            }
        } else {
            this._instance.innerHTML = this.element().innerHTML;
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

    public setUuid(uuid: string) {
        this._uuid = uuid;
    }

    public addOwnMessage(message: IMessage) {
        this._messages.push(message);
        this.update(this._messages);    }

    private _request() {
        (new MessagesRequest(new Protocol.Messages.Request({}))).send().then((response: Protocol.Messages.Response | Protocol.Messages.Err) => {
            if (response instanceof Protocol.Messages.Err) {
                return console.log(`Error: ${response.error}`);
            }
            this.update(response.messages.map((msg: Protocol.Messages.Message) => {
                return {
                    user: msg.user,
                    uuid: msg.uuid,
                    message: msg.message,
                    datetime: (new Date(Number(msg.timestamp) * 1000)).toLocaleString()
                }
            }));
        }).catch((err: Error) => {
            console.log(err);
        });
    }

    private _onMessage(event: Protocol.Events.Message) {
        const date = new Date(Number(event.timestamp));
        this._messages.push({
            message: event.message,
            user: event.user,
            uuid: event.uuid,
            datetime: date.toLocaleDateString(),
        });
        this.update(this._messages);
    }


}
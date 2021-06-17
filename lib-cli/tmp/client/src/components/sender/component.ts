import { Component } from '../component';
import { Protocol, MessageRequest } from '../../consumer/index';
import { MessagesComponent } from '../messages/component';

export class SenderComponent extends Component {

    private _instance: HTMLTextAreaElement | undefined;
    private _username: string | undefined;
    private _messages: MessagesComponent | undefined;
    private _uuid: string | undefined;

    constructor() {
        super();
        this._onKeyUp = this._onKeyUp.bind(this);
    }

    public mount(): Error | undefined {
        if (this._instance !== undefined) {
            return new Error(`Already mount`);
        }
        this.link(`./components/sender/style.css`);
        this._instance = this.element();
        const holder: HTMLElement | null = document.body.querySelector('div[id="input"]');
        if (holder === null) {
            return new Error(`Fail find holder DOM element`);
        }
        holder.appendChild(this._instance);
        this._instance.focus();
        this._events().bind();
    }

    public umount(): Error | undefined {
        if (this._instance === undefined || this._instance.parentNode === null || this._instance.parentNode === undefined) {
            return new Error(`Already umount`);
        }
        this._events().unbind();
        this._instance.parentNode.removeChild(this._instance);
        this._instance = undefined;
    }

    public element(): HTMLTextAreaElement {
        const element: HTMLTextAreaElement = document.createElement('textarea');
        element.id = 'message-input';
        element.className = 'border-a';
        return element;
    }

    public destroy() {
        if (this._instance === undefined) {
            return;
        }
        this.umount();
        this.unlink();
        this._instance = undefined;
    }

    public setUsername(username: string) {
        this._username = username;
    }

    public setMessagesRef(messages: MessagesComponent) {
        this._messages = messages;
    }

    public setUuid(uuid: string) {
        this._uuid = uuid;
    }

    private _events(): {
        bind: () => void,
        unbind: () => void,
    } {
        const self = this;
        return {
            bind() {
                self._instance !== undefined && self._instance.addEventListener('keyup', self._onKeyUp);
            },
            unbind() {
                self._instance !== undefined && self._instance.removeEventListener('keyup', self._onKeyUp);
            },
        };
    }

    private _onKeyUp(event: KeyboardEvent) {
        if (event.key === 'Enter' && event.ctrlKey) {
            const value: string = this._instance.value.replace(/</gi, '(').replace(/>/gi, ')');
            if (value.trim() !== '') {
                this._instance.disabled = true;
                this._send(value).then(() => {
                    this._instance.value = '';
                    this._instance.focus();
                }).catch((err: Error) => {
                    console.log(err);
                }).finally(() => {
                    this._instance.disabled = false;
                });
            }
        }
    }

    private _send(message: string): Promise<void> {
        return new Promise((resolve, reject) => {
            (new MessageRequest(new Protocol.Message.Request({
                user: this._username,
                message: message,
            }))).send().then((response: Protocol.Message.Accepted | Protocol.Message.Denied | Protocol.Message.Err) => {
                if (response instanceof Protocol.Message.Err) {
                    return reject(new Error(response.error));
                }
                if (response instanceof Protocol.Message.Denied) {
                    return reject(new Error(`Message cannot be posted because: ${response.reason}`));
                }
                this._messages.addOwnMessage({
                    uuid: this._uuid,
                    user: this._username,
                    message: message,
                    datetime: (new Date()).toLocaleString(),
                });
                resolve();
            }).catch(reject);
        });
    }

}
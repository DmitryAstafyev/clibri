import { Component } from '../component';

export class LoginComponent extends Component {

    private _instance: HTMLElement | undefined;

    constructor() {
        super();
        this._onKeyUp = this._onKeyUp.bind(this);
    }

    public mount(): Error | undefined {
        if (this._instance !== undefined) {
            return new Error(`Already mount`);
        }
        this.link(`./components/login/style.css`);
        this._instance = this.element();
        document.body.appendChild(this._instance);
        this._events().bind();
    }

    public umount(): Error | undefined {
        if (this._instance.parentNode === null || this._instance.parentNode === undefined) {
            return new Error(`Already umount`);
        }
        this._events().unbind();
        this._instance.parentNode.removeChild(this._instance);
    }

    public element(): HTMLElement {
        const holder: HTMLElement = document.createElement('div');
        holder.className = 'holder';
        holder.innerHTML = `
            <div id="login" class="modal background-a border-a">
                <div>
                    <label>User name</label>
                    <input type="text"/>
                </div>
            </div>`;
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

    private _events(): {
        bind: () => void,
        unbind: () => void,
    } {
        const getter = () => {
            if (self._instance === undefined) {
                return;
            }
            const input: HTMLInputElement | null | undefined = self._instance.querySelector('input');
            if (input === null || input === undefined) {
                return;
            }
            return input;
        };
        const self = this;
        return {
            bind: () => {
                const input = getter();
                input !== undefined && input.addEventListener('keyup', self._onKeyUp);
            },
            unbind: () => {
                const input = getter();
                input !== undefined && input.removeEventListener('keyup', self._onKeyUp);
            },
        };
    }

    private _onKeyUp(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            console.log(`Input: ${(event.target as HTMLInputElement).value}`);
        }
    }

}
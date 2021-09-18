import { Component } from '../component';

export class SpinnerComponent extends Component {

    private _instance: HTMLElement | undefined;

    public mount(): Error | undefined {
        if (this._instance !== undefined) {
            return new Error(`Already mount`);
        }
        this.link(`./components/spinner/style.css`);
        this._instance = this.element();
        document.body.appendChild(this._instance);
    }

    public umount(): Error | undefined {
        if (this._instance === undefined || this._instance.parentNode === null || this._instance.parentNode === undefined) {
            return new Error(`Already umount`);
        }
        this._instance.parentNode.removeChild(this._instance);
        this._instance = undefined;
    }

    public element(): HTMLElement {
        const holder: HTMLElement = document.createElement('div');
        holder.className = 'holder';
        holder.innerHTML = `
            <div id="pending" class="modal spinner-holder">
                <div class="spinner">
                    <div class="lds-ripple"><div></div><div></div></div>
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

}
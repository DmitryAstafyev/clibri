export abstract class Component {

    private _link: HTMLLinkElement | undefined;

    public abstract element(): HTMLElement;
    public abstract mount(): Error | undefined;
    public abstract umount(): Error | undefined;
    public abstract destroy(): void;

    public link(path: string) {
        if (this._link !== undefined) {
            return;
        }
        this._link = document.createElement('link');
        this._link.href = path;
        this._link.type = 'text/css';
        this._link.rel = 'stylesheet';
        this._link.media = 'screen,print';
        document.getElementsByTagName('head')[0].appendChild(this._link);
    }

    public unlink() {
        if (this._link === undefined) {
            return;
        }
        this._link.parentNode.removeChild(this._link);
        this._link = undefined;
    }

}